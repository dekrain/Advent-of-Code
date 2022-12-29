use crate::unreachable;
use std::collections::HashSet;

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

// Tail position = head position + {dir}
#[derive(Clone, Copy, Debug)]
#[repr(i8)]
enum TailPosition {
    TL=-4, TC=-3, TR=-2,
    CL=-1, CC= 0, CR= 1,
    BL= 2, BC= 3, BR= 4,
}

#[derive(Clone, Copy)]
enum Distance {
    L2, L1, Eq, G1, G2,
}

impl TailPosition {
    /// Update the position after head moved
    pub fn moved(self, dir: Direction) -> Self {
        // It was ~6:03 when I started this
        use {TailPosition::*, Direction::*};
        match (self, dir) {
            (TL, Left) => TC,
            (TL, Right) => CL,
            (TL, Up) => CL,
            (TL, Down) => TC,
            (TC, Left) => TR,
            (TC, Right) => TL,
            (TC, Up) => CC,
            (TC, Down) => TC,
            (TR, Left) => CR,
            (TR, Right) => TC,
            (TR, Up) => CR,
            (TR, Down) => TC,
            (CL, Left) => CC,
            (CL, Right) => CL,
            (CL, Up) => BL,
            (CL, Down) => TL,
            (CC, Left) => CR,
            (CC, Right) => CL,
            (CC, Up) => BC,
            (CC, Down) => TC,
            (CR, Left) => CR,
            (CR, Right) => CC,
            (CR, Up) => BR,
            (CR, Down) => TR,
            (BL, Left) => BC,
            (BL, Right) => CL,
            (BL, Up) => BC,
            (BL, Down) => CL,
            (BC, Left) => BR,
            (BC, Right) => BL,
            (BC, Up) => BC,
            (BC, Down) => CC,
            (BR, Left) => CR,
            (BR, Right) => BC,
            (BR, Up) => BC,
            (BR, Down) => CR,
        }
        // It is now 6:10, and I am done.
    }

    /// Offset the head position to get the tail position
    pub fn offset(self, mut head: Pos) -> Pos {
        match self {
            TailPosition::TL | TailPosition::CL | TailPosition::BL => head.0 -= 1,
            TailPosition::TC | TailPosition::CC | TailPosition::BC => (),
            TailPosition::TR | TailPosition::CR | TailPosition::BR => head.0 += 1,
        }
        match self {
            TailPosition::TL | TailPosition::TC | TailPosition::TR => head.1 -= 1,
            TailPosition::CL | TailPosition::CC | TailPosition::CR => (),
            TailPosition::BL | TailPosition::BC | TailPosition::BR => head.1 += 1,
        }
        head
    }

    #[inline]
    pub fn hor(self) -> Distance {
        match self {
            TailPosition::TL | TailPosition::CL | TailPosition::BL => Distance::L1,
            TailPosition::TC | TailPosition::CC | TailPosition::BC => Distance::Eq,
            TailPosition::TR | TailPosition::CR | TailPosition::BR => Distance::G1,
        }
    }

    #[inline]
    pub fn ver(self) -> Distance {
        match self {
            TailPosition::TL | TailPosition::TC | TailPosition::TR => Distance::L1,
            TailPosition::CL | TailPosition::CC | TailPosition::CR => Distance::Eq,
            TailPosition::BL | TailPosition::BC | TailPosition::BR => Distance::G1,
        }
    }

    unsafe fn cmp_internal(lhs: Distance, rhs: Distance) -> Distance {
        match (lhs, rhs) {
            (Distance::L1, Distance::L1) => Distance::Eq,
            (Distance::L1, Distance::Eq) => Distance::L1,
            (Distance::L1, Distance::G1) => Distance::L2,
            (Distance::Eq, Distance::L1) => Distance::G1,
            (Distance::Eq, Distance::Eq) => Distance::Eq,
            (Distance::Eq, Distance::G1) => Distance::L1,
            (Distance::G1, Distance::L1) => Distance::G2,
            (Distance::G1, Distance::Eq) => Distance::G1,
            (Distance::G1, Distance::G1) => Distance::Eq,
            _ => unreachable!(),
        }
    }

    /// Compare positions horizontally
    #[inline(always)]
    pub fn cmp_hor(self, rhs: Self) -> Distance {
        unsafe{ Self::cmp_internal(self.hor(), rhs.hor()) }
    }

    /// Compare positions verically
    #[inline(always)]
    pub fn cmp_ver(self, rhs: Self) -> Distance {
        unsafe{ Self::cmp_internal(self.ver(), rhs.ver()) }
    }

