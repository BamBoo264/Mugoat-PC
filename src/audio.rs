// muguet - audio.rs
// Constantin Clerc (c22dev) - 0.1

use crate::RT;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU64, Ordering};

static UNDERRUN_FRAMES: AtomicU64 = AtomicU64::new(0);
static CALLBACKS: AtomicU64 = AtomicU64::new(0);

static STARTED: AtomicBool = AtomicBool::new(false);

pub fn start(ring: u64, capacity: u32) {
    if STARTED.swap(true, Ordering::SeqCst) || std::env::var("MUGUET_MUTE").is_ok() {
        return;
    }

    eprintln!("[audio-sync] v2 diagnostics enabled: waits >100 ms, at most 2 lines/s; synchronization unchanged");
    std::thread::Builder::new()
        .name("audio".into())
        .spawn(move || match play(ring, capacity) {
            Ok(stream) => {
                eprintln!("[audio] Audio output started (ring 0x{ring:x}, {capacity} frames)");
                let _keep = stream;
                loop { std::thread::park(); }
            }
            Err(e) => eprintln!("[audio] no output: {e:#}"),
        })
        .expect("spawn audio thread");
}

fn play(ring: u64, capacity: u32) -> anyhow::Result<cpal::Stream> {
    let host = cpal::default_host();
    let device = host.default_output_device().ok_or_else(|| anyhow::anyhow!("no default output device"))?;
    let config = cpal::StreamConfig { channels: 2, sample_rate: 48000, buffer_size: cpal::BufferSize::Default };
    let (base, _) = crate::mem_base_len(&RT.get().unwrap().mem);
    let base = base as usize;
    let hdr = move |i: usize| -> &'static AtomicI32 { unsafe { &*((base + ring as usize + 4 * i) as *const AtomicI32) } };
    let data = base + ring as usize + 64;
    let mask = capacity.wrapping_sub(1);
    let stream = device.build_output_stream(
        config,
        move |out: &mut [f32], _| {
            let frames = out.len() / 2;
            let read = hdr(1).load(Ordering::SeqCst) as u32;
            let write = hdr(0).load(Ordering::SeqCst) as u32;
            let take = (write.wrapping_sub(read) as usize).min(frames);
            for i in 0..take {
                let at = ((read.wrapping_add(i as u32) & mask) * 2) as usize;
                unsafe {
                    let p = (data as *const f32).add(at);
                    out[2 * i] = *p;
                    out[2 * i + 1] = *p.add(1);
                }
            }
            for s in &mut out[take * 2..] { *s = 0.0; }
            UNDERRUN_FRAMES.fetch_add((frames - take) as u64, Ordering::Relaxed);
            CALLBACKS.fetch_add(1, Ordering::Relaxed);
            hdr(1).store(read.wrapping_add(take as u32) as i32, Ordering::SeqCst);
            hdr(3).fetch_add(1, Ordering::SeqCst);
        },
        |e| eprintln!("[audio] stream error: {e}"),
        None,
    )?;
    stream.play()?;

    std::thread::spawn(move || {
        let mut last_w = hdr(0).load(Ordering::SeqCst) as u32;
        let mut last_u = 0;
        let mut last_c = 0;
        loop {
            std::thread::sleep(std::time::Duration::from_secs(5));
            let w = hdr(0).load(Ordering::SeqCst) as u32;
            let (u, c) = (UNDERRUN_FRAMES.load(Ordering::Relaxed), CALLBACKS.load(Ordering::Relaxed));
            eprintln!("[audio] mixer produced {:.0} frames/s (needs 48000), underrun {:.0} frames/s, {} callbacks/s, ring fill {}",
                w.wrapping_sub(last_w) as f64 / 5.0, (u - last_u) as f64 / 5.0, (c - last_c) / 5, w.wrapping_sub(hdr(1).load(Ordering::SeqCst) as u32));
            last_w = w; last_u = u; last_c = c;
        }
    });
    Ok(stream)
}

