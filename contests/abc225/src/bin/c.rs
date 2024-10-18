use proconio::{input, marker::Usize1};
fn main() {
    input!{
        n: usize,
        m: usize,
        b: [[Usize1; m]; n],
    }

    let si = b[0][0] / 7;
    let sj = b[0][0] % 7;

    for i in 0..n {
        for j in 0..m {
            if sj + j >= 7 || b[i][j] != (si + i) * 7 + (sj + j) {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}

