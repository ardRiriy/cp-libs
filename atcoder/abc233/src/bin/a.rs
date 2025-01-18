use proconio::input;
fn main() {
    input!{
        x: i32,
        y: i32,
    }

    let ans = (y - x).max(0) / 10 + if (y - x).max(0) % 10 == 0 {
        0
    } else {
        1
    };
    println!("{ans}");
}
