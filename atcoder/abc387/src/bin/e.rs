use std::iter::repeat;

use itertools::Itertools;
use proconio::input;

fn digit_sum(s: &String) -> u32 {
    s.chars().map(|c| c as u32 - '0' as u32).sum()
}

fn small_n(n: &String) {
    let n = n.parse::<usize>().unwrap();
    for a in n..2*n {
        if a % digit_sum(&a.to_string()) as usize == 0 && (a+1) % digit_sum(&(a+1).to_string()) as usize == 0{
            println!("{}", a);
            return;
        }
    }
    println!("-1");
}

fn big_n(n: &String) {
    let d = n.chars().next().unwrap();
    let length = n.len();
    let z = repeat(0).take(length-4).join("");
    let ans = match d {
        '1' => {
            let d2 = n.chars().nth(1).unwrap();
            let f = format!("{}{}", d, d2).parse::<u64>().unwrap() + 1;
            let z = repeat(0).take(length-5).join("");

            // let s = f.to_string().chars().map(|c| c as u64 - '0' as u64).sum::<u64>();

            match f {
                11 => format!("11{}240", z),
                12 => format!("12{}104", z),
                13 => format!("13{}040", z),
                14 => format!("14{}120", z),
                15 => format!("15{}200", z),
                _ => format!("20{}024", z),
            }
        },
        '2' => format!("3{}104", z),
        '3' => format!("4{}040", z),
        '4' => format!("5{}120", z),
        '5' => format!("6{}200", z),
        '6' | '7' | '8' | '9' => format!("11{}240", z),
        _ => unreachable!()
    };
    println!("{}", ans);
}


fn main() {
    input!{
        n: String,
    }

    if n.len() < 7 {
        small_n(&n);
    } else {
        big_n(&n);
    }
}

