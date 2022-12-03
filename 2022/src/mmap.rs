#![allow(dead_code)]

use std::io;
use std::os::unix::io::{BorrowedFd, AsRawFd, RawFd};

pub struct MemoryView {
    addr: *mut libc::c_void,
    len: usize,
    prot: MemoryProtectionFlags,
}

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum MemoryProtectionFlags {
    None = 0,
    Read = libc::PROT_READ,
    Write = libc::PROT_WRITE,
    Exec = libc::PROT_EXEC,
}

impl MemoryProtectionFlags {
    pub fn can_read(self) -> bool {
        self as i32 & Self::Read as i32 == Self::Read as i32
    }

    pub fn can_write(self) -> bool {
        self as i32 & (Self::Write as i32 | Self::Read as i32) == Self::Write as i32 | Self::Read as i32
    }
}

impl MemoryView {
    pub fn map_file(fd: BorrowedFd, offset: i64, len: usize, prot: MemoryProtectionFlags, shared: bool) -> io::Result<Self> {
        unsafe {
            Self::raw_mmap(len, prot, shared, fd.as_raw_fd(), offset)
        }
    }

    /// `len` must be aligned to page boundary
    pub fn map_area(len: usize, prot: MemoryProtectionFlags, shared: bool) -> io::Result<Self> {
        unsafe {
            Self::raw_mmap(len, prot, shared, -1, 0)
        }
    }

    unsafe fn raw_mmap(len: usize, prot: MemoryProtectionFlags, shared: bool, fd: RawFd, offs: i64) -> io::Result<Self> {
        let res = libc::mmap(std::ptr::null_mut(), len, prot as libc::c_int, if shared { libc::MAP_SHARED } else { libc::MAP_PRIVATE }, fd, offs);
        if res == libc::MAP_FAILED {
            Err(io::Error::last_os_error())
        } else {
            Ok(MemoryView {
                addr: res, len, prot,
            })
        }
    }
}

impl MemoryView {
    pub fn as_bytes(&self) -> Option<&[u8]> {
        if self.prot.can_read() {
            Some(unsafe { std::slice::from_raw_parts(self.addr as *const u8, self.len) })
        } else {
            None
        }
    }

    pub fn as_bytes_mut(&self) -> Option<&mut [u8]> {
        if self.prot.can_write() {
            Some(unsafe { std::slice::from_raw_parts_mut(self.addr as *mut u8, self.len) })
        } else {
            None
        }
    }
}

impl Drop for MemoryView {
    fn drop(&mut self) {
        unsafe {
            if libc::munmap(self.addr, self.len) != 0 {
                #[cfg(debug_assertions)]
                panic!("Munmap failed");
                #[cfg(not(debug_assertions))]
                eprintln!("[WARNING] Munmap failed");
            }
        }
    }
}
