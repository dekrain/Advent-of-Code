//! Overengineering, part 1

use std::{str::{Lines, FromStr}, collections::HashMap, hash::{Hash, BuildHasher}};

#[allow(unused)]
trait HierarchyReceiver {
    type DirectoryData: Default;
    type FileData: Default;
    const NEEDS_STRICT_FINISH_DIR: bool = false;

    fn new_dir(parent: &mut Directory<Self>, dir: &mut Directory<Self>) {}
    fn new_file(parent: &mut Directory<Self>, file: &mut File<Self>) -> bool { true }
    fn finish_dir(parent: Option<&mut Directory<Self>>, dir: &mut Directory<Self>) {}
}

struct Directory<Receiver: ?Sized + HierarchyReceiver> {
    children: HashMap<String, HierarchyEntry<Receiver>>,
    #[allow(unused)] pub data: Receiver::DirectoryData,
}

impl<Receiver: ?Sized + HierarchyReceiver> Default for Directory<Receiver> {
    fn default() -> Self {
        Directory {
            children: HashMap::default(),
            data: Default::default(),
        }
    }
}

struct File<Receiver: ?Sized + HierarchyReceiver> {
    pub size: usize,
    #[allow(unused)] pub data: Receiver::FileData,
}

impl<Receiver: ?Sized + HierarchyReceiver> File<Receiver> {
    pub fn new(size: usize) -> Self {
        File {
            size,
            data: Default::default(),
        }
    }
}

enum HierarchyEntry<Receiver: ?Sized + HierarchyReceiver> {
    File(File<Receiver>),
    Directory(Directory<Receiver>),
}

struct Hierarchy<Receiver: ?Sized + HierarchyReceiver> {
    root: Directory<Receiver>,
}

impl<Receiver: ?Sized + HierarchyReceiver> Default for Hierarchy<Receiver> {
    fn default() -> Self {
        Hierarchy {
            root: Directory::default(),
        }
    }
}

fn insert_map<K: Eq + Hash, V, S: BuildHasher>(map: &mut HashMap<K, V, S>, key: K, value: V) -> &mut V {
    match map.entry(key) {
        std::collections::hash_map::Entry::Vacant(vac) => {
            vac.insert(value)
        }
        std::collections::hash_map::Entry::Occupied(_) => {
            panic!("Entry already present")
        }
    }
}

impl<Receiver: ?Sized + HierarchyReceiver> Hierarchy<Receiver> {
    pub fn root(&self) -> &Directory<Receiver> {
        &self.root
    }

    pub fn parse(&mut self, mut input_stream: Lines) {
        let mut stack: Vec<*mut Directory<Receiver>> = Vec::default();
        loop {
            let mut line = match input_stream.next() {
                Some(line) => line,
                None => break,
            };
            if line.starts_with("$ ") {
                line = &line[2..];
                'next_line: loop {
                    let (cmd, data) = line.split_once(' ').unwrap_or((line, ""));
                    match cmd {
                        "ls" => {
                            debug_assert_eq!(data, "");
                            for sub in &mut input_stream {
                                if sub.starts_with("$ ") {
                                    line = &sub[2..];
                                    continue 'next_line;
                                }
                                let p: *const i32 = &2;
                                unsafe{ *p };
                                let parent = unsafe{&mut *stack.last().map(|a| *a).unwrap_or(&mut self.root)};
                                if sub.starts_with("dir ") {
                                    let name = sub[4..].into();
                                    let dir = Directory::default();
                                    insert_map(&mut parent.children, name, HierarchyEntry::Directory(dir));
                                } else {
                                    let (size, name) = sub.split_once(' ').unwrap();
                                    let size = usize::from_str(size).unwrap();
                                    let mut file = File::new(size);
                                    if Receiver::new_file(parent, &mut file) {
                                        insert_map(&mut parent.children, name.into(), HierarchyEntry::File(file));
                                    }
                                }
                            }
                        }
                        "cd" => {
                            match data {
                                "/" => {
                                    if !stack.is_empty() {
                                        if Receiver::NEEDS_STRICT_FINISH_DIR {
                                            unsafe{ self.pop_dirs(&mut stack); }
                                        } else {
                                            stack.clear();
                                        }
                                    }
                                }
                                ".." => {
                                    let dir = unsafe{&mut *stack.pop().unwrap()};
                                    let parent = unsafe{&mut *stack.last().map(|a| *a).unwrap_or(&mut self.root)};
                                    Receiver::finish_dir(Some(parent), dir);
                                }
                                dir => {
                                    let parent = unsafe{&mut *stack.last().map(|a| *a).unwrap_or(&mut self.root)};
                                    let dir = parent.children.get_mut(dir).unwrap();
                                    stack.push(match dir {
                                        HierarchyEntry::Directory(ref mut dir) => dir,
                                        _ => panic!("File is not a directory"),
                                    });
                                }
                            }
                        }
                        _ => panic!("Unknown command {cmd}"),
                    }
                    break;
                }
            }
        }

