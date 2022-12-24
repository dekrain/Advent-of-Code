#[rustc_layout_scalar_valid_range_start(0)]
#[rustc_layout_scalar_valid_range_end(9)]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct TreeHeight(u8);

struct BitVec {
    data: Box<[u8]>,
    #[cfg(debug_assertions)]
    size: usize,
}

impl BitVec {
    pub fn new(size: usize) -> Self {
        BitVec {
            data: unsafe{ Box::<[u8]>::new_zeroed_slice((size + 7) >> 3).assume_init() },
            #[cfg(debug_assertions)]
            size,
        }
    }

    pub fn get(&self, idx: usize) -> bool {
        #[cfg(debug_assertions)]
        assert!(idx < self.size);
        self.data[idx >> 3].wrapping_shr(idx as u32) & 1 != 0
    }

    pub fn set(&mut self, idx: usize) {
        #[cfg(debug_assertions)]
        assert!(idx < self.size);
        self.data[idx >> 3] |= 1u8.wrapping_shl(idx as u32);
    }
}

macro unreachable($($args:tt)*) {
    {
        #[cfg(debug_assertions)]
        std::unreachable!($($args)*);
        #[cfg(not(debug_assertions))]
        unsafe{ std::hint::unreachable_unchecked(); }
    }
}

impl From<u8> for TreeHeight {
    fn from(value: u8) -> Self {
        match value {
            b'0'..=b'9' => unsafe{ TreeHeight(value - b'0') },
            _ => unreachable!("Invalid byte value"),
        }
    }
}

pub fn task(input: &str) -> (u32, u32) {
    let width = input.find('\n').unwrap();
    debug_assert_eq!(input.len() % (width + 1), 0);
    let height = input.len() / (width + 1);
    let size = width * height;

    let hor_map = {
        let mut hor_map = Box::<[TreeHeight]>::new_uninit_slice(size);
        for y in 0..height {
            for x in 0..width {
                hor_map[y*width + x].write(TreeHeight::from(input.as_bytes()[y*(width + 1) + x]));
            }
        }
        unsafe{ hor_map.assume_init() }
    };

    let ver_map = {
        let mut ver_map = Box::<[TreeHeight]>::new_uninit_slice(size);
        for x in 0..width {
            for y in 0..height {
                ver_map[x*height + y].write(hor_map[y*width + x]);
            }
        }
        unsafe{ ver_map.assume_init() }
    };

    // Part 1
    let p1 = {
        let mut result = 0;
        let mut visible = BitVec::new(size);

        // Count left-right
        for y in 0..height {
            for x in 0..width {
                let idx = y*width + x;
                if visible.get(idx) {
                    continue;
                }
                let v = hor_map[idx];
                // Left edge
                if hor_map[y*width..y*width + x].iter().all(|&n| n < v)
                // Right edge
                || hor_map[y*width + x + 1..(y+1)*width].iter().all(|&n| n < v) {
                    visible.set(idx);
                    result += 1;
                }
            }
        }

        // Count top-bottom
        for x in 0..width {
            for y in 0..height {
                let idx = y*width + x;
                if visible.get(idx) {
                    continue;
                }
                let v = ver_map[x*height + y];
                // Top edge
                if ver_map[x*height..x*height + y].iter().all(|&n| n < v)
                // Right edge
                || ver_map[x*height + y + 1..(x+1)*height].iter().all(|&n| n < v) {
                    visible.set(idx);
                    result += 1;
                }
            }
        }
        result
    };

    // Part 2
    let p2 = {
        #[derive(Clone)]
        struct FoldState {
            count: u32,
            value: TreeHeight,
        }

        use std::ops::ControlFlow::{self, Break, Continue};

        trait Unpack<T> {
            fn unpack(self) -> T;
        }

        impl Unpack<u32> for ControlFlow<u32, FoldState> {
            fn unpack(self) -> u32 {
                match self {
                    Break(count) => count,
                    Continue(state) => state.count,
                }
            }
        }

        fn p2fold(mut prev: FoldState, &next: &TreeHeight) -> ControlFlow<u32, FoldState> {
            prev.count += 1;
            if next >= prev.value {
                Break(prev.count)
            } else {
                Continue(prev)
            }
        }

        (1..height-1).map(|y| {
            (1..width-1).map(|x| {
                let v = hor_map[y*width + x];
                let state = FoldState {
                    count: 0,
                    value: v,
                };
                let left = hor_map[y*width..y*width + x].iter().rev().try_fold(state.clone(), p2fold).unpack();
                let right = hor_map[y*width + x + 1..(y+1)*width].iter().try_fold(state.clone(), p2fold).unpack();
                let top = ver_map[x*height..x*height + y].iter().rev().try_fold(state.clone(), p2fold).unpack();
                let bottom = ver_map[x*height + y + 1..(x+1)*height].iter().try_fold(state, p2fold).unpack();
                left * right * top * bottom
            }).fold(0, u32::max)
        }).fold(0, u32::max)
    };

    (p1, p2)
}

#[cfg(test)]
mod test {
    use super::task;

    #[test]
    fn test() {
        let (p1, p2) = task("30373\n25512\n65332\n33549\n35390\n");
        assert_eq!(p1, 21);
        assert_eq!(p2, 8);
    }
}
