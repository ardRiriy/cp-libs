use proconio::{input, marker::Chars};

fn main() {
    input!{
        n: Chars,
    }
    println!("{}", if n.iter().map(|c| (*c as u8 - b'0') as u64).sum::<u64>() % 9 == 0 {
        "Yes"
    } else {
        "No"
    });
}

