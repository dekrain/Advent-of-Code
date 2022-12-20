use std::str::FromStr;

trait ConsumeStr {
    fn consume(&mut self, str: &str);
    fn consume_token<T: FromStr>(&mut self, sep: char) -> Result<T, T::Err>;
}

impl<'a> ConsumeStr for &'a str {
    fn consume(&mut self, str: &str) {
        if !self.starts_with(str) {
            panic!("String does not start with the given string");
        }
        *self = &self[str.len()..];
    }
    fn consume_token<T: FromStr>(&mut self, sep: char) -> Result<T, T::Err> {
        match self.split_once(sep).unwrap() {
            (token, rest) => {
                *self = rest;
                T::from_str(token)
            }
        }
    }
}

#[derive(Clone)]
struct Carry {
    stacks: Vec<Vec<u8>>,
}

impl Carry {
    pub fn from_description(description: &str) -> Self {
        // Assumption: input will always be divided into 4-char sections, per stack
        let line_len = description.find('\n').unwrap();
        debug_assert_eq!(line_len & 3, 3);
        let line_len = line_len + 1;
        let num_stacks = line_len >> 2;
        let mut carry = Carry { stacks: vec![Vec::new(); num_stacks] };
        // Lines must be the same length, sans the last newline seperator
        debug_assert_eq!((description.len() + 1) % line_len, 0);
        // Skip last line
        let mut index = description.len() - line_len + 1;
        let bytes = description.as_bytes();
        loop {
            for stack_idx in 0..num_stacks {
                if bytes[index + 4*stack_idx] != b' ' {
                    debug_assert_eq!(bytes[index + 4*stack_idx], b'[');
                    debug_assert_eq!(bytes[index + 4*stack_idx + 2], b']');
                    carry.stacks[stack_idx].push(bytes[index + 4*stack_idx + 1]);
                }
            }
            if index == 0 {
                break;
            }
            index -= line_len;
        }
        carry
    }

    pub fn carry(&mut self, from: u32, to: u32) {
        let item = self.stacks[from as usize].pop().expect("From stack is empty");
        self.stacks[to as usize].push(item);
    }

    pub fn carry_n(&mut self, from: u32, to: u32, count: u32) {
        let from = from as usize;
        let to = to as usize;
        let count = count as usize;
        assert_ne!(from, to, "Stack indices must not be equal");
        let (from, to) = unsafe {
            let ptr = self.stacks.as_mut_ptr();
            (
                &mut *ptr.add(from),
                &mut *ptr.add(to),
            )
        };
        assert!(from.len() >= count, "From stack is drained");
        let items = from.drain(from.len()-count..);
        to.extend(items);
    }

    pub fn render_top(&self) -> String {
        String::from_utf8(self.stacks.iter().map(|stack| *stack.last().expect("Stack is empty")).collect()).unwrap()
    }
}

impl std::fmt::Debug for Carry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        struct StackFmt<'a>(&'a Vec<u8>);
        impl<'a> std::fmt::Debug for StackFmt<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut l = f.debug_list();
                for item in self.0 {
                    l.entry(&(*item as char));
                }
                l.finish()
            }
        }
        let mut l = f.debug_list();
        for stack in &self.stacks {
            l.entry(&StackFmt(stack));
        }
        l.finish()
    }
}

pub fn task(input: &str) -> (String, String) {
    let (carry, proc) = input.split_once("\n\n").unwrap();
    let mut carry1 = Carry::from_description(carry);
    let mut carry2 = carry1.clone();
    //dbg!(&carry);
    for mut action in proc.lines() {
        action.consume("move ");
        let count: u32 = action.consume_token(' ').unwrap();
        action.consume("from ");
        let from: u32 = action.consume_token(' ').unwrap();
        action.consume("to ");
        let to = u32::from_str(action).unwrap();

        //println!("move {count} from {from} to {to}");

        let from = from - 1;
        let to = to - 1;
        // Part 1
        for _ in 0..count {
            carry1.carry(from, to);
        }

        // Part 2
        carry2.carry_n(from, to, count);
    }
    (carry1.render_top(), carry2.render_top())
}
