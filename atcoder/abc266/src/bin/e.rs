use proconio::input;
fn main() {
    input!{
        n: usize,
    }
    println!("{}", dfs(n));
}

fn dfs(turn: usize) -> f64 {
    if turn == 0 {
        return 0.;
    }

    let mut res = 0.;
    let rec = dfs(turn-1);

    for i in 1..=6 {
        let p =  i as f64 / 6.;
        if rec > i as f64 {
            res += rec / 6.;
        } else {
            res += p;
        }
    }
    res
}
