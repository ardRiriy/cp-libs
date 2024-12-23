use std::collections::BTreeSet;

use itertools::iproduct;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        m: usize,
        akari: [(Usize1, Usize1); n],
        block: [(Usize1, Usize1); m],
    }

    let mut akari_row = vec![BTreeSet::new(); h];
    let mut akari_col = vec![BTreeSet::new(); w];
    for &(i, j) in &akari {
        akari_row[i].insert(j);
        akari_col[j].insert(i);
    }

    let mut blcok_row = vec![BTreeSet::new(); h];
    let mut block_col = vec![BTreeSet::new(); w];
    for &(i, j) in &block {
        blcok_row[i].insert(j);
        block_col[j].insert(i);
    }

    let mut ans = 0i64;
    let blocks = block.iter().copied().collect::<BTreeSet<_>>();

    'i_j: for (i, j) in iproduct!(0..h, 0..w) {
        if blocks.contains(&(i, j)) {
            continue;
        }

        if let Some(&akari_j) = akari_row[i].range(0..=j).last() {
            if let Some(&block_j) = blcok_row[i].range(0..=j).last() {
                if akari_j > block_j {
                    ans += 1;
                    continue 'i_j;
                }
            } else {
                ans += 1;
                continue 'i_j;
            }
        }

        if let Some(&akari_j) = akari_row[i].range(j..).next() {
            if let Some(&block_j) = blcok_row[i].range(j..).next() {
                if akari_j < block_j {
                    ans += 1;
                    continue 'i_j;
                }
            } else {
                ans += 1;
                continue 'i_j;
            }
        }

        if let Some(&akari_i) = akari_col[j].range(0..=i).last() {
            if let Some(&block_i) = block_col[j].range(0..=i).last() {
                if akari_i > block_i {
                    ans += 1;
                    continue 'i_j;
                }
            } else {
                ans += 1;
                continue 'i_j;
            }
        }

        if let Some(&akari_i) = akari_col[j].range(i..).next() {
            if let Some(&block_i) = block_col[j].range(i..).next() {
                if akari_i < block_i {
                    ans += 1;
                    continue 'i_j;
                }
            } else {
                ans += 1;
                continue 'i_j;
            }
        }
    }
    println!("{ans}");
}
