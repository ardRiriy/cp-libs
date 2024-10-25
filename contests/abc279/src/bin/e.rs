use itertools::Itertools;
use proconio::{input, marker::Usize1};
fn main() {
    input!{
        n: usize,
        m: usize,
        a: [Usize1; m]
    }

    let mut master = 0;
    let mut pos = vec![vec![]; n];

    for (i, &x) in a.iter().enumerate() {
        pos.swap(x, x+1);
        pos[master].push(i);

        if master == x {
            master += 1;
        } else if master == x + 1 {
            master -= 1;
        }
    }

    let mut ans = vec![0; m];
    for i in 0..n {
        for x in pos[i].iter() {
            ans[*x] = i+1;
        }
    }
    println!("{}", ans.iter().join("\n"));
}
