#[allow(unused_imports)]
use cps::debug::*;
use proconio::{input, marker::Chars};

fn main() {
    input!{
        h: usize,
        w: usize,
        c: [Chars; h],
    }

    let mut migi = vec![vec![0; 26]; h];
    let mut sita = vec![vec![0; 26]; w];
    for i in 0..h {
        for j in 0..w {
            migi[i][c[i][j] as usize - 'a' as usize] += 1;
            sita[j][c[i][j] as usize - 'a' as usize] += 1;
        }
    }

    loop {
        let mut migi_delete_list = vec![0; 26];
        let mut sita_delete_list = vec![0; 26];

        for i in 0..h {
            if migi[i].iter().filter(|&xi| *xi > 0).count() == 1 {
                let pos = migi[i].iter().position(|xi| *xi > 0).unwrap();
                if migi[i][pos] >= 2 {
                    migi[i][pos] = 0;
                    sita_delete_list[pos] += 1;
                }
            }
        }

        for i in 0..w {
            if sita[i].iter().filter(|&xi| *xi > 0).count() == 1 {
                let pos = sita[i].iter().position(|xi| *xi > 0).unwrap();
                if sita[i][pos] >= 2 {
                    sita[i][pos] = 0;
                    migi_delete_list[pos] += 1;
                }
            }
        }

        if migi_delete_list.iter().all(|ai| *ai == 0) && sita_delete_list.iter().all(|ai| *ai == 0) {
            break;
        }

        for i in 0..h {
            for (j, mi) in migi_delete_list.iter().enumerate() {
                migi[i][j] -= *mi;
            }
        }

        for i in 0..w {
            for (j, mi) in sita_delete_list.iter().enumerate() {
                sita[i][j] -= *mi;
            }
        }
    }
    println!("{}", migi.iter().filter(|v| v.iter().any(|vi| *vi > 0)).count() * sita.iter().filter(|v| v.iter().any(|vi| *vi > 0)).count());
}

