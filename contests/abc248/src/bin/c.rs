use std::collections::BTreeMap;

use ac_library::{Mod998244353, ModInt998244353 as Mint, StaticModInt};
use proconio::input;

struct Memo {
    _n: usize,
    m: usize,
    _k: usize,
    memo: BTreeMap<(usize, usize), StaticModInt<Mod998244353>>, // (残りのA_iの個数, 使っていい上限値) -> 通り数 
}

impl Memo {
    fn dfs(&mut self, i:usize, left: usize) -> StaticModInt<Mod998244353> {
        if let Some(val) = self.memo.get(&(i, left)) {
            return *val;
        }
        if i == 0 {
            return Mint::new(1);
        }
        
        let mut res = Mint::new(0);
        for j in 1..=self.m {
            if left < j {
                // 合計でKを超える
                continue;
            }
            
            res += self.dfs(i-1, left - j);
        }

        self.memo.insert((i, left), res);
        res 
    }
}


fn main() {
    input!{
        n: usize,
        m: usize,
        k: usize,
    }
    
    let mut memo = Memo { _n: n, m, _k: k, memo: BTreeMap::new() };
    println!("{}", memo.dfs(n, k));
    
    
}

