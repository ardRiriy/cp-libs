use std::mem::swap;

use library::utils::input::Input;

fn solve(input: &mut Input) {
    let (n, k) = input.pair::<usize,u64>();
    let mut a = input.vector::<u64>(n);
    a.sort();
    let mut ans = a.iter().sum::<u64>();
    let mut m1 = a[0];
    let mut m2 = a[1];
    let mut left = k;
    while left > 0 {
        let s = (m1 + m2 + 1) / 2;
        if m1 == s || m2 == s { 
            ans += left * s;
            break; 
        }
        ans += s;
        m2 = s;
        if m1 > m2 {
            swap(&mut m1, &mut m2);
        }
        left -= 1;
    }
    println!("{}", ans);
}

fn main() {
    let mut input = Input::new();
    let t = input.next::<usize>();
    for _ in 0..t {
        solve(&mut input);
    }
}
