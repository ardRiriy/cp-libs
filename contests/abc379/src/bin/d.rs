use std::collections::BinaryHeap;

use proconio::input;
fn main() {
    input!{
        q: usize,
    }
    
    let mut day = 0i64;
    let mut heap = BinaryHeap::new();
    for _ in 0..q {
        input! {t: u8}
        match t {
            1 => {
                heap.push(-day);
            },
            2 => {
                input! {t: i64}
                day += t;
            },
            3 => {
                let mut ans = 0;
                input! {
                    h: i64
                }
                while let Some(x) = heap.pop() {
                    if x + day >= h {
                        ans += 1;
                    } else {
                        heap.push(x);
                        break;
                    }
                }
                println!("{ans}");
            },
            _=> unreachable!()
        }
    }
}