// Bounded audio synchronization diagnostics for the pinned v0.1 engine.
// No scheduling changes; the guest wait/signal functions execute unchanged.
static DIAG_START: std::sync::LazyLock<std::time::Instant> = std::sync::LazyLock::new(std::time::Instant::now);
static UPDATE_BEGIN_US: AtomicU64 = AtomicU64::new(0);
static UPDATE_SIGNAL_US: AtomicU64 = AtomicU64::new(0);
static LAST_REPORT_US: AtomicU64 = AtomicU64::new(0);
thread_local! { static WAIT_START: std::cell::Cell<(u64,u64)> = const { std::cell::Cell::new((0,0)) }; }
fn stamp() -> u64 { DIAG_START.elapsed().as_micros() as u64 }
fn audio_semas(mem: &wasmtime::SharedMemory) -> (u64,u64) {
    (crate::read_u64(mem,15715080).unwrap_or(0),crate::read_u64(mem,15715088).unwrap_or(0))
}
pub fn sync_wait(mem: &wasmtime::SharedMemory, sem: u64, end: bool) {
    let thread=std::thread::current();
    let name=thread.name().unwrap_or("unnamed");
    let (start,finish)=audio_semas(mem);
    if sem!=start && sem!=finish && !name.contains("Audio") {return;}
    let now=stamp();
    if !end {WAIT_START.with(|v|v.set((sem,now)));return;}
    if sem==start {UPDATE_BEGIN_US.store(now,Ordering::Relaxed);}
    let (old,began)=WAIT_START.with(|v|v.replace((0,0)));
    if sem==start || old!=sem || now.saturating_sub(began)<=100_000 {return;}
    let last=LAST_REPORT_US.load(Ordering::Relaxed);
    if now.saturating_sub(last)<500_000 || LAST_REPORT_US.compare_exchange(last,now,Ordering::Relaxed,Ordering::Relaxed).is_err(){return;}
    eprintln!("[audio-sync] thread={name} id={:?} sem=0x{sem:x} role={} wait_start_us={began} wait_end_us={now} wait_ms={:.1} engine_begin_us={} engine_signal_us={}",thread.id(),if sem==finish{"main-finish"}else if sem==start{"engine-trigger"}else{"audio-worker"},(now-began) as f64/1000.0,UPDATE_BEGIN_US.load(Ordering::Relaxed),UPDATE_SIGNAL_US.load(Ordering::Relaxed));
}
pub fn sync_signal(mem: &wasmtime::SharedMemory, sem: u64) {
    let (_,finish)=audio_semas(mem);
    if sem==finish && finish!=0 {UPDATE_SIGNAL_US.store(stamp(),Ordering::Relaxed);}
}

// Read the engine's existing thread profiler on a freeze; no guest hooks needed
// for inlined wait paths. The pinned layout is derived from SemaTakeUntil.
pub fn snapshot(mem: &wasmtime::SharedMemory) {
    static LAST: AtomicU64 = AtomicU64::new(0);
    let now=stamp();let last=LAST.load(Ordering::Relaxed);
    if now.saturating_sub(last)<1_000_000 || LAST.compare_exchange(last,now,Ordering::Relaxed,Ordering::Relaxed).is_err(){return;}
    let (base,len)=crate::mem_base_len(mem);
    let load=|addr:usize|->u64{if addr%8!=0 || addr+8>len{return 0;}unsafe{(&*(base.add(addr) as *const AtomicU64)).load(Ordering::Relaxed)}};
    let string=|addr:u64|->String{if addr==0 || addr as usize>=len{return String::new();}let mut v=Vec::new();for i in 0..96usize{if addr as usize+i>=len{break;}let b=unsafe{std::ptr::read_volatile(base.add(addr as usize+i))};if b==0{break;}v.push(b);}String::from_utf8_lossy(&v).into_owned()};
    let mut rows=Vec::new();
    for i in 0..256usize {
        let p=8092384+i*112;
        let name=string(load(p));if !name.contains("Audio") && !name.contains("Main"){continue;}
        let kind=string(load(p+8));let address=load(p+16);let since=f64::from_bits(load(p+24));
        rows.push(format!("slot={i} name={name:?} state={kind:?} object=0x{address:x} since_ms={since:.3}"));
    }
    let (start,finish)=audio_semas(mem);
    eprintln!("[audio-snapshot] t_us={now} trigger=0x{start:x} finish=0x{finish:x} {}",rows.join(" | "));
}

