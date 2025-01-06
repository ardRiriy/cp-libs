use std::{collections::BTreeSet, iter::repeat};

#[allow(unused_imports)]
use cps::debug::*;
use cps::multiset::MultiSet;
use itertools::Itertools;
use proconio::input;

fn main() {
    input!{
        _n: usize,
        q: usize,
    }

    static N: usize = 2e5 as usize + 2;
    let mut boxset = vec![MultiSet::new(); N]; 
    let mut cardset = vec![BTreeSet::new(); N];

    for _ in 0..q {
        input! {
            t: u8,
        }
        if t == 1 {
            input! {
                i: usize,
                j: usize,
            }
            boxset[j].add(i, 1);
            cardset[i].insert(j);
        } else if t == 2 {
            input! {
                j: usize,
            }
            for (card, size) in boxset[j].iter() {
                print!("{}", repeat(card).take(*size).join(" "));
                print!(" ");
            }
            println!();
        } else {
            input! {
                i: usize,
            }
            println!("{}", cardset[i].iter().join(" "));
        }
    }
}

