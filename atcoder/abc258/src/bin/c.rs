use proconio::{input, marker::Chars};
fn main() {
    input!{
        n: usize,
        q: usize,
        s: Chars,
    }

    let mut idx = 0;

    for _ in 0..q {
        input! {
            t: u8,
            x: usize,

        }

        match t {
            1 => {
                idx = (idx + n - x) % n;
            }
            2 => {
                println!("{}", s[(idx + (x - 1)) % n]);
            }
            _ => unreachable!(),
        }
    }
}

