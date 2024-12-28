use std::collections::BTreeSet;
use itertools::iproduct;
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        h: usize,
        w: usize,
        q: usize,
    }

    let mut breaked = BTreeSet::new();
    let mut row = vec![BTreeSet::new(); h];
    let mut column = vec![BTreeSet::new(); w];

    for (i, j) in iproduct!(0..h, 0..w) {
        row[i].insert(j);
        column[j].insert(i);
    }

    let mut state = vec![vec![true; w]; h];

    for _ in 0..q {
        input! {
            r: Usize1,
            c: Usize1,
        }

        if !breaked.insert((r, c)) {
            if let Some(&j) = row[r].range(0..c).last() {
                row[r].remove(&j);
                column[j].remove(&r);
                breaked.insert((r, j));
                state[r][j] = false;
            }
            if let Some(&i) = column[c].range(0..r).last() {
                row[i].remove(&c);
                column[c].remove(&i);
                breaked.insert((i, c));
                state[i][c] = false;
            }

            if let Some(&j) = row[r].range(c+1..).next() {
                row[r].remove(&j);
                column[j].remove(&r);
                breaked.insert((r, j));
                state[r][j] = false;
            }
            if let Some(&i) = column[c].range(r+1..).next() {
                row[i].remove(&c);
                column[c].remove(&i);
                breaked.insert((i, c));
                state[i][c] = false;
            }
        } else {
            state[r][c] = false;
            row[r].remove(&c);
            column[c].remove(&r);
        }
    }
    println!("{}", h*w-breaked.len());
}
