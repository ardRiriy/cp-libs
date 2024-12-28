use itertools::Itertools;
use proconio::input;
fn main() {
    input!{
        n: u32,
        m: usize,
    }

    let v = (1..=m).collect_vec();
    let mut ans = Vec::new();
    for i in 0..1u32 <<m {
        if i.count_ones() != n {
            continue;
        }
        let mut res = Vec::new();
        for j in 0..m {
            if i >> j & 1 == 1 {
                res.push(v[j]);
            }
        }
        ans.push(res);
    }

    println!("{}", ans
        .iter()
        .sorted()
        .map(|v| v.iter().join(" "))
        .join("\n")
    );
}
