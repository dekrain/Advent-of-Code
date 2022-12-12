#![allow(non_snake_case)]

use std::{fs, io, os::unix::io::AsFd};

mod lisp;
mod line;
mod mmap;

// Tasks
mod t1;
mod t2;

fn open_input<P: AsRef<std::path::Path>>(path: P) -> io::Result<mmap::MemoryView> {
    let file = fs::File::open(path)?;
    let size = file.metadata()?.len() as usize;
    mmap::MemoryView::map_file(file.as_fd(), 0, size, mmap::MemoryProtectionFlags::Read, false)
}

fn do_task<P: AsRef<std::path::Path>>(input_path: P, task: fn(&str)) {
    let view = open_input(input_path).unwrap();
    task(std::str::from_utf8(view.as_bytes().unwrap()).unwrap());
}

fn main() {
    macro_rules! task {
        [$($num:ident)*] => {
            $(do_task(concat!("inputs/", stringify!($num), ".input"), $num::task);)*
        };
    }
    task![ t1 t2 ];
}
