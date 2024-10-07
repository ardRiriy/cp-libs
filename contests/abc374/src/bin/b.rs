use proconio::{input, marker::Chars};
fn main() {
    input!{
        s: Chars,
        t: Chars,
    }

    if s == t {
        println!("0");
        return;
    }

    for i in 0..s.len().min(t.len()) {
        if s[i] != t[i] {
            println!("{}", i+1);
            return;
        }
    }
    println!("{}", s.len().min(t.len()) + 1);
}
