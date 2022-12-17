#[repr(u8)]
#[derive(Clone, Copy, Debug)]
enum Move {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

impl Move {
    pub fn outcome(self, against: Self) -> usize {
        ((u8::from(self) as isize - u8::from(against) as isize + 4) % 3 * 3) as usize
    }
}

impl From<u8> for Move {
    #[inline(always)]
    fn from(i: u8) -> Move {
        match i {
            0..=2 => unsafe { std::mem::transmute(i) },
            _ => panic!("Invalid move"),
        }
    }
}

impl From<Move> for u8 {
    #[inline(always)]
    fn from(m: Move) -> Self {
        m as u8
    }
}

// Task input is: ([ABC] ' ' [XYZ] '\n')+
pub fn task(input: &str) -> (usize, usize) {
    let input = input.as_bytes();
    assert_eq!(input.len() & 3, 0, "Input length is not a multiple of 4");
    ({
        // Part 1
        let mut total: usize = 0;
        for idx in 0..(input.len() >> 2) {
            let challange = Move::from(input[idx << 2] - b'A');
            let response = Move::from(input[(idx << 2) + 2] - b'X');
            total += u8::from(response) as usize + 1 + response.outcome(challange);
        }
        total
    },
    {
        // Part 2
        let mut total: usize = 0;
        for idx in 0..(input.len() >> 2) {
            let challange = input[idx << 2] - b'A';
            let outcome = (input[(idx << 2) + 2] - b'X') as usize;
            total += ((challange as usize + outcome + 2) % 3) as usize + 1 + outcome * 3;
        }
        total
    })
}
