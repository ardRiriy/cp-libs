use cps::veclibs::VecLibs;
use proconio::input;

fn main() {
    input!{
        n: usize,
        a: [u64; n],
    }

    let mut v = vec![];

    for &ai in a.iter().rev() {
        if v.is_empty() || *v.last().unwrap() <= ai {
            v.push(ai);
            continue;
        }

        let i = v.lower_bound(ai+1);
        v[i] = ai;
    }
    println!("{}", v.len());
}
