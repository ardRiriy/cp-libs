use proconio::input;

fn main() {
    input! {
        n: usize,
        a: u64,
        t: [u64; n],
    }

    let mut current = 0;
    for &x in &t {
        if current < x {
            current = x + a;
        } else {
            current += a;
        }
      println!("{}", current);
    }
}
