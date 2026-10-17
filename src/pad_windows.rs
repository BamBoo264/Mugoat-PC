//! XInput controller support using the same game key mapping as the Apple host.
use crate::input;
use std::{collections::HashSet,sync::{Mutex,atomic::{AtomicBool,Ordering}},time::{Instant,Duration}};
pub static ACTIVE:AtomicBool=AtomicBool::new(false);
pub static IN_VEHICLE:AtomicBool=AtomicBool::new(false);
pub static HORN_PAD:AtomicBool=AtomicBool::new(false);
pub static HORN_TOUCH:AtomicBool=AtomicBool::new(false);
pub fn horn_held()->bool {HORN_PAD.load(Ordering::Relaxed)||HORN_TOUCH.load(Ordering::Relaxed)}
#[repr(C)] #[derive(Default)]
struct Pad {buttons:u16,lt:u8,rt:u8,lx:i16,ly:i16,rx:i16,ry:i16}
#[repr(C)] #[derive(Default)]
struct XState {packet:u32,pad:Pad}
#[link(name="xinput")]
extern "system" {fn XInputGetState(index:u32,state:*mut XState)->u32;}
struct State {held:HashSet<&'static str>,buttons:u8,last:Instant,down_since:Option<Instant>}
static STATE:Mutex<Option<State>>=Mutex::new(None);
fn release(s:State) {for k in s.held {input::key(k,false,false,None,false);}for (b,n) in [(1,0),(2,2)] {if s.buttons&b!=0 {input::button(n,false);}}HORN_PAD.store(false,Ordering::Relaxed);}
pub fn poll()->bool {
    let mut raw=XState::default();
    if !(0..4).any(|i|unsafe {XInputGetState(i,&mut raw)==0}) {
        ACTIVE.store(false,Ordering::Relaxed);
        if let Some(s)=STATE.lock().unwrap().take(){release(s);}return false;
    }
    ACTIVE.store(true,Ordering::Relaxed);
    let mut lock=STATE.lock().unwrap();
    let s=lock.get_or_insert_with(||State{held:HashSet::new(),buttons:0,last:Instant::now(),down_since:None});
    let dt=s.last.elapsed().as_secs_f64().min(0.05);s.last=Instant::now();
    let p=raw.pad;let down=|m|p.buttons&m!=0;let in_car=IN_VEHICLE.load(Ordering::Relaxed);
    let mut keys=HashSet::new();let mut buttons=0;
    if p.lx < -9830 {keys.insert("KeyA");}if p.lx>9830 {keys.insert("KeyD");}
    if in_car {
        if p.ly>9830 {keys.insert("Numpad8");}if p.ly < -9830 {keys.insert("Numpad2");}
        if p.rt>51 {keys.insert("KeyW");}if p.lt>51 {keys.insert("KeyS");}
        if down(0x0200)||down(0x4000) {keys.insert("Space");}
        HORN_PAD.store(down(0x0040),Ordering::Relaxed);
    }else{
        HORN_PAD.store(false,Ordering::Relaxed);
        if p.ly>9830 {keys.insert("KeyW");}if p.ly < -9830 {keys.insert("KeyS");}
        if p.rt>51 {buttons|=1;}if p.lt>51 {buttons|=2;}
        for (mask,key) in [(0x1000,"ShiftLeft"),(0x4000,"Space"),(0x0200,"KeyQ"),(0x0040,"ControlLeft")] {if down(mask){keys.insert(key);}}
    }
    for (mask,key) in [(0x1000,"Enter"),(0x2000,"KeyR"),(0x2000,"Backspace"),(0x8000,"KeyF"),(0x0100,"Tab"),(0x0080,"KeyC"),(0x0001,"ArrowUp"),(0x0004,"ArrowLeft"),(0x0008,"ArrowRight"),(0x0010,"Escape"),(0x0020,"KeyV")] {if down(mask){keys.insert(key);}}
    if down(0x0002){let t=*s.down_since.get_or_insert_with(Instant::now);keys.insert(if t.elapsed()>=Duration::from_millis(300){"AltLeft"}else{"ArrowDown"});}else{s.down_since=None;}
    let curve=|n:i16|{let v=(n as f64/32767.0).clamp(-1.0,1.0);if v.abs()<0.12 {0.0}else{((v.abs()-0.12)/0.88).powf(1.8).copysign(v)}};
    let aim=std::env::var("MUGUET_AIM_SENS").ok().and_then(|v|v.parse().ok()).unwrap_or(0.4);
    let (dx,dy)=crate::aim::scale_delta((curve(p.rx)*700.0*dt,-curve(p.ry)*700.0*dt),1.0,buttons&2!=0&&!in_car,aim);
    if dx!=0.0||dy!=0.0 {input::mouse_delta_f(dx,dy);}
    for k in s.held.difference(&keys){input::key(k,false,false,None,false);}
    for k in keys.difference(&s.held){crate::mp::key_pressed(k);input::key(k,true,false,None,false);}
    for (bit,dom) in [(1,0),(2,2)]{if (buttons^s.buttons)&bit!=0 {input::button(dom,buttons&bit!=0);}}
    s.held=keys;s.buttons=buttons;true
}
