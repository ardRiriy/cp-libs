use cps::multiset::MultiSet;
use proconio::input;

fn main() {
    input!{
        n: usize,
        a: [u64; n],
    }
    let mut map_p = MultiSet::new();
    let mut map_a = MultiSet::new();

    for &ai in a.iter() {
        map_p.add(ai, 1);
    }

    let mut ans = map_p.len();

    for &ai in a.iter() {
        map_p.remove(ai, 1);
        map_a.add(ai, 1);
        let val = map_a.len() + map_p.len();
        if ans < val {
            ans = val;
        }
    }

    println!("{}", ans);

}

