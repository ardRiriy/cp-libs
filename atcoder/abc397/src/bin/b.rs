use proconio::{input, marker::Chars};

fn main() {
    input!{
        s: Chars,
    }
    let mut v = vec![];
    let k = vec!['i', 'o'];
    for &c in s.iter() {
        if k[v.len()%2] == c {
            v.push(c);
        } else {
            v.push('@');
            v.push(c);
        }
    }
    if v.len() % 2 == 1 {
        v.push('@');
    }
    println!("{}", v.len() - s.len());
}

