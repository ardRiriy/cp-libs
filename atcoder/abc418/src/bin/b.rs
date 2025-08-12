use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let s = ip.next::<String>().chars().collect::<Vec<_>>();
    let mut ans = 0.; 
    for i in 0..s.len() {
        if s[i] != 't' {
            continue;
        }
        for j in i+1..s.len() {
            if s[j-1] != 't' || j-i < 3 {
                continue;
            }
            let c = s[i..j].iter().filter(|&&c| c=='t').count();
            if c >= 3 {
                let val = (c-2) as f64 / (j-i - 2) as f64;
                if ans < val {
                    ans = val;
                }
            }
        }
    }

    println!("{:.20}", ans);
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
