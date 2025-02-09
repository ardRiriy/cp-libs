#[allow(unused_imports)]
use cps::debug::*;
use itertools::Itertools;
use proconio::input;

fn main() {
    input!{
        n: usize,
        a: [i32; n],
    }
    if n % 4 == 0 || (n%4 != 2 && a.iter().any(|&i| i == 1)) {
        // 合ってる!
        println!("Yes");
        return;
    } if n%4 != 2 {
        println!("No");
        return;
    }

    let mut rle = a.iter().dedup_with_count().map(|(count, &value)| (count, value)).collect_vec();
    let mut m = rle.len();
    if m != 1 && rle[0].1 == rle[m-1].1 {
        rle[0].0 += rle[m-1].0;
        rle.pop();
        m -= 1;
    }

    'i: for i in 0..rle.len() {
        if rle[i].1 == 1 && rle[i].0 >= 2 {
            println!("Yes");
            return;
        }
        if rle[i].1 == 1 {
            continue;
        }
        if rle[i].0 % 2 == 0 && rle.len() >= 2 {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
