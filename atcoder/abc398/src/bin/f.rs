use ac_library::z_algorithm;
#[allow(unused_imports)]
use cps::debug::*;
use itertools::Itertools;
use proconio::input;

fn main() {
    input!{
        s: String,
    }    
    let t = s.chars().rev().collect::<String>();
    let ss = format!("{}#{}", t, s);
    let res = z_algorithm(&ss);
    let mut l = 0;
    for i in s.len()+1..ss.len() {
        if i + res[i] == ss.len() {
            l = res[i];
            break;
        }
    }
    let mut v = s.chars().collect_vec();
    let s_chars = s.chars().collect_vec();
    let ll = s_chars.len() - l;
    for i in (0..ll).rev() {
        v.push(s_chars[i]);
    }
    println!("{}", v.iter().join(""));
}

