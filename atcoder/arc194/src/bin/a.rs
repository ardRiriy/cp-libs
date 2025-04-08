use proconio::input;

fn main() {
    input!{
        n: usize,
        a: [i64; n],
    }

    let mut memo = vec![-(1<<60), 0];
    for i in 0..n {
        memo[i%2] = memo[i%2].max(a[i] + memo[1-i%2]);
    }

    println!("{}", memo[1-n%2]);
}