    /// Returns the move of `self`, relative to previous position
    pub fn compose(head_move: Self, tail: &mut Self) -> std::ops::ControlFlow<(), Self> {
        use {TailPosition::*, Distance::*, std::ops::ControlFlow};
        // Fast path when nothing changed
        if matches!(head_move, CC) {
            return ControlFlow::Break(());
        }
        let hor = tail.cmp_hor(head_move);
        let ver = tail.cmp_ver(head_move);
        let rel = match (hor, ver) {
            (L2, L1 | L2) => BR,
            (L2, G1 | G2) => TR,
            (G2, L1 | L2) => BL,
            (G2, G1 | G2) => TL,
            (L1,      L2) => BR,
            (L1,      G2) => TR,
            (G1,      L2) => BL,
            (G1,      G2) => TL,
            (L2, Eq) => CR,
            (G2, Eq) => CL,
            (Eq, L2) => BC,
            (Eq, G2) => TC,
            _ => CC,
        };
        *tail = match (hor, ver) {
            (Eq     , Eq     ) => CC,
            (L1, L1) | (L2, L2) => TL,
            (L1, G1) | (L2, G2) => BL,
            (G1, L1) | (G2, L2) => TR,
            (G1, G1) | (G2, G2) => BR,

            (Eq, L1) | (L1 | Eq | G1, L2) => TC,
            (L1, Eq) | (L2, L1 | Eq | G1) => CL,
            (G1, Eq) | (G2, L1 | Eq | G1) => CR,
            (Eq, G1) | (L1 | Eq | G1, G2) => BC,
        };
        ControlFlow::Continue(rel)
    }

    unsafe fn diff(&mut self, mv: TailPosition) {
        *(self as *mut _ as *mut i8) += mv as i8
    }
}

type Pos = (i16, i16);

pub fn task(input: &str) -> (usize, usize) {
    let mut p1 = HashSet::new();
    let mut p2 = HashSet::new();

    let mut head: Pos = (0, 0);
    let mut tails = [TailPosition::CC; 9];
    p1.insert(head);
    for line in input.lines() {
        debug_assert_eq!(line.as_bytes()[1], b' ');
        let count: u32 = line[2..].parse().unwrap();
        let dir = match line.as_bytes()[0] {
            b'L' => Direction::Left,
            b'R' => Direction::Right,
            b'U' => Direction::Up,
            b'D' => Direction::Down,
            _ => unreachable!(),
        };
        let dir_pos = match dir {
            Direction::Left => TailPosition::CL,
            Direction::Right => TailPosition::CR,
            Direction::Up => TailPosition::TC,
            Direction::Down => TailPosition::BC,
        };
        for _ in 0..count {
            match dir {
                Direction::Left => head.0 -= 1,
                Direction::Right => head.0 += 1,
                Direction::Up => head.1 -= 1,
                Direction::Down => head.1 += 1,
            }
            tails.iter_mut().try_fold(dir_pos, TailPosition::compose);
            p1.insert(tails[0].offset(head));
            p2.insert(tails.iter().fold(head, |pos, tail| tail.offset(pos)));
        }
    }

    (p1.len(), p2.len())
}

#[cfg(test)]
mod test {
    use std::assert_matches::assert_matches;

    use super::*;

    #[test]
    fn test() {
        let res = task("R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2\n");
        assert_eq!(res.0, 13, "Part 1");
        assert_eq!(res.1, 1, "Part 2/1");

        let res = task("R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20\n");
        assert_eq!(res.1, 36, "Part 2/2");
    }

    #[test]
    fn dirs() {
        use {TailPosition::*, Direction::*};

        macro cases(
            $(($tail:ident, $dir:ident) => $res:ident,)*
        ) {
            $(
                assert_matches!($tail.moved($dir), $res);
            )*
        }

        cases! {
            // Center
            (CC, Left) => CR,
            (CC, Right) => CL,
            (CC, Up) => BC,
            (CC, Down) => TC,

            // Side
            (CL, Left) => CC,
            (CL, Right) => CL,
            (CL, Up) => BL,
            (CL, Down) => TL,

            // Corner
            (TR, Left) => CR,
            (TR, Right) => TC,
            (TR, Up) => CR,
            (TR, Down) => TC,
        };
    }

    #[test]
    fn compose() {
        use TailPosition::*;
        use std::ops::ControlFlow;

        #[inline(always)]
        fn unfoldify(input: ControlFlow<(), TailPosition>) -> TailPosition {
            match input {
                ControlFlow::Continue(x) => x,
                ControlFlow::Break(_) => CC,
            }
        }

        macro cases(
            $(($tail:ident, $head_move:ident) => ($res_tail:ident, $tail_move:ident),)*
        ) {
            let mut tail;
            $(
                tail = $tail;
                let res = unfoldify(TailPosition::compose($head_move, &mut tail));
                if !matches!(res, $tail_move)
                || !matches!(tail, $res_tail) {
                    panic!(concat!(
                        "\nAssertion failed: (tail:",
                            stringify!($tail),
                        " head_move:",
                            stringify!($head_move),
                        ").\nExpected: (tail:",
                            stringify!($res_tail),
                        " moved:",
                            stringify!($tail_move),
                        ")\nGot: (tail:{:?} moved:{:?})"), tail, res);
                }
            )*
        }

        cases! {
            // Center
            (CC, CC) => (CC, CC),
            (CC, CL) => (CR, CC),
            (CC, CR) => (CL, CC),
            (CC, TC) => (BC, CC),
            (CC, BC) => (TC, CC),

            // Side
            (CL, CC) => (CL, CC),
            (CL, CL) => (CC, CC),
            (CL, CR) => (CL, CR),
            (CL, TC) => (BL, CC),
            (CL, BC) => (TL, CC),

            // Corner
            (TR, CC) => (TR, CC),
            (TR, CL) => (CR, BL),
            (TR, CR) => (TC, CC),
            (TR, TC) => (CR, CC),
            (TR, BC) => (TC, BL),
        };
    }
}
