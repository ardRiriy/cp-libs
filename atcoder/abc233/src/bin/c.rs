use proconio::input;
fn main() {
    input!{
        n: usize,
        x: u64,
    }
    let l = (0..n).fold(vec![], |mut acc, _| {
        input! {
            li: usize,
            a: [u64; li]
        }
        acc.push(a);
        acc
    });

    println!("{}", dfs(&l, n, x));

}

fn dfs(l: &Vec<Vec<u64>>, idx: usize, target: u64) -> u64 {
    if idx == 0 {
        return if target == 1 {
            1
        } else {
            0
        };
    }

    let mut res = 0;
    for elem in l[idx-1].iter() {
        if target % elem == 0 {
            res += dfs(l, idx-1, target/elem);
        }
    }

    res
}
