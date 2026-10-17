"""Exercise the unmodified relay locally. This is NOT an iOS/gameplay test."""
import socket, subprocess, sys, time
from pathlib import Path

exe = Path(sys.argv[1]) if len(sys.argv)>1 else Path(__file__).resolve().parents[1]/'target/release/muguet-server.exe'
with socket.socket(socket.AF_INET,socket.SOCK_DGRAM) as probe:
    probe.bind(('127.0.0.1',0)); port=probe.getsockname()[1]
address=('127.0.0.1',port)
proc=subprocess.Popen([str(exe),str(port),'8'],stdout=subprocess.DEVNULL,stderr=subprocess.PIPE,creationflags=getattr(subprocess,'CREATE_NO_WINDOW',0))
sockets=[]
def client():
    s=socket.socket(socket.AF_INET,socket.SOCK_DGRAM);s.bind(('127.0.0.1',0));s.settimeout(0.2);sockets.append(s);return s
def send(s,kind,payload=b''):s.sendto(bytes([0x47,kind])+payload,address)
def receive(s,kind,timeout=2):
    deadline=time.monotonic()+timeout
    while time.monotonic()<deadline:
        try:data,_=s.recvfrom(2048)
        except (socket.timeout, ConnectionResetError):continue
        if data[:2]==bytes([0x47,kind]):return data[2:]
    raise AssertionError('No packet kind '+str(kind))
def join(s,room,name):
    for _ in range(15):
        if proc.poll() is not None:raise RuntimeError('Relay exited: '+proc.stderr.read().decode(errors='replace'))
        send(s,1,room+b'\0'+name)
        try:return receive(s,2,0.2)[0]
        except AssertionError:pass
    raise AssertionError('Handshake failed')
try:
    a,b,c=client(),client(),client()
    aid=join(a,b'pc-ios-check#secret',b'PC');bid=join(b,b'pc-ios-check#secret',b'iOS-protocol-simulator');join(c,b'pc-ios-check#other',b'Other-room')
    assert aid!=bid
    assert receive(a,4)[0]==bid
    assert receive(b,4)[0]==aid
    payload=bytes(range(128))
    send(a,3,payload);assert receive(b,3)==bytes([aid])+payload
    try:receive(c,3,0.4);raise AssertionError('Room isolation failed')
    except AssertionError as e:
        if str(e)!='No packet kind 3':raise
    send(b,8,b'appearance');assert receive(a,8)==bytes([bid])+b'appearance'
    send(b,6);assert receive(a,5)==bytes([bid])
    print('PASS: handshake, distinct player IDs, peer names, state relay, room/password isolation, appearance relay, leave notification.')
    print('No public server contacted. No actual iOS or full-game interoperability tested.')
finally:
    for s in sockets:s.close()
    if proc.poll() is None:proc.terminate()
    proc.wait(timeout=5);proc.stderr.close()
