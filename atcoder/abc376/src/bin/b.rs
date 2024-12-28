use proconio::{input, marker::Usize1};
fn main() {
    input!{
        n: usize,
        q: usize,
    }

    let mut l = 0;
    let mut r = 1;
    let mut ans = 0;
    for _ in 0..q {
        input! {
            c: char,
            t: Usize1,
        }

        match c {
            'L' => {
                for i in 0..n {
                    if r == (l + i) % n {
                        break;
                    }
                    if (l + i) % n == t {
                        ans += i;
                        break;
                    }
                }
                for i in 0..n {
                    if r == (l + n - i) % n {
                        break;
                    }
                    if (l + n - i) % n == t {
                        ans += i;
                        break;
                    }
                }
                l = t;
            }
            'R' => {
                for i in 0..n {
                    if l == (r + i) % n {
                        break;
                    }
                    if (r + i) % n == t {
                        ans += i;
                        break;
                    }
                }
                for i in 0..n {
                    if l == (r + n - i) % n {
                        break;
                    }
                    if (r + n - i) % n == t {
                        ans += i;
                        break;
                    }
                }
                r = t;
            }
            _ => { }
        }
    }
    println!("{ans}");
}
