use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let n = ip.next::<usize>();
    let mut ans = vec![];
    
    let cc = ('a'..='d').collect::<Vec<_>>();
    for i in 0..n {
        ans.push(cc[i % 4]);
    }

    println!("{}", ans.iter().collect::<String>());
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
