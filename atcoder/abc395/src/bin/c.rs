use std::collections::BTreeMap;

use cps::chlibs::ChLibs; 
use cps::consts::INF;
use proconio::input;

fn main() {
    input!{
        n: usize,
        a: [u64; n],
    }
    let mut map = BTreeMap::new();

    let mut ans = INF as usize;
    for (i, &ai) in a.iter().enumerate() {
        if let Some(&j) = map.get(&ai) {
            ans.chmin(i-j+1);
        }
        map.insert(ai, i);
    }
    println!("{}", if ans == INF as usize{
        "-1".to_string()
    } else {
        ans.to_string()
    });
}


