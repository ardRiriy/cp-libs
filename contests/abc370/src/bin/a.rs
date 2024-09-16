use proconio::{input};
fn main() {
    input!{
        a: (i32, i32)
    }

    match a {
        (1, 0) => { println!("Yes"); },
        (0, 1) => { println!("No"); },
        _ => { println!("Invalid"); }
    }
}
