use library::utils::{chlibs::ChLibs, input::Input};

fn solve(ip: &mut Input) {
    let n = ip.next::<usize>();
    let v = (0..n)
        .map(|_| ip.pair::<usize,usize>())
        .map(|(a,b)| (a-1, b-1))
        .collect::<Vec<_>>();
    
    let mut g :Vec<Option<(usize, usize)>> = vec![None; 2*n];
    for (i, &(a, b)) in v.iter().enumerate() {
        if g[a].is_none() || g[a].unwrap().0 < b+1 {
            g[a] = Some((b+1,i));
        } 
    }
    
    let mut cur_r = 0;
    let mut ans = vec![];
    
    for i in 0..2*n {
        if let Some((val, x)) = g[i] {
            if cur_r.chmax(val) {
                ans.push(x);
            }
        }
    }
    println!("{}", ans.len());
    println!("{}", ans.iter().map(|x| (*x+1).to_string()).collect::<Vec<String>>().join(" "));

}

fn main() {
    static IS_MULTI_TEST_CASE :bool = true;
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
