// 素因数分解

use std::collections::BTreeMap;

fn prime_factorization(n: usize) -> BTreeMap<usize, usize> {
    let mut res = BTreeMap::new();

    let mut i = 2usize;
    let mut k = n;
    while i * i <= n {
        let mut cnt = 0usize;
        while k % i == 0 {
            k /= i;
            cnt += 1;
        }
        if cnt != 0 {
            res.insert(i, cnt);
        }
        i += 1;
    }

    if k != 1 {
        res.insert(k, 1);
    }

    res
}
