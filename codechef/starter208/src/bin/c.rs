use library::utils::{
    input::Input,
    iterlibs::{collect::CollectIter, strs::StrUtilIter},
};

fn solve(ip: &mut Input) {
    let (n, k) = ip.pair::<usize>();
    if k == 0 {
        println!("{}", (1..=n).join(" "));
        return;
    }

    if k == 1 {
        let mut ans = (1..=n).collect_vec();
        for i in 1..=n {
            if i ^ k > 0 && i ^ k <= n && i != (i ^ k) {
                ans.swap(i - 1, (i ^ k) - 1);
                println!("{}", ans.iter().join(" "));
                return;
            }
        }
        println!("-1");
        return;
    }

    let mut b = 1;
    while b * 2 <= k {
        b *= 2;
    }
    if let Some(i) = (1..=n)
        .filter(|i| *i < b && *i ^ k > 0 && *i ^ k <= n && i ^ k >= b && (*i != *i ^ k))
        .next()
    {
        let mut ans = vec![];
        for j in 1..b {
            if i == j {
                continue;
            }
            ans.push(j);
        }
        ans.push(i ^ k);
        ans.push(i);
        for j in b..=n {
            if i ^ k == j {
                continue;
            }
            ans.push(j);
        }
        println!("{}", ans.iter().join(" "));
    } else {
        println!("-1");
    }
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
