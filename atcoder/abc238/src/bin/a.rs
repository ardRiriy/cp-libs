use proconio::input;
fn main() {
    input!{
        n: u64,
    }

    if n < 15 {
        let k = (0..n).fold(1u64, |acc, _| acc * 2);
        if k > n * n {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        println!("Yes");
    }
}
