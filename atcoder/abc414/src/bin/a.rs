use proconio::input;

fn main() {
    input!{
        n: usize,
        l: u64,
        r: u64,
        v: [(u64,u64); n],
    }
    let ans = v.iter()
        .filter(|&(x,y)| x <= &l && &r <= y)
        .count();
    println!("{}", ans);
}
