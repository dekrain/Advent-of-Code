#![allow(non_snake_case)]
#![feature(iter_array_chunks, iter_next_chunk, decl_macro, rustc_attrs, new_uninit, assert_matches)]

use std::{fs, io, os::unix::io::AsFd, fmt::Display};

mod lisp;
mod line;
mod mmap;

// Tasks
mod t1;
mod t2;
mod t3;
mod t4;
mod t5;
mod t6;
mod t7;
mod t8;
mod t9;
mod t10;

pub(crate) macro unreachable($($args:tt)*) {
    {
        #[cfg(debug_assertions)]
        std::unreachable!($($args)*);
        #[cfg(not(debug_assertions))]
        unsafe{ std::hint::unreachable_unchecked(); }
    }
}

trait TaskPrint {
    fn print(task: u32, result: Self);
}

impl<T: Display> TaskPrint for (T, ) {
    fn print(task: u32, result: Self) {
        println!("Task {task}: {}", result.0);
    }
}

impl<T: Display, U: Display> TaskPrint for (T, U) {
    fn print(task: u32, result: Self) {
        println!("Task {task}: {} {}", result.0, result.1);
    }
}

/*impl<const N: usize> TaskPrint for [u8; N] {
    fn print(task: u32, result: Self) {}
}*/

fn open_input<P: AsRef<std::path::Path>>(path: P) -> io::Result<mmap::MemoryView> {
    let file = fs::File::open(path)?;
    let size = file.metadata()?.len() as usize;
    mmap::MemoryView::map_file(file.as_fd(), 0, size, mmap::MemoryProtectionFlags::Read, false)
}

fn do_task<P: AsRef<std::path::Path>, Res: TaskPrint>(input_path: P, id: u32, task: fn(&str) -> Res) {
    let view = open_input(input_path).unwrap();
    TaskPrint::print(id, task(std::str::from_utf8(view.as_bytes().unwrap()).unwrap()));
}

fn main() {
    macro task($($impl:ident $id:literal)*) {
        $(do_task(concat!("inputs/", stringify!($id), ".input"), $id, $impl::task);)*
    }
    task![ t1 1 t2 2 t3 3 t4 4 t5 5 t6 6 t7 7 t8 8 t9 9 t10 10 ];
}
