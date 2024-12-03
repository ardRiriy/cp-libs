use proconio::input;

fn main() {
    input!{
        n: u64,
        t: u64,
        a: u64,
    }

    let left = n - (t + a);
    if t + left < a || a + left < t {
        println!("Yes");
    } else {
        println!("No");
    }
}
