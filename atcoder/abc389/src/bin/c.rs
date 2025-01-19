#[allow(unused_imports)]
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        q: usize,
    }

    let mut mergin = 0;
    let mut v = vec![];
    let mut len = vec![];
    let mut sum = 0;
    let mut i = 0;
    for _ in 0..q {
        input! {
            t: u8
        }
        if t == 1 {
            input! {
                l: u64,
            }
            v.push(sum);
            len.push(l);
            sum += l;
        } else if t == 2 {
            mergin += len[i];
            i += 1;
        } else {
            input! {
                k: Usize1,
            }
            println!("{}", v[i+k]-mergin);
        }
    }
}

