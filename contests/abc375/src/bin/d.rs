use proconio::{input, marker::Chars};
fn main() {
    input!{
        s: Chars,
    }

    let mut cnt = vec![0u64; 26];
    let mut adder = vec![0u64; 26];
    let mut last = vec![1usize << 60; 26];


    fn to_usize(c: char) -> usize {
        c as usize - 'A' as usize
    }
    let mut ans = 0;
    for (i, &ci) in s.iter().enumerate() {
        for cj in 'A'..='Z' {
            if ci == cj {
                ans += adder[to_usize(ci)];
            }

            adder[to_usize(cj)] += cnt[to_usize(cj)];
        }


        cnt[to_usize(ci)] += 1;
    }

    println!("{ans}");
}
