use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        x: usize,
        s: Chars,
    }

    let mut stk = vec![];

    for &c in &s {
        if let Some(prev) = stk.last() {
            if (*prev == 'L' || *prev == 'R') && c == 'U' {
                stk.pop();
            } else {
                stk.push(c);
            }
        } else {
            stk.push(c);
        }
    }
    dbg!(&stk);
    let mut current = x as u128;

    for &c in &stk {
        match c {
            'U' => {
                current = current / 2;
            }
            'L' => {
                current = current * 2;
            }
            'R' => {
                current = current * 2 + 1;
            },
            _ => { unreachable!(); }
        }
    }

    println!("{}", current);
}
