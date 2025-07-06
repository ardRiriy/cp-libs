use std::collections::{BTreeMap, BTreeSet, VecDeque};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        (sj,si): (Usize1,Usize1),
        (wj,wi): (Usize1,Usize1),
        v: [(Usize1, Usize1); n],
    }

    let mut i_to_j :BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    let mut j_to_i :BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();

    for (bj, bi) in v {
        if let None = j_to_i.get(&bj) {
            j_to_i.insert(bj, BTreeSet::new());
        }
        if let Some(st) = j_to_i.get_mut(&bj) {
            st.insert(bi);
        }

        if let None = i_to_j.get(&bi) {
            i_to_j.insert(bi, BTreeSet::new());
        }
        if let Some(st) = i_to_j.get_mut(&bi) {
            st.insert(bj);
        }
    }


    let mut seen = BTreeMap::new();
    let mut que = VecDeque::from([(si,sj)]);
    seen.insert((si,sj), 0);

    while let Some((pi,pj)) = que.pop_front() {
        let val = *seen.get(&(pi,pj)).unwrap();

        if let Some(st) = i_to_j.get(&pi) {
            // 右
            if let Some(&nj) = st.range(pj+1..).next() {
                // (pi, nj-1)に移動
                if !seen.contains_key(&(pi,nj-1)) {
                    seen.insert((pi,nj-1), val+1);
                    que.push_back((pi,nj-1));
                }
            }

            if let Some(&nj) = st.range(..pj).last() {
                if !seen.contains_key(&(pi,nj+1)) {
                    seen.insert((pi,nj+1), val+1);
                    que.push_back((pi,nj+1));
                }
            }
        }

        if let Some(st) = j_to_i.get(&pj) {
            if let Some(&ni) = st.range(pi+1..).next() {
                if !seen.contains_key(&(ni-1,pj)) {
                    seen.insert((ni-1,pj), val+1);
                    que.push_back((ni-1,pj));
                }
            }

            if let Some(&ni) = st.range(..pi).next() {
                if !seen.contains_key(&(ni+1,pj)) {
                    seen.insert((ni+1,pj), val+1);
                    que.push_back((ni+1,pj));
                }
            }
        }
    }

    println!("{}", if let Some(&ans) = seen.get(&(wi,wj)) { ans as i64 } else { -1 });

}

