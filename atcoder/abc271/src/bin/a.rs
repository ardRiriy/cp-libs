use std::convert::TryInto;

use proconio::input;

fn main() {
    input! {
        n: i32,
    }
    let a = n/16;
    let b = n%16;
    let ac :char = if a < 10 {
        (b'0' + TryInto::<u8>::try_into(a).unwrap()) as char
    } else {
        (b'A' + TryInto::<u8>::try_into(a-10).unwrap()) as char
    };
    let bc :char = if b < 10 {
        (b'0' + TryInto::<u8>::try_into(b).unwrap()) as char
    } else {
        (b'A' + TryInto::<u8>::try_into(b-10).unwrap()) as char
    };
    println!("{}{}", ac, bc);
}