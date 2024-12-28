use std::collections::BTreeMap;
use itertools::Itertools;
use proconio::input;

// 小さい方からK個を管理する
#[derive(Debug)]
struct MinK {
    k: usize,
    cnt: usize,
    small_k_map: BTreeMap<u64, u64>,
    big_map: BTreeMap<u64, u64>,
    sum_small_k: u64,
}

impl MinK {
    fn new(k: usize) -> Self {
        assert_ne!(k, 0);
        MinK {
            k,
            cnt: 0,
            small_k_map: BTreeMap::new(),
            big_map: BTreeMap::new(),
            sum_small_k: 0,
        }
    }

    fn add(&mut self, x: u64) {
        *self.small_k_map.entry(x).or_insert(0) += 1;
        self.cnt += 1;
        self.sum_small_k += x;


        if self.cnt > self.k {
            let (&key, _) = self.small_k_map.last_key_value().unwrap();
            remove_from_map(&mut self.small_k_map, key);
            self.cnt -= 1;
            *self.big_map.entry(key).or_insert(0) += 1;

            self.sum_small_k -= key;
        }
    }

    fn remove(&mut self, x: u64) {
        if self.big_map.contains_key(&x) {
            remove_from_map(&mut self.big_map, x);
        } else {
            remove_from_map(&mut self.small_k_map, x);
            self.sum_small_k -= x;

            if let Some((&key, _)) = self.big_map.first_key_value() {
                *self.small_k_map.entry(key).or_insert(0) += 1;
                remove_from_map(&mut self.big_map, key);
                self.sum_small_k += key;
            } else {
                self.cnt -= 1;
            }
        }
    }

    fn ans(&self) -> u64 {
        self.sum_small_k
    }
}

fn remove_from_map(map: &mut BTreeMap<u64, u64>, key: u64) {
    if map[&key] == 1 {
        map.remove(&key);
    } else {
        map.insert(key, map[&key] - 1);
    }
}

fn main() {
    input!{
        n: usize,
        m: usize,
        k: usize,
        a: [u64; n]
    }

    let mut mink = MinK::new(k);
    let mut ans = vec![];
    for i in 0..m {
        mink.add(a[i]);
    }
    ans.push(mink.ans());

    for i in m..n {
        mink.remove(a[i-m]);
        mink.add(a[i]);

        ans.push(mink.ans());
    }
    println!("{}", ans.iter().join(" "));
}
