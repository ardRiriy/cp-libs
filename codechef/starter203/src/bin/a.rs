use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let n = ip.next();
    let s = ip.chars();
    
    let mut ans = vec![];
    let mut horyu = vec![];
    
    for (i, ci) in s[1..].iter().enumerate() {
        if *ci == '0' {
            horyu.push(i);
        } else {
            ans.push(i);
            horyu.reverse();
            ans.append(&mut horyu);
        }
    }
    
    let last = ans[ans.len()-1];
    while ans.len() < n {
        ans.push(last);
    }
    
    if ans.iter().all(|x| *x == 0) {
        ans[n-1] = n;
    }

    println!("{}", ans.iter().map(|x|x.to_string()).collect::<Vec<_>>().join(" "));
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
