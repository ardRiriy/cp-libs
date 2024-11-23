use itertools::Itertools;
use proconio::input;
fn main() {
    input!{
        _n: usize,
        s: String,
    }

    let sv = s.split("/").collect_vec();

    if sv.len() != 2 || s.chars().filter(|c| *c == '/').count() != 1 {
        println!("No");
        return;
    }

    let a = sv[0].to_string();
    let b = sv[1].to_string();

    if a.chars().map(|c| if c == '1' { '2' } else { '@' }).eq(b.chars()) {
        println!("Yes");
    } else {
        println!("No");
    }
}

