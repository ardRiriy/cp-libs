use std::collections::BTreeMap;

use ac_library::ModInt998244353 as Mint;
use proconio::input;

struct Combinations {
    pascal_triangle: Vec<Vec<Mint>>
}

impl Combinations {
    fn new(size: usize) -> Self {
        let mut res = vec![vec![Mint::new(0); size+1]; size+1];
        res[0][0] = Mint::new(1);

        for i in 1..=size {
            for j in 0..=(size/2) {
                if j != 0 {
                    res[i][j] = res[i][j] + res[i-1][j-1];
                }
                res[i][j] = res[i][j] + res[i-1][j];
            }
        }
    
        Combinations { pascal_triangle: res }
    }

    fn get(&self, n: usize, k: usize) -> Mint {
        self.pascal_triangle[n][k]
    }
}

static N: usize = 26;

fn dfs(x: usize, left: usize, c: &[usize], cmb: &mut Combinations, memo: &mut BTreeMap<(usize, usize), Mint>) -> Mint {
    if x == N {
        return if left == 0 { Mint::new(1) } else { Mint::new(0) };
    }
    if let Some(val) = memo.get(&(x, left)) {
        return *val;
    }

    let mut res = Mint::new(0);

    for i in 0..=left.min(c[x]) {
        // (leftからi個選ぶ選び方) * (left - i)で他の選び方
        res += cmb.get(left, i) * dfs(x+1, left - i, c, cmb, memo);
    }

    memo.insert((x, left), res);
    res
}

fn main() {
    input! {
        k: usize,
        c: [usize; N]
    }

    let mut cmb = Combinations::new(k+1);
    let mut memo = BTreeMap::new();
    let ans = (1..=k)
        .map(|i| dfs(0, i, &c, &mut cmb, &mut memo))
        .sum::<Mint>();
    println!("{ans}");
}

