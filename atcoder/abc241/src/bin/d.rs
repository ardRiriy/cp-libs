use std::collections::BTreeMap;

use proconio::{input};
fn main() {
    input!{
        q: usize,
    }

    let mut map = BTreeMap::new();

    for _ in 0..q {
        input! {
            t: u8,
            x: u64,
        }

        match t {
            1 => {
                *map.entry(x).or_insert(0) += 1;
            }
            2 => {
                input! {
                    k: u32
                }
                let mut cnt = 0;
                let mut x = x;
                while cnt < k {
                    match map.range(..=x).next_back() {
                        Some((&key, &val)) => {
                            cnt += val;
                            if cnt >= k {
                                println!("{}", key);
                            } else {
                                x = key - 1;
                            }
                        },
                        None => {
                            println!("-1");
                            break;
                        }
                    }
                }
            }
            3 => {
                input! {
                    k: u32
                }
                let mut cnt = 0;
                let mut x = x;
                while cnt < k {
                    match map.range(x..).next() {
                        Some((&key, &val)) => {
                            cnt += val;
                            if cnt >= k{
                                println!("{}", key);
                            } else {
                                x = key + 1;
                            }
                        },
                        None => {
                            println!("-1");
                            break;
                        }
                    }   
                }
            }
            _ => { panic!() }
        }
    }
}

