use itertools::Itertools;
#[allow(unused_imports)]
use proconio::input;

fn main() {
    input!{
        n: usize,
        m: usize,
    }
    let mut ans = Vec::new();
    let mut used = vec![false; n];
    let mut cur = 0;
    for i in 1..=m {
        loop {
            if used[cur] {
                cur += 1;
            } else {    
                ans.push((cur, cur+i));
                dbg!(cur, cur+i);
                used[cur] = true;
                used[cur+i] = true;
                break;
            }
        }
    }
    println!("{}", ans.iter().map(|(x,y)| format!("{} {}", x+1,y+1)).join("\n"));
}

