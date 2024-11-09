use itertools::Itertools;
use proconio::input;
fn main() {
    input!{
        n: usize,
        k: u64,
        p: [(i64, i64); n]
    }
    if k == 1 {
        println!("Infinity");
        return;
    }
    
    let mut ans = 0u64;

    let mut seen = vec![vec![false; n]; n];
    for i in 0..n {
        for j in i+1..n {
            if seen[i][j] {
                continue;
            }
            let mut v = vec![j];
            let mut cnt = 2;
            let fixed_pj = (p[j].0 - p[i].0, p[j].1 - p[i].1);
            for k in j+1..n {
                let fixed_pk = (p[k].0 - p[i].0, p[k].1 - p[i].1);
                if fixed_pj.0 * fixed_pk.1 == fixed_pj.1 * fixed_pk.0 { // もし傾きが等しいなら
                    cnt += 1;
                    v.push(k);
                    seen[j][k] = true;
                    seen[i][k] = true;
                }
            }
            if cnt >= k {
                ans += 1;
            }
            
            for vi in v.iter().combinations(2) {
                seen[*vi[0]][*vi[1]] = true;
            }
        }
    }
    println!("{ans}");
}

