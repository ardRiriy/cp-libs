use itertools::Itertools;
use proconio::input;

fn main() {
    input!{
        n: usize,
        k: usize,
    }

    let mut d = (n-1)*(n-2)/2; // star (upper limit)
    if d < k {
        println!("-1");
        return;
    }

    let mut ans = vec![];
    for i in 2..=n {
        ans.push((1, i));
    }

    for i in 2..=n {
        for j in i+1..=n {
            if d == k {
                break;
            }
            ans.push((i, j)); 
            d-=1;
        }
    }

    println!("{}", ans.len());
    println!("{}", ans.iter().map(|v| format!("{} {}", v.0, v.1)).join("\n"));
}

