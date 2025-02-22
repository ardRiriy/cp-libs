use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input!{
        mut s: Chars,
    }
    let n = s.len();
    for i in 0..n-1 {
        if s[i] == 'W' && s[i+1] == 'A' {
            // 戻る
            let mut last_found = i;

            for j in (0..=i).rev() {
                if s[j] != 'W' { 
                    break; 
                }
                last_found = j;
                s[j] = 'C';
            }
            s[last_found] = 'A';
            s[i+1] = 'C';
        }
    }
    println!("{}", s.iter().join(""));
}

