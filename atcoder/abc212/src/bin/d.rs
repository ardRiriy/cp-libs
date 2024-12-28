use std::collections::BTreeMap;

use proconio::input;
fn main() {
    input!{
        q: usize,
    }
    let mut map = BTreeMap::new();
    let mut e = 0;
    for _ in 0..q {
        input! {
            t: u8,
        }

        match t {
            1 => {
                input! {
                    x: i64,
                }
                *map.entry(x+e).or_insert(0u64) += 1;
            },
            2 => {
                input! {
                    x: i64,
                }
                e -= x;
            },
            3 => {
                let (&k, &v) = map.iter().min().unwrap();
                println!("{}", k - e);
                if v == 1 {
                    map.remove(&k);
                } else {
                    map.insert(k, v - 1);
                }
            },
            _ => { }
        }
    }
}
