use proconio::input;
fn main() {
    input!{
        n: usize,
        m: usize,
    }
    let inf = 1usize << 62;
    let mut ans = inf;
    let mut a = 1usize;
    
    while a <= n && a * a <= m * 10{
        let b = m / a + if m % a == 0 { 0 } else { 1 };
        let x = a * b;
        if b <= n && ans > x && x >= m {
            ans = x;
        }
        a += 1;
    }
    
    println!("{}", if ans == inf { usize::max_value() as i64 } else { ans as i64 });
}