thread_local! { static COMMAND_BEGIN: std::cell::Cell<u64> = const { std::cell::Cell::new(0) }; }
static COMMAND_LAST: AtomicU64 = AtomicU64::new(0);
pub fn command_span(device: u64, end: bool) {
    let now=stamp();
    if !end {COMMAND_BEGIN.with(|v|v.set(now));return;}
    let began=COMMAND_BEGIN.with(|v|v.replace(0));
    if std::thread::current().name().unwrap_or("").contains("North Audio") { COMMAND_COUNT.fetch_add(1,Ordering::Relaxed); COMMAND_US.store(now.saturating_sub(began),Ordering::Relaxed); }
    if now.saturating_sub(began)<=100_000{return;}
    let last=COMMAND_LAST.load(Ordering::Relaxed);
    if now.saturating_sub(last)<500_000 || COMMAND_LAST.compare_exchange(last,now,Ordering::Relaxed,Ordering::Relaxed).is_err(){return;}
    let th=std::thread::current();
    eprintln!("[audio-command] thread={} id={:?} stage=WaitOnThreadCommandBufferProcessing device=0x{device:x} start_us={began} end_us={now} duration_ms={:.1}",th.name().unwrap_or("unnamed"),th.id(),(now-began) as f64/1000.0);
}

static COMMAND_COUNT: AtomicU64 = AtomicU64::new(0);
static COMMAND_US: AtomicU64 = AtomicU64::new(0);
static UPDATE_LAST: AtomicU64 = AtomicU64::new(0);
thread_local! { static UPDATE_START: std::cell::Cell<u64> = const { std::cell::Cell::new(0) }; }
thread_local! {
    static UPDATE_CPU: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
    static LOCK_START: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
    static LOCK_TIMES: std::cell::Cell<[u64;3]> = const { std::cell::Cell::new([0;3]) };
}
#[cfg(windows)]
fn cpu_us() -> u64 {
    #[repr(C)] struct Ft {low:u32,high:u32}
    #[link(name="kernel32")] unsafe extern "system" {
        fn GetCurrentThread() -> *mut std::ffi::c_void;
        fn GetThreadTimes(t:*mut std::ffi::c_void,c:*mut Ft,e:*mut Ft,k:*mut Ft,u:*mut Ft)->i32;
    }
    let mut c=Ft{low:0,high:0};let mut e=Ft{low:0,high:0};let mut k=Ft{low:0,high:0};let mut u=Ft{low:0,high:0};
    if unsafe{GetThreadTimes(GetCurrentThread(),&mut c,&mut e,&mut k,&mut u)}==0{return 0;}
    (((k.high as u64)<<32|k.low as u64)+((u.high as u64)<<32|u.low as u64))/10
}
#[cfg(not(windows))] fn cpu_us()->u64{0}
pub fn update_span(phase: u64) {
    let now=stamp();
    if phase==0 {UPDATE_START.with(|v|v.set(now));UPDATE_CPU.with(|v|v.set(cpu_us()));LOCK_TIMES.with(|v|v.set([0;3]));return;}
    if phase>=2 {
        let slot=((phase-2)/2) as usize;
        if phase%2==0 {LOCK_START.with(|v|v.set(now));}
        else if slot<3 {let elapsed=now.saturating_sub(LOCK_START.with(|v|v.get()));LOCK_TIMES.with(|v|{let mut a=v.get();a[slot]+=elapsed;v.set(a);});}
        return;
    }
    let began=UPDATE_START.with(|v|v.replace(0));
    if now.saturating_sub(began)<=100_000{return;}
    let last=UPDATE_LAST.load(Ordering::Relaxed);
    if now.saturating_sub(last)<500_000 || UPDATE_LAST.compare_exchange(last,now,Ordering::Relaxed,Ordering::Relaxed).is_err(){return;}
    let th=std::thread::current();
    eprintln!("[audio-update] thread={} id={:?} stage=audNorthAudioEngine::Update start_us={began} end_us={now} duration_ms={:.1} command_calls={} last_command_us={} cpu_us={} lock_us={:?}",th.name().unwrap_or("unnamed"),th.id(),(now-began) as f64/1000.0,COMMAND_COUNT.load(Ordering::Relaxed),COMMAND_US.load(Ordering::Relaxed),cpu_us().saturating_sub(UPDATE_CPU.with(|v|v.get())),LOCK_TIMES.with(|v|v.get()));
}
