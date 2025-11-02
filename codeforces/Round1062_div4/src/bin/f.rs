use library::utils::input::Input;

fn dfs1(
    p: usize,
    g: &Vec<Vec<usize>>,
    seen: &mut Vec<bool>,
    res: &mut Vec<Vec<(usize, usize)>>,
) -> usize {
    seen[p] = true;
    let mut sum = 0;

    for ni in g[p].iter() {
        if seen[*ni] {
            continue;
        }
        let ret = dfs1(*ni, g, seen, res);
        res[p].push((ret, *ni));
        sum += ret;
    }

    sum + 1
}

fn dfs2(p: usize, res: &Vec<Vec<(usize, usize)>>, cnt: &mut Vec<usize>, k: usize) {
    let sum = res[p].iter().map(|(val, _)| *val).sum::<usize>();
    let ue = res.len() - sum - 1;
    let mut ans = 1;
    if res.len() - ue >= k {
        ans += ue;
    }
    for (nval, ni) in res[p].iter() {
        if res.len() - nval >= k {
            ans += *nval;
        }
        dfs2(*ni, res, cnt, k);
    }
    cnt[p] = ans;
}

fn solve(ip: &mut Input) {
    let (n, k) = ip.pair();
    let g = ip.graph(n, n - 1, false);
    let mut res = vec![vec![]; n];
    let mut seen = vec![false; n];
    let mut cnt = vec![0; n];
    dfs1(0, &g, &mut seen, &mut res);
    dfs2(0, &res, &mut cnt, k);
    println!("{}", cnt.iter().sum::<usize>());
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
