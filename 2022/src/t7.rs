//! Overengineering, part 1

use std::str::{Lines, FromStr};
use std::collections::{HashMap, hash_map};
use std::hash::{Hash, BuildHasher};

trait HierarchyReceiver: {
    type DirectoryData: Default;
    type FileData: Default;
    const NEEDS_STRICT_FINISH_DIR: bool = false;
}

#[allow(unused)]
trait Traverser<Receiver: ?Sized + HierarchyReceiver> {
    fn new_dir(&mut self, parent: &mut Directory<Receiver>, dir: &mut Directory<Receiver>) {}
    fn new_file(&mut self, parent: &mut Directory<Receiver>, file: &mut File<Receiver>) -> bool { true }
    fn finish_dir(&mut self, parent: Option<&mut Directory<Receiver>>, dir: &mut Directory<Receiver>) {}
}

impl HierarchyReceiver for () {
    type DirectoryData = ();
    type FileData = ();
}
impl<Receiver: ?Sized + HierarchyReceiver> Traverser<Receiver> for () {}

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

    /// Parses the filesystem tree, using `traverser` to act on nodes
    ///
    /// # Safety
    /// I checked it with miri, so it must be safe, right?
    pub fn parse<T: Traverser<Receiver>>(&mut self, mut input_stream: Lines, mut traverser: T)-> T {
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
                                    if traverser.new_file(parent, &mut file) {
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
                                            unsafe{ self.pop_dirs(&mut stack, &mut traverser); }
                                        } else {
                                            stack.clear();
                                        }
                                    }
                                }
                                ".." => {
                                    let dir = unsafe{&mut *stack.pop().unwrap()};
                                    let parent = unsafe{&mut *stack.last().map(|a| *a).unwrap_or(&mut self.root)};
                                    traverser.finish_dir(Some(parent), dir);
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
            } else {
                panic!("Invalid command line");
            }
        }

        if Receiver::NEEDS_STRICT_FINISH_DIR && !stack.is_empty() {
            unsafe{ self.pop_dirs(&mut stack, &mut traverser); }
        }

        traverser.finish_dir(None, &mut self.root);
        traverser
    }

    /// Walk the parsed tree, as if by [`parse`]
    /// 
    /// # Safety
    /// I checked it with miri, so it must be safe, right?
    ///
    /// [`parse`]: Hierarchy::parse
    pub fn walk<T: Traverser<Receiver>>(&mut self, mut traverser: T) -> T {
        struct Entry<'a, R: ?Sized + HierarchyReceiver> {
            dir: *mut Directory<R>,
            iter: hash_map::ValuesMut<'a, String, HierarchyEntry<R>>,
        }
        impl<'a, R: ?Sized + HierarchyReceiver> Entry<'a, R> {
            #[inline]
            fn new(dir: &'a mut Directory<R>) -> Self {
                Entry {
                    dir,
                    iter: dir.children.values_mut(),
                }
            }

            #[inline(always)]
            unsafe fn dir(&mut self) -> &'a mut Directory<R> {
                &mut *self.dir
            }
        }
        let mut stack = vec![ Entry::new(&mut self.root) ];
        while let Some(top) = stack.last_mut() {
            match top.iter.next() {
                Some(sub) => {
                    match sub {
                        HierarchyEntry::Directory(dir) => {
                            unsafe{ traverser.new_dir(top.dir(), dir); }
                            stack.push(Entry::new(dir));
                        }
                        HierarchyEntry::File(file) => {
                            unsafe{ traverser.new_file(top.dir(), file); }
                        }
                    }
                }
                None => {
                    unsafe {
                        let mut top = stack.pop().unwrap_unchecked();
                        traverser.finish_dir(stack.last_mut().map(|e| e.dir()), top.dir());
                    }
                }
            }
        }
        traverser
    }

    /// # Safety
    /// The stack must be ensured not to be empty before calling this function
    unsafe fn pop_dirs<T: Traverser<Receiver>>(&mut self, stack: &mut Vec<*mut Directory<Receiver>>, traverser: &mut T) {
        let mut dir = unsafe{&mut *stack.pop().unwrap_unchecked()};
        while let Some(parent) = stack.pop() {
            let parent = unsafe{&mut *parent};
            traverser.finish_dir(Some(parent), dir);
            dir = parent;
        }
        traverser.finish_dir(Some(&mut self.root), dir);
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
}

impl Traverser<Part1Recv> for Part1Recv {
    fn new_file(&mut self, parent: &mut Directory<Self>, file: &mut File<Self>) -> bool {
        parent.data.total_size += file.size;
        false
    }

    fn finish_dir(&mut self, mut parent: Option<&mut Directory<Self>>, dir: &mut Directory<Self>) {
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

struct Part2Recv;
struct Part2Walker {
    free_needed: usize,
    min_size: usize,
}

impl Part2Recv {
    const MAX_SPACE: usize = 70_000_000 - 30_000_000;
}

impl Part2Walker {
    fn new(free_needed: usize) -> Self {
        Part2Walker {
            free_needed,
            min_size: usize::MAX,
        }
    }

    fn from_root(root: &Part2Dir) -> Self {
        Self::new(root.total_size - Part2Recv::MAX_SPACE)
    }
}

impl HierarchyReceiver for Part2Recv {
    type DirectoryData = Part2Dir;
    type FileData = ();
    const NEEDS_STRICT_FINISH_DIR: bool = true;
}

impl Traverser<Part2Recv> for Part2Recv {
    fn new_file(&mut self, parent: &mut Directory<Self>, file: &mut File<Self>) -> bool {
        parent.data.total_size += file.size;
        false
    }

    fn finish_dir(&mut self, parent: Option<&mut Directory<Self>>, dir: &mut Directory<Self>) {
        if let Some(parent) = parent {
            parent.data.total_size += dir.data.total_size;
        }
    }
}

impl Traverser<Part2Recv> for Part2Walker {
    fn new_dir(&mut self, _parent: &mut Directory<Part2Recv>, dir: &mut Directory<Part2Recv>) {
        if self.free_needed < dir.data.total_size && dir.data.total_size < self.min_size {
            self.min_size = dir.data.total_size;
        }
    }
}

struct Part2Dir {
    total_size: usize,
}

impl Default for Part2Dir {
    fn default() -> Self {
        Part2Dir {
            total_size: 0,
        }
    }
}

pub fn task(input: &str) -> (usize, usize) {
    // Part 1
    let p1 = {
        let mut hier = Hierarchy::<Part1Recv>::default();
        hier.parse(input.lines(), Part1Recv);
        #[cfg(debug_assertions)] {
            hier.debug_print();
        }
        hier.root().data.subdir_size
    };

    // Part 2
    let p2 = {
        let mut hier = Hierarchy::<Part2Recv>::default();
        hier.parse(input.lines(), Part2Recv);
        hier.walk(Part2Walker::from_root(&hier.root().data)).min_size
    };

    (p1, p2)
}

#[cfg(test)]
mod test {
    use super::*;

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
            let mut hier = Hierarchy::<()>::default();
            hier.parse(input.lines(), ());
            hier.debug_print();
        }

        // Part 1
        {
            let mut hier = Hierarchy::<Part1Recv>::default();
            hier.parse(input.lines(), Part1Recv);
            assert_eq!(95437, hier.root().data.subdir_size);
        }

        // Part 2
        {
            let mut hier = Hierarchy::<Part2Recv>::default();
            hier.parse(input.lines(), Part2Recv);
            assert_eq!(24933642, hier.walk(Part2Walker::from_root(&hier.root().data)).min_size);
        }
    }
}
