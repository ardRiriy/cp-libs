use proconio::{input};
fn main() {
    input!{
        p: [(i32, i32); 4],
    }

    // What a stupid math problem...
    for i in 0..4 {
        let ba = (p[(i + 1) % 4].0 - p[i].0, p[(i + 1) % 4].1 - p[i].1);
        let bc = (p[(i + 2) % 4].0 - p[(i + 1) % 4].0, p[(i + 2) % 4].1 - p[(i + 1) % 4].1);
        let product = ba.0 * bc.1 - ba.1 * bc.0;
        if product < 0 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
