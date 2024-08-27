use proconio::{input, marker::Usize1};
fn main() {
    input!{
        n: usize,
    }

    let arr = (0..n).into_iter()
        .fold(vec![], |mut v, _| {
            input! {
                t: u64,
                k: usize,
                a: [Usize1; k]
            }
            v.push((t, a));
            v
        });

    let ans = dfs(&arr, &mut vec![false; n], n-1);
    println!("{ans}");
}

fn dfs(arr: &Vec<(u64, Vec<usize>)>, seen: &mut Vec<bool>, pos: usize) -> u64 {
    let mut res = arr[pos].0;
    seen[pos] = true;

    for v in &arr[pos].1 {
        if !seen[*v] {
            res += dfs(arr, seen, *v);
        }
    }

    res
}
