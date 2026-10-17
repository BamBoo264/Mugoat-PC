//! Host-specific operations; the guest engine and multiplayer wire format stay unchanged.
#[cfg(unix)]
pub use std::os::unix::fs::{FileExt, MetadataExt};

pub fn gpu_backends() -> wgpu::Backends {
    #[cfg(windows)]
    { match std::env::var("MUGOAT_GPU").unwrap_or_default().to_ascii_lowercase().as_str() {
        "dx12" => wgpu::Backends::DX12,
        "vulkan" => wgpu::Backends::VULKAN,
        _ => wgpu::Backends::VULKAN | wgpu::Backends::DX12,
    } }
    #[cfg(target_vendor = "apple")]
    { wgpu::Backends::METAL }
}

#[cfg(windows)]
pub trait FileExt {
    fn read_at(&self, buf: &mut [u8], offset: u64) -> std::io::Result<usize>;
    fn read_exact_at(&self, buf: &mut [u8], offset: u64) -> std::io::Result<()>;
    fn write_all_at(&self, buf: &[u8], offset: u64) -> std::io::Result<()>;
}
#[cfg(windows)]
impl FileExt for std::fs::File {
    fn write_all_at(&self, mut buf: &[u8], mut offset: u64) -> std::io::Result<()> {
        use std::io::{Seek, SeekFrom};
        let mut file = self;
        let saved = file.stream_position()?;
        let result = (|| {
            while !buf.is_empty() {
                match std::os::windows::fs::FileExt::seek_write(self, buf, offset) {
                    Ok(0) => return Err(std::io::ErrorKind::WriteZero.into()),
                    Ok(n) => { offset += n as u64; buf = &buf[n..]; }
                    Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {},
                    Err(e) => return Err(e),
                }
            }
            Ok(())
        })();
        let restore = file.seek(SeekFrom::Start(saved));
        match (result, restore) { (Ok(()), Ok(_)) => Ok(()), (Err(e), _) | (_, Err(e)) => Err(e) }
    }
    fn read_at(&self, buf: &mut [u8], offset: u64) -> std::io::Result<usize> {
        use std::io::{Seek, SeekFrom};
        // All guest file operations are protected by fs::FS; streaming handles
        // belong to one I/O thread. Restore the cursor, as POSIX pread requires.
        let mut file = self;
        let saved = file.stream_position()?;
        let result = std::os::windows::fs::FileExt::seek_read(self, buf, offset);
        let restore = file.seek(SeekFrom::Start(saved));
        match (result, restore) { (Ok(n), Ok(_)) => Ok(n), (Err(e), _) | (_, Err(e)) => Err(e) }
    }
    fn read_exact_at(&self, mut buf: &mut [u8], mut offset: u64) -> std::io::Result<()> {
        while !buf.is_empty() {
            match self.read_at(buf, offset) {
                Ok(0) => return Err(std::io::ErrorKind::UnexpectedEof.into()),
                Ok(n) => { offset += n as u64; buf = &mut buf[n..]; }
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {},
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

#[cfg(windows)]
pub trait MetadataExt { fn atime(&self) -> i64; fn mtime(&self) -> i64; fn ctime(&self) -> i64; fn ino(&self) -> u64; }
#[cfg(windows)]
fn unix_seconds(t: std::io::Result<std::time::SystemTime>) -> i64 {
    t.ok().and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok()).map(|d| d.as_secs() as i64).unwrap_or(0)
}
#[cfg(windows)]
impl MetadataExt for std::fs::Metadata {
    fn atime(&self) -> i64 { unix_seconds(self.accessed()) }
    fn mtime(&self) -> i64 { unix_seconds(self.modified()) }
    fn ctime(&self) -> i64 { unix_seconds(self.created()) }
    // Stable Rust Metadata does not expose a Windows file index. The guest
    // stat consumer receives an unavailable inode, not a fabricated identity.
    fn ino(&self) -> u64 { 0 }
}

#[cfg(windows)]
pub fn footprint_mb() -> u64 {
    use std::ffi::c_void;
    #[repr(C)]
    struct Counters { cb: u32, faults: u32, peak: usize, working: usize, paged_peak: usize,
        paged: usize, nonpaged_peak: usize, nonpaged: usize, pagefile: usize, pagefile_peak: usize }
    #[link(name="kernel32")]
    extern "system" { fn GetCurrentProcess() -> *mut c_void; }
    #[link(name="psapi")]
    extern "system" { fn GetProcessMemoryInfo(p: *mut c_void, c: *mut Counters, size: u32) -> i32; }
    let mut c: Counters = unsafe { std::mem::zeroed() };
    c.cb = std::mem::size_of::<Counters>() as u32;
    if unsafe { GetProcessMemoryInfo(GetCurrentProcess(), &mut c, c.cb) } == 0 { 0 } else { (c.working as u64) >> 20 }
}

#[cfg(all(test,windows))]
mod tests {
    use super::*;
    use std::io::{Read,Seek,SeekFrom,Write};
    #[test]
    fn positioned_reads_preserve_cursor_and_report_eof() {
        let p=std::env::temp_dir().join(format!("mugoat-pread-{}",std::process::id()));
        let mut f=std::fs::OpenOptions::new().create_new(true).read(true).write(true).open(&p).unwrap();
        f.write_all(b"0123456789").unwrap();f.seek(SeekFrom::Start(7)).unwrap();
        let mut buf=[0;3];FileExt::read_exact_at(&f,&mut buf,2).unwrap();assert_eq!(&buf,b"234");
        assert_eq!(f.stream_position().unwrap(),7);
        let mut b=[0];f.read_exact(&mut b).unwrap();assert_eq!(&b,b"7");
        assert_eq!(FileExt::read_exact_at(&f,&mut buf,9).unwrap_err().kind(),std::io::ErrorKind::UnexpectedEof);
        assert_eq!(f.stream_position().unwrap(),8);
        FileExt::write_all_at(&f,b"XY",3).unwrap();assert_eq!(f.stream_position().unwrap(),8);
        FileExt::read_exact_at(&f,&mut buf,2).unwrap();assert_eq!(&buf,b"2XY");
        drop(f);std::fs::remove_file(p).unwrap();
    }
}
