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
#[derive(Clone, Copy)]
enum TailPosition {
    TL, TC, TR,
    CL, CC, CR,
    BL, BC, BR,
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
}

type Pos = (i16, i16);

pub fn task(input: &str) -> (usize, usize) {
    let mut p1 = HashSet::new();
    //let mut p2 = HashSet::new();

    let mut head: Pos = (0, 0);
    let mut tail = TailPosition::CC;
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
        for _ in 0..count {
            match dir {
                Direction::Left => head.0 -= 1,
                Direction::Right => head.0 += 1,
                Direction::Up => head.1 -= 1,
                Direction::Down => head.1 += 1,
            }
            tail = tail.moved(dir);
            p1.insert(tail.offset(head));
        }
    }

    (p1.len(), 0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let res = task("R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2\n");
        assert_eq!(res.0, 13);
        assert_eq!(res.1, 1);

        let res = task("R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20\n");
        assert_eq!(res.1, 36);
    }
}
