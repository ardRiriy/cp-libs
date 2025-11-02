use library::{
    cumulative_sum::CumulativeSum,
    utils::{input::Input, iterlibs::collect::CollectIter},
};

fn solve(ip: &mut Input) {
    let (n, a, b) = ip.triple();
    let s = ip.chars();
    let av = s
        .iter()
        .map(|c| if c == &'a' { 1 } else { 0 })
        .collect_vec();
    let bv = s
        .iter()
        .map(|c| if c == &'b' { 1 } else { 0 })
        .collect_vec();
    let ca = CumulativeSum::new(&av);
    let cb = CumulativeSum::new(&bv);

    let mut ans = 0;
    for i in 0..n {
        if ca.get(i..n) < a {
            break;
        }
        let mut ok = n;
        let mut ng = if i == 0 { !0 } else { i - 1 };
        while ok.wrapping_sub(ng) > 1 {
            let mid = ok.wrapping_add(ng) >> 1;
            if ca.get(i..mid) >= a {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        let k = ok - 1;

        if cb.get(i..n) < b {
            ans += n - ok + 1;
            continue;
        }
        let mut ok = i;
        let mut ng = n;
        while (ng - ok) > 1 {
            let mid = (ok + ng) >> 1;
            if cb.get(i..=mid) < b {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        if ok >= k {
            ans += ok - k + 1;
        }
    }

    println!("{}", ans);
}

fn main() {
    static IS_MULTI_TEST_CASE: bool = false;
    let mut ip = Input::new();

    let t = if IS_MULTI_TEST_CASE {
        ip.next::<usize>()
    } else {
        1
    };

    for _ in 0..t {
        solve(&mut ip);
    }
}
