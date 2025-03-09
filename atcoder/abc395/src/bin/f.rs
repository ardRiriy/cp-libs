use proconio::input;

fn judge(key: i64, x: i64, v: &[(i64, i64)]) -> bool {
    let mut prev_range = (0, 1e10 as i64);

    for &(a, b) in v {
        let range = ((key-b).max(0), (key+b).min(a));
        let low = range.0.max(prev_range.0-x);
        let high = range.1.min(prev_range.1+x);
        if high < low {
            return false;
        }
        prev_range = (low, high);
    }
    true
}

fn main() {
    input!{
        n: usize,
        x: i64,
        v: [(i64, i64); n],
    }

    let mut ok = 0;
    let mut ng = 1e10 as usize;

    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        if judge(mid as i64, x, &v) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", v.iter().map(|&(a, b)| (a+b)-ok as i64).sum::<i64>());
    
}

