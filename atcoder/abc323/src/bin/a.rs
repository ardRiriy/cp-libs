use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    println!(
        "{}",
        if s.iter().enumerate().all(|(i, &c)| i % 2 == 0 || c == '0') {
            "Yes"
        } else {
            "No"
        }
    );
}
