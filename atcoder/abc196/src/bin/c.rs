use proconio::input;
fn main() {
    input!{
        n: usize,
    }

    let mut ans = 0u64;
    for i in 1..1e6 as usize {
        let s = format!("{i}{i}");
        if s.parse::<usize>().unwrap() <= n {
            ans += 1;
        } else {
            break;
        }
    }
    println!("{ans}");
}
