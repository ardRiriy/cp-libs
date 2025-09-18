use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let (n, m) = ip.pair::<usize ,usize>();
    
    let vs = ip.vector::<String>(n);
    
    let is_one_win = (0..m)
        .map(|i| vs.iter().filter(|s| s.chars().nth(i).unwrap() == '1').count() > n/2)
        .collect::<Vec<bool>>();
    
    let count = vs.iter()
        .map(|s| {
            s.chars()
            .enumerate()
            .filter(|(i,c)| is_one_win[*i] != (*c == '1'))
            .count()
        })
        .collect::<Vec<_>>();
    

    let mx = *count.iter().max().unwrap();
    
    let ans = count.iter()
        .enumerate()
        .filter_map(|(i,c)| if *c == mx { Some(i+1) } else { None })
        .collect::<Vec<_>>();

    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
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
