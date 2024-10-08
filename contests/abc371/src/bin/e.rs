use std::collections::BTreeSet;

use proconio::{input, marker::Usize1};
fn main() {
    input!{
        n: usize,
        a: [Usize1; n],
    }

    let mut bit = BIT::new(n);
    let mut set = vec![BTreeSet::new(); n];
    let mut cnt = BTreeSet::new();

    for  (i, &x) in a.iter().enumerate() {
        set[x].insert(i);
        cnt.insert(x);

        bit.add(i+1, i+2, cnt.len() as isize);
    }

    let mut ans = 0;
    for (i, &x) in a.iter().enumerate() {
        ans += bit.sum(n) - bit.sum(i);
        if let Some(&idx) =  set[x].range(i+1..).next() {
            bit.add(i+2, idx+1, -1);
        } else {
            bit.add(i+2, n+2, -1);
        }
    }
    println!("{ans}");
}

use num_traits::{FromPrimitive, Num, NumCast};
struct BIT<T> {
    n: usize, // 配列の要素数
    bit: Vec<Vec<T>>, // データを持つ。1-indexedで、初期値は0となる。
}

// all 1-indexed

impl<T> BIT<T> where T: Num + Clone + std::ops::Neg<Output = T> + NumCast + FromPrimitive {
    fn new(size: usize) -> BIT<T>{
        let v = vec![vec![T::zero(); size+1]; 2];
        BIT{ n: size+1, bit: v}
    }

    //[l, r)にxを追加
    fn add(&mut self, l: usize, r: usize, x: T) {
        self._add_sub(0, l, -x.clone() * NumCast::from(l - 1).unwrap());
        self._add_sub(0, r, x.clone() * NumCast::from(r - 1).unwrap());
        self._add_sub(1, l, x.clone());
        self._add_sub(1, r, -x.clone());
    }

    fn _add_sub(&mut self, p: usize, i: usize, x: T) {
        let mut idx = i;
        while idx < self.n {
            self.bit[p][idx] = self.bit[p][idx].clone() + x.clone();

            idx += (idx as isize & -(idx as isize)) as usize;
        }
    }

    fn sum(&self, i: usize) -> T { self._sum_sub(0, i) + self._sum_sub(1, i) * NumCast::from(i).unwrap() }

    fn _sum_sub(&self, p: usize, i: usize) -> T{
        let mut s :T = T::zero();
        let mut idx = i;
        while idx > 0 {
            s = s + self.bit[p][idx].clone();
            idx -= (idx as isize & -(idx as isize)) as usize;
        }
        s
    }
}
