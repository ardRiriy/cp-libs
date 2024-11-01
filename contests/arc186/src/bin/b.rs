use std::collections::{BTreeMap, BTreeSet, VecDeque};

use ac_library::{Min, ModInt998244353 as Mint};
use proconio::input;


fn create_edges(a: &Vec<usize>) -> BTreeSet<(usize, usize)> {
    let mut res = BTreeSet::new();
    for (i, &ai) in a.iter().enumerate() {
        if ai != 0 {
            res.insert((i+1, ai));
        }

        for j in ai+1..i+1 {
            res.insert((j, i+1));
        }
    }
    res
}

fn dfs(
    g: &Vec<Vec<usize>>,
    memo: &mut BTreeMap<Vec<usize>, Mint>,
    inv: &Vec<usize>,
    checked: &mut BTreeSet<usize>,
) -> Mint {
    if let Some(&val) = memo.get(inv) {
        return val;
    }
    let mut result = Mint::new(0);
    
    for v in 0..inv.len() {
        if checked.contains(&v) {
            continue;
        }
        if inv[v] == 0 {
            checked.insert(v);
            let mut next_in_deg = inv.clone();
            
            for &u in &g[v] {
                next_in_deg[u] -= 1;
            }
            
            result += dfs(g, memo, &next_in_deg, checked);
            checked.remove(&v);
        }
    }
    
    memo.insert(inv.clone(), result);
    result
}

fn main() {
    input!{
        n: usize,
        a: [usize; n],
    }

    let mut memo = BTreeMap::new();
    let mut inv = vec![0usize; n];
    memo.insert(inv.clone(), Mint::new(1));
    let mut g = vec![vec![]; n];

    for (u, v) in create_edges(&a) {
        g[u-1].push(v-1);
        inv[v-1] += 1;
    }
    
    let result = dfs(&g, &mut memo, &inv, &mut BTreeSet::new());        
    println!("{}", result);
}

