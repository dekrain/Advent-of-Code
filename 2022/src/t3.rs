fn settify(s: &str) -> u64 {
    let mut set = 0u64;
    for &c in s.as_bytes() {
        let pos = match c {
            b'a'..=b'z' => c - b'a' + 1,
            b'A'..=b'Z' => c - b'A' + 27,
            _ => panic!("Bad item"),
        };
        set |= 1u64 << pos;
    }
    set
}

pub fn task(input: &str) {
    {
        // Part 1
        let mut total = 0usize;
        for line in input.lines() {
            debug_assert_eq!(line.len() & 1, 0);
            let (left, right) = line.split_at(line.len() >> 1);
            let item = settify(left) & settify(right);
            let item = item.trailing_zeros();
            total += item as usize;
        }
        println!("{total}");
    }
    {
        // Part 2
        let mut total = 0usize;
        for lines in input.lines().array_chunks::<3>() {
            let item = lines.map(settify).into_iter().reduce(std::ops::BitAnd::bitand).unwrap();
            let item = item.trailing_zeros();
            total += item as usize;
        }
        println!("{total}");
    }
}
