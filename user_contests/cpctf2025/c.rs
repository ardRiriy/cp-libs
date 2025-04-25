use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input!{
        _n: i32,
        s: Chars,
    }

    let ans = s.iter()
        .copied()
        .combinations(5)
        .map(|v| {
            for i in 0..5 {
                for j in i+1..5 {
                    if i == 0 && j == 2 {
                        if v[i] != v[j] {
                            return 0;
                        }
                    } else {
                        if v[i] == v[j] {
                            return 0;
                        } 
                    }
                }
            }
            1
        })
        .sum::<i32>();
    println!("{}", ans);
}