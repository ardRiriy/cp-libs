use library::utils::{
    chlibs::ChLibs,
    input::Input,
    iterlibs::{collect::CollectIter, dedup::RleItertor},
};

fn solve(ip: &mut Input) {
    let n = ip.next::<usize>();
    let s = ip.chars();

    let zero_cnt = s.iter().filter(|ci| ci == &&'0').count();
    let one_cnt = n - zero_cnt;

    let rs = s.iter().copied().rle().collect_vec();

    let mut ans = if rs.len() == 1 { 0 } else { 1 << 60 };

    for i in 0..rs.len() - 1 {
        let (cnt1, c1) = rs[i];
        let (cnt2, c2) = rs[i + 1];

        let cost = (n - cnt1 - cnt2)
            + (cnt1
                + if c1 == '0' {
                    one_cnt - cnt2
                } else {
                    zero_cnt - cnt2
                })
            .min(
                cnt2 + if c2 == '0' {
                    one_cnt - cnt1
                } else {
                    zero_cnt - cnt1
                },
            );
        ans.chmin(cost);
    }

    println!("{}", ans);
}

fn main() {
    static IS_MULTI_TEST_CASE: bool = true;
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
