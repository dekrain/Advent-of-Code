fn different_chars(chunk: &[u8]) -> bool {
    // TODO: Make it better
    /*for i in 1..4 {
        if chunk[0] == chunk[i] {
            return false;
        }
    }
    for i in 2..4 {
        if chunk[1] == chunk[i] {
            return false;
        }
    }
    chunk[2] != chunk[3]*/
    // Here ya go, bud! (literally 5 minutes later, lol)
    let mut set = 0u32;
    for c in chunk {
        debug_assert!((b'a'..=b'z').contains(c));
        let c = 1u32 << (c - b'a');
        if set & c != 0 {
            return false;
        }
        set |= c;
    }
    true
}

pub fn task(input: &str) -> (usize, usize) {
    let input = input.trim_end();
    let mut index = 4;
    let p1 = loop {
        if different_chars(unsafe{input.as_bytes().get_unchecked(index - 4..index)}) {
            break index;
        }
        if index >= input.len() {
            panic!("Pattern not found");
        }
        index += 1;
    };
    index = index.max(14);
    let p2 = loop {
        if different_chars(unsafe{input.as_bytes().get_unchecked(index - 14..index)}) {
            break index;
        }
        if index > input.len() {
            panic!("Pattern not found");
        }
        index += 1;
    };
    (p1, p2)
}
