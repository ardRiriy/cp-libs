use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
    }

    let mut v = vec![];

    let mut l = 1;
    while l <= n {
        let val = n / l;
        v.push(val);

        // migihaji
        let mut ok = l;
        let mut ng = n+1;
        while ng - ok > 1 {
            let mid = (ok + ng) / 2;
            if val == n / mid {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        // ok ga migihaji
        l = ok + 1;
    }
}