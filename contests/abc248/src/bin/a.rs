use proconio::{input, marker::Chars};
fn main() {
    input!{
        mut s: Chars
    }
    s.sort_unstable();
    
    for (i, &c) in s.iter().enumerate() {
        if c as u8 != b'0' + i as u8 {
            println!("{}", (b'0' + i as u8) as char);
            return;
        }
    }
    
    println!("9");
}

