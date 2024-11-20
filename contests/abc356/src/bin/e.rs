use ac_library::FenwickTree;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    }

    let m = 1e6 as usize + 3;
    a.sort_unstable();

    let mut ft :FenwickTree<i64> = FenwickTree::new(m, 0);
    for &xi in a.iter() {
        ft.add(xi, 1);
    }

    let mut memo = vec![None; m];
    let mut ans = 0;
    for &ai in a.iter() {
        ft.add(ai, -1);
        if let Some(val) = memo[ai] {
            memo[ai] = Some(val - 1);
            ans += val - 1;
            continue;
        }

        let mut k = 1;
        let mut s = 0;
        while ai * k < m {
            let val = ft.sum(ai*k..(ai*(k+1)).min(m));
            s += val * k as i64;
            k += 1;
        }
        ans += s;
        memo[ai] = Some(s);
    }
    
    println!("{ans}");

}
