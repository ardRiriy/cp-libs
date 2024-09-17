use proconio::{input, marker::{Chars, Usize1}};
fn main() {
    input!{
        s: Chars,
        q: usize,
    }

    for _ in 0..q {
        input! {
            t: usize,
            k: Usize1,
        }
        println!("{}", r(&s, t, k));
    }
}

fn r(s: &[char], t: usize, k: usize) -> char {
    if t == 0 {
        return s[k];
    } else if k == 0 {
        let u = s[0] as usize - 'A' as usize;
        return ('A' as u8 + ((u + t) % 3) as u8) as char;
    }
    let ret = r(s, t-1, if k % 2 == 0 { k / 2 } else { (k - 1) / 2}) as u8 - 'A' as u8;
    return ((ret + if k % 2 == 0 { 1 } else { 2 }) % 3 + 'A' as u8) as char;
}
