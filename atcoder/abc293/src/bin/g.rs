use itertools::Itertools;
use num_integer::Roots;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input!{
        n: usize,
        q: usize,
        a: [Usize1; n],
        queries: [(Usize1, Usize1); q],
    }
    let bucket_size = (2*n/q.sqrt()).max(1);
    let indicates = (0..q)
        .sorted_unstable_by_key(|&i| (queries[i].0 / bucket_size, queries[i].1))
        .collect_vec();

    static N: usize = 2e5 as usize+1;
    // [l, r)に含まれるa[i]の個数
    let mut cnt = vec![0; N];
    fn s(x: usize) -> usize {
        if x <= 2 {
            0
        } else {
            x * (x-1) * (x-2) / 6
        }

    }
    let mut sum = 0;

    let mut l = 0;
    let mut r = 0;
    let mut ans = vec![0; q];
    for i in indicates {
        let (ql, qr) = queries[i];
        while r < qr+1 {
            sum -= s(cnt[a[r]]);
            cnt[a[r]] += 1;
            sum += s(cnt[a[r]]);
            r += 1;
        }

        while l > ql {
            l -= 1;
            sum -= s(cnt[a[l]]);
            cnt[a[l]] += 1;
            sum += s(cnt[a[l]]);
        }
        while r > qr+1 {
            r -= 1;
            sum -= s(cnt[a[r]]);
            cnt[a[r]] -= 1;
            sum += s(cnt[a[r]]);
        }
        while l < ql {
            sum += s(cnt[a[l]]-1);
            sum -= s(cnt[a[l]]);
            cnt[a[l]] -= 1;
            l += 1;
        }

        ans[i] = sum;
    }
    println!("{}", ans.iter().join("\n"));
}