        if Receiver::NEEDS_STRICT_FINISH_DIR && !stack.is_empty() {
            unsafe{ self.pop_dirs(&mut stack); }
        }

        Receiver::finish_dir(None, &mut self.root);
    }

    /// # Safety
    /// The stack must be ensured not to be empty before calling this function
    unsafe fn pop_dirs(&mut self, stack: &mut Vec<*mut Directory<Receiver>>) {
        let mut dir = unsafe{&mut *stack.pop().unwrap_unchecked()};
        while let Some(parent) = stack.pop() {
            let parent = unsafe{&mut *parent};
            Receiver::finish_dir(Some(parent), dir);
            dir = parent;
        }
        Receiver::finish_dir(Some(&mut self.root), dir);
    }

    #[cfg(debug_assertions)]
    pub fn debug_print(&self) {
        use std::io::Write;
        let mut lock = std::io::stdout().lock();
        print_dir(&mut lock, "/", &self.root, 0);

        fn prefix(lock: &mut std::io::StdoutLock, mut depth: usize) {
            const NUM_TABS: usize = 64;
            static TABS: [u8; NUM_TABS] = [b'\t'; NUM_TABS];

            while depth > NUM_TABS {
                lock.write_all(&TABS).unwrap();
                depth -= NUM_TABS;
            }
            lock.write_all(&TABS[NUM_TABS - depth..]).unwrap();
        }

        fn print_dir<R: ?Sized + HierarchyReceiver>(
            lock: &mut std::io::StdoutLock,
            name: &str,
            dir: &Directory<R>,
            depth: usize
        ) {
            prefix(lock, depth);
            writeln!(lock, "{name} (dir)").unwrap();

            let depth = depth + 1;
            for (name, entry) in &dir.children {
                match entry {
                    HierarchyEntry::Directory(dir) => {
                        print_dir(lock, name, dir, depth);
                    }
                    HierarchyEntry::File(file) => {
                        print_file(lock, name, file, depth);
                    }
                }
            }
        }

        fn print_file<R: ?Sized + HierarchyReceiver>(
            lock: &mut std::io::StdoutLock,
            name: &str,
            file: &File<R>,
            depth: usize
        ) {
            prefix(lock, depth);
            writeln!(lock, "{name} (file, size={})", file.size).unwrap();
        }
    }
}

struct Part1Recv;
impl Part1Recv {
    const MAX_SIZE: usize = 100_000;
}

impl HierarchyReceiver for Part1Recv {
    type DirectoryData = Part1Dir;
    type FileData = ();
    const NEEDS_STRICT_FINISH_DIR: bool = true;

    fn new_dir(_parent: &mut Directory<Self>, _dir: &mut Directory<Self>) {}

    fn new_file(parent: &mut Directory<Self>, file: &mut File<Self>) -> bool {
        parent.data.total_size += file.size;
        false
    }

    fn finish_dir(mut parent: Option<&mut Directory<Self>>, dir: &mut Directory<Self>) {
        if let Some(ref mut parent) = &mut parent {
            parent.data.subdir_size += dir.data.subdir_size;
            parent.data.total_size += dir.data.total_size;
        }
        if let size@ ..=Self::MAX_SIZE = dir.data.total_size {
            parent.unwrap_or(dir).data.subdir_size += size;
        }
    }
}

struct Part1Dir {
    total_size: usize,
    subdir_size: usize,
}

impl Default for Part1Dir {
    fn default() -> Self {
        Part1Dir {
            total_size: 0,
            subdir_size: 0,
        }
    }
}

pub fn task(input: &str) -> (usize, ) {
    // Part 1
    let mut hier = Hierarchy::<Part1Recv>::default();
    hier.parse(input.lines());
    #[cfg(debug_assertions)] {
        hier.debug_print();
    }
    (hier.root().data.subdir_size, )
}

#[cfg(test)]
mod test {
    use super::*;

    struct NullHier;
    impl HierarchyReceiver for NullHier {
        type DirectoryData = ();
        type FileData = ();
    }

    #[test]
    fn example() {
        let input =
r"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        // Hierarchy
        {
            let mut hier = Hierarchy::<NullHier>::default();
            hier.parse(input.lines());
            hier.debug_print();
        }

        // Part 1
        {
            let mut hier = Hierarchy::<Part1Recv>::default();
            hier.parse(input.lines());
            println!("{}", hier.root.data.subdir_size);
        }
    }
}
