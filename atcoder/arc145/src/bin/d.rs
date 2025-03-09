use itertools::Itertools;
use num_traits::Float;
use proconio::input;

fn main() {
    input!{
        n: i64,
        m: i64,
    }

    let mut s = vec![];
    {
        let mut i = 0;
        while s.len() < n as usize {
            let mut k = 3*i;
            let mut flag = true;
            while k != 0 {
                if k % 3 == 2 {
                    flag = false;
                    break;
                } else {
                    k /= 3;
                }
            }
            if flag {
                s.push(3*i);
            }
            i += 1;
        }
    }

    let ssum = s.iter().sum::<i64>();
    let diff = m - ssum;
    let val = if diff > 0 {
        diff % n
    } else {
        (n - (-diff) % n) % n
    };
    for i in 0..val {
        s[i as usize] += 1;
    }
    let addr = (m - ssum - val) / n;

    let a = 0.1;
    let b = f64::nan();
    let c = a < b;


    println!("{}", s.iter().map(|i| *i + addr).join(" "));
}

