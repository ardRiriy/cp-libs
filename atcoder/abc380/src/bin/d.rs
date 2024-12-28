use itertools::Itertools;
use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn inv(c: char) -> char {
    if b'a' <= c as u8 {
        (b'A' + (c as u8 - b'a')) as char
    } else {
        (b'a' + (c as u8 - b'A')) as char
    }
}

fn dfs(s: &Vec<char>, q: usize, len: usize) -> char {
    if len == s.len() {
        return s[q];
    }

    return if q < len / 2 {
        dfs(s, q, len / 2)
    } else {
        inv(dfs(s, q - len / 2, len / 2))
    };
}

fn main() {
    input! {
        s: Chars,
        q: usize,
        k: [Usize1; q],
    }

    let mut ans = vec![];
    for &x in k.iter() {
        let mut len = s.len();
        while len <= x {
            len *= 2;
        }
        ans.push(dfs(&s, x, len));
    }
    println!("{}", ans.iter().join(" "));
}
