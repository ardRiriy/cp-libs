use proconio::input;

fn main() {
    input!{
        n: usize,
        a: [[u64; n]; 2],
    }

    let mut mx = vec![vec![0; n]; 2];
    mx[0][0] = a[0][0];
    for i in 0..2 {
        for j in 0..n {
            if j+1 < n {
                mx[i][j+1] = mx[i][j+1].max(mx[i][j] + a[i][j+1]);
            }
            if i+1 < 2 {
                mx[i+1][j] = mx[i+1][j].max(mx[i][j] + a[i+1][j]);
            }
        }
    }
    println!("{}", mx[1][n-1]);
}

