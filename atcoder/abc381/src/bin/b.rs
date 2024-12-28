use itertools::Itertools;
use proconio::{input, marker::Chars};
fn main() {
    input!{
        s: Chars,
    }

    let v = s.iter().dedup_by_with_count(|a, b| a == b).collect_vec();

    let v2 = s.iter().fold(vec![0; 26], |mut v, &c| {
        v[c as usize - 'a' as usize] += 1;
        v
    });

    if v2.iter().all(|x| *x == 0 || *x == 2) && 
        v.iter().all(|(cnt, _)| *cnt == 2) {
            println!("Yes");
        } else {
            println!("No");
        }
}

