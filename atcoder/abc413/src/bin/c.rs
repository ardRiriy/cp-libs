use std::collections::VecDeque;

#[allow(unused_imports)]
use proconio::input;

fn main() {
    input!{
        q: usize,
    }

    let mut que = VecDeque::new();

    for _ in 0..q {
        input! {
            t: u8,
        }
        if t==1 {
            input! {
                c: u64,
                x: u64,
            }

            que.push_back((x,c));
        } else {
            input! {
                k: u64,
            }

            let mut left = k;
            let mut ans = 0;
            while let Some((val, cnt)) = que.pop_front() {
                if left < cnt {
                    let v = cnt - left;
                    if v != 0 {
                        que.push_front((val,v));
                    }
                    ans += left*val;
                    break;
                } else {
                    //let v = left-cnt;
                    left -= cnt;
                    ans +=val * cnt;
                    if left == 0 {
                        break;
                    }
                }
            }
            println!("{}", ans);
        }
    }
}

