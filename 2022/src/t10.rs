use crate::unreachable;
use std::io::Write;

pub fn task(input: &str) -> (i32, ) {
    struct State<'ol> {
        x: i32,
        rows: u32,
        cycles: u32,

        p1: i32,
        image_lock: std::io::StdoutLock<'ol>,
    }

    impl<'ol> State<'ol> {
        fn run(&mut self, input: &str) {
            for line in input.lines() {
                self.advance();
                debug_assert!(line.len() >= 4);
                match unsafe { line.get_unchecked(..4) } {
                    "noop" => {
                        debug_assert!(line.len() == 4);
                    }
                    "addx" => {
                        debug_assert!(line.as_bytes()[4] == b' ');
                        let delta: i32 = unsafe { line.get_unchecked(5..) }.parse().unwrap();
                        self.advance();
                        if self.cycles == 20 {
                            self.p1 += self.x * 20 * (1 | (self.rows<<2)) as i32;
                        }
                        self.x += delta;
                    }
                    _ => unreachable!(),
                }
            }
            self.image_lock.write(b"\n").unwrap();
            self.image_lock.flush().unwrap();
            
        }

        fn advance(&mut self) {
            // Check if x in cycles - 1 ..= cycles + 1
            self.image_lock.write(if (self.x - self.cycles as i32 + 1) as u32 <= 2 { b"#" } else { b"." }).unwrap();
            self.cycles += 1;
            if self.cycles == 20 {
                self.p1 += self.x * self.cycles as i32;
            } else if self.cycles == 40 {
                self.cycles = 0;
                self.rows += 1;
                self.image_lock.write(b"\n").unwrap();
            }
        }
    }

    let mut state = State {
        x: 1,
        rows: 0,
        cycles: 0,

        p1: 0,
        image_lock: std::io::stdout().lock(),
    };
    state.run(input);

    std::mem::drop(state.image_lock);

    #[cfg(debug_assertions)]
    println!("Cycles processed: {}", state.cycles + state.rows*40);
    (state.p1, )
}
