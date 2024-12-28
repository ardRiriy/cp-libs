#[allow(unused_imports)]
use cps::debug::*;
#[allow(unused_imports)]
use cps::lg;
use proconio::input;

fn main() {
    input!{
        n: usize,
    }
    let mut ans = 0;

    let mut l = 1;
    let mut r = 2;
    let mut cnt = 1;

    loop {
        let mut tl = l;
        let mut tr = r;
        loop {
            ans += tr.min(n+1) - tl;
            tr *= 10;
            tl *= 10;
            if tl > n {
                break;
            }
        }
        l += 10usize.pow(cnt as u32); 
        r += 10usize.pow(cnt as u32); 
        cnt += 1;
        if l > n {
            break;
        }
    }
    println!("{ans}");
}
