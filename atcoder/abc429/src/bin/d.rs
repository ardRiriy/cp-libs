use library::{
    cumulative_sum::CumulativeSum,
    utils::{
        input::Input,
        iterlibs::{
            collect::CollectIter,
            dedup::{DedupIterator, RleItertor},
        },
    },
};

fn solve(ip: &mut Input) {
    let n = ip.next();
    let m: usize = ip.next();
    let c = ip.next();
    let mut a = ip.vector::<u64>(n);
    a.sort();

    let b_pos = a.iter().rle().map(|(_, i)| *i as usize).collect_vec();
    let b = a.iter().rle().map(|(c, _)| c).collect_vec();

    let s = {
        let mut bd = b.clone();
        bd.extend(&b);
        bd
    };
    let csum = CumulativeSum::new(&s);

    let mut l = 0;
    let mut ans = 0;
    for (i, r) in b_pos.iter().enumerate() {
        let mut ok = 2 * b_pos.len();
        let mut ng = i;
        while ok.abs_diff(ng) > 1 {
            let mid = (ok + ng) >> 1;
            if csum.get(i..mid) >= c {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        let xi = csum.get(i..ok);
        ans += (*r - l) * xi;
        if i == 0 {
            ans += (m - b_pos.last().unwrap()) * xi;
        }
        l = *r;
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
