use proconio::{input, marker::{Chars, Usize1}};
fn main() {
    input!{
        n: usize,
        q: usize,
        mut s: Chars,
    }

    let mut ans = 0u64;
    'i: for i in 0..n-2 {
        for j in 0..3 {
            if s[i + j] != (b'A' + j as u8) as char {
                continue 'i;
            }
        }
        ans += 1;
    }


    for _ in 0..q {
        input! {
            x: Usize1,
            c: char,
        }

        let mut prev = 0;
        let start = if x >= 2 { x - 2 } else { 0 };

        'i: for i in start..=x {
            for j in 0..3 {
                if i + j >= n {
                    break 'i;
                }
                if s[i + j] != (b'A' + j as u8) as char {
                    continue 'i;
                }
            }
            prev += 1;
        }

        s[x] = c;
        let mut cur = 0;
        'i: for i in start..=x {
            for j in 0..3 {
                if i + j >= n {
                    break 'i;
                }
                if s[i + j] != (b'A' + j as u8) as char {
                    continue 'i;
                }
            }
            cur += 1;
        }

        ans = ans + cur - prev;
        println!("{}", ans);
    }

}
