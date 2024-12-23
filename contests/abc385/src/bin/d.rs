use std::collections::{BTreeMap, BTreeSet};

use cps::consts::DC;
#[allow(unused_imports)]
use cps::debug::*;
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        n: usize,
        m: usize,
        si: i64,
        sj: i64,
        home: [(i64, i64); n],
        moves: [(char, i64); m],
    }

    let mut home_i :BTreeMap<i64, BTreeSet<i64>>= BTreeMap::new();
    let mut home_j :BTreeMap<i64, BTreeSet<i64>>= BTreeMap::new();

    for (i, j) in home {
        if let Some(set) = home_i.get_mut(&i) {
            set.insert(j);
        } else {
            let mut set = BTreeSet::new();
            set.insert(j);
            home_i.insert(i, set);
        }

        if let Some(set) = home_j.get_mut(&j) {
            set.insert(i);
        } else {
            let mut set = BTreeSet::new();
            set.insert(i);
            home_j.insert(j, set);
        }
    }

    let mut i = si;
    let mut j = sj;
    for (c, d) in moves {

    }
}

