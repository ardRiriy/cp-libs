use cps::cumulative_sum::CumulativeSum;
use proconio::{input, marker::Chars};

fn to_usize(c: char) -> usize {
    c as usize - 'A' as usize
}

fn solve1(s: &[char]) {
    // 左から順に見る
    let mut cnt = vec![0u64; 26];
    let mut adder = vec![0u64; 26];

    let mut ans = 0;
    for &ci in s.iter() {
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

fn solve2(s: &[char]) {
    // 中央に着目
    let mut csums :Vec<CumulativeSum<u64>> = vec![];

    for c in 'A'..='Z' {
        let v = s.iter()
            .map(|si| if *si == c { 1 } else { 0 })
            .collect();
        csums.push(CumulativeSum::new(&v));
    }


    let mut ans = 0;
    for i in 0..s.len() {
        if i != 0 && i != s.len() {
            for c in 'A'..='Z' {
                let l = csums[to_usize(c)].get(..i);
                let r = csums[to_usize(c)].get(i+1..);
                ans += l * r;
            }
        }
    }
    println!("{ans}");
}

fn main() {
    input!{
        s: Chars,
    }

    // solve1(&s);
    solve2(&s);
}
