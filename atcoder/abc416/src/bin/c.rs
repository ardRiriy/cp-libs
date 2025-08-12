use library::utils::input::Input;

fn dfs(idx: usize, k: usize, v: String, s: &Vec<String>, res: &mut Vec<String>) {
    if idx >= k {
        res.push(v);
        return;
    }

    for i in 0..s.len() {
        dfs(idx+1, k, format!("{}{}", v, s[i]), s, res);
    }
}

fn solve(ip: &mut Input) {
    let (n,k,x) = ip.triple::<usize,usize,usize>();
    let vs = ip.vector::<String>(n);
    let mut ans = Vec::new();
    dfs(0, k, String::new(), &vs, &mut ans);
    ans.sort();
    println!("{}", ans[x-1]);
}

fn main() {
    static IS_MULTI_TEST_CASE :bool = false;
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
