use std::str::FromStr;

pub fn task(input: &str) -> (usize, usize) {
    input.lines().fold((0, 0), |(mut p1, mut p2), line| {
        let [a, b, c, d] = line.split(&['-', ',']).next_chunk().unwrap().map(|x| u32::from_str(x).unwrap());
        // Check if a..=b contains c..=d
        if (a <= c && d <= b) ||
        // ...or if c..=d contains a..=b
        (c <= a && b <= d) {
            p1 += 1;
        }

        // Check if a..=b overlaps c..=d
        if a.max(c) <= b.min(d) {
            p2 += 1;
        }

        (p1, p2)
    })
}
