use proconio::{input};
fn main() {
    input!{
        s: [String; 12]
    }

    let ans = s.iter()
        .enumerate()
        .filter(|(idx, s)| idx + 1 == s.len())
        .count();

    println!("{ans}");
}
