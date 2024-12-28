use proconio::input;
fn main() {
    input!{
        k: u64,
    }

    let h = 21 + k / 60;
    let m = k % 60;

    println!("{:2}:{:0>2}", h, m);
}

