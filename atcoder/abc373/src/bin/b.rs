use proconio::{input, marker::Chars};
fn main() {
    input!{
        s: Chars,
    }

    let mut ans = 0;
    let mut current = s.iter().position(|c| *c == 'A').unwrap();
    for c in 'B'..='Z' {
        let pos = s.iter().position(|x| *x == c).unwrap();

        ans += current.abs_diff(pos);
        current = pos;
    }

    println!("{ans}");
}
