use std::convert::TryInto;

use cps::consts::INF;
use cps::multiset::MultiSet;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    const N: usize = 3e5 as usize + 1;
    let mut v = vec![false; N];
    let mut mset = MultiSet::new();
    for ai in a {
        if ai >= N {
            mset.add(INF, 1);
        } else if v[ai] {
            mset.add(INF, 1);
        } else {
            v[ai] = true;
            mset.add(ai.try_into().unwrap(), 1);
        }
    }

    let mut ans = 0;
    for i in 1.. {
        if let Some(val) = mset.get(i) {
            assert_eq!(val, 1);
            mset.remove(i, 1);
        } else {
            if let Some((&k, &v)) = mset.max_key() {
                if v >= 2 {
                    mset.remove(k, 2);
                } else {
                    assert_eq!(v, 1);
                    mset.remove(k, 1);
                    if let Some((&l, _)) = mset.max_key() {
                        mset.remove(l, 1);
                    } else {
                        break;
                    }
                }
            } else {
                break;
            }
        }
        ans = i;
    }
    println!("{}", ans);
}