use std::collections::{BTreeMap, BTreeSet};

use library::{data_structure::unionfind::UnionFind, misc::static_mint::modint998::Modint998, utils::input::Input};

// N頂点のなもりグラフに含まれるサイクルの種類数は√Nで抑えられる

fn solve(ip: &mut Input) {
    let n = ip.next::<usize>();
    let a = ip.vector::<usize>(n)
        .iter()
        .map(|i| *i - 1)
        .collect::<Vec<_>>();
    
    let mut uf = UnionFind::new(n, |_,_| 0);

    for (i, ai) in a.iter().enumerate() {
        uf.merge(i, *ai);
    }
    
    let s = (0..n).map(|i| uf.leader(i)).collect::<BTreeSet<_>>();

    let con_sizes = s.iter()
        .map(|i| uf.size(*i))
        .fold(BTreeMap::new(), |mut acc, val| {
            *acc.entry(val).or_insert(0) += 1u64;
            acc
        });
    
    
    fn sum(size: u64) -> Modint998 {
        Modint998::new(size) * Modint998::new(size + 1) * Modint998::new(499122177) 
    }
    
    
    let mut ans = Modint998::new(0);
    for (x, cnt) in con_sizes {
        if x == 1 {
            continue;
        }

        let mut res = Modint998::new(0);
        for a in 1..=n {
            let lim = (a * x / (x-1)).min(n);
            
            res += sum(lim as u64) * (x as u64 - 1);
            res += Modint998::new((n-lim) as u64) * Modint998::new(a as u64) * x as u64;
        }
        ans += res * cnt;
    }

    println!("{}", ans.value);
}

fn main() {
    static IS_MULTI_TEST_CASE :bool = true;
    let mut ip = Input::new();

    let t = if IS_MULTI_TEST_CASE {
        ip.next::<usize>()
    } else {
        1
    };

    for _ in 0..t {
        solve(&mut ip);
    }
}
