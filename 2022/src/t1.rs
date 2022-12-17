use crate::lisp;

fn task_lisp(input: &str) -> u32 {
    lisp! {
        (let (
                lines (line-iter input)
                sum (cell 0)
                current (cell 0)
            ) (for-each (line lines)
                (if (empty? line)
                    (if (< (deref sum) (deref current))
                        (reset! sum (deref current)))
                    (swap! current + (parse (u32) line))
                )
            )
            (deref (if (< (deref sum) (deref current))
                current sum))
        )
    }
}

fn task1(input: &str) -> u32 {
    let mut max = 0;
    let mut current = 0;
    for line in input.lines() {
        if line.is_empty() {
            if current > max {
                max = current;
            }
            current = 0;
        } else {
            let num: u32 = line.parse().unwrap();
            current += num;
        }
    }
    if current > max {
        current
    } else {
        max
    }
}

fn task2(input: &str) -> u32 {
    let mut top = [0; 3];
    let mut current = 0;
    for line in input.lines() {
        if line.is_empty() {
            let mut i = 0;
            while i != 3 {
                if top[i] < current {
                    break;
                }
                i += 1;
            }
            if i != 3 {
                for j in i..2 {
                    top[j+1] = top[j];
                }
                top[i] = current;
            }
            current = 0;
        } else {
            let num: u32 = line.parse().unwrap();
            current += num;
        }
    }
    {
        let mut i = 0;
        while i != 3 {
            if top[i] < current {
                break;
            }
            i += 1;
        }
        if i != 3 {
            for j in i..2 {
                top[j+1] = top[j];
            }
            top[i] = current;
        }
    }
    top[0] + top[1] + top[2]
}

pub fn task(input: &str) -> (u32,) {
    (if false {
        task_lisp(input)
    } else if true {
        task1(input)
    } else {
        task2(input)
    },)
}
