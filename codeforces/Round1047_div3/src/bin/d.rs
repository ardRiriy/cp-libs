
use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let n = ip.next();
    let b = ip.vector::<usize>(n);

    let mut v = vec![vec![]; n+1];

    for (i, bi) in b.iter().enumerate() {
        v[*bi].push(i);
    }

    let mut cur = 1;
    let mut ans = vec![0; n];   
    
    for (i, vi) in v.iter().enumerate() {
        if i == 0 { 
            assert_eq!(vi.len(), 0);
            continue; 
        }

        if vi.len() % i != 0 {
            println!("-1");
            return;
        }

        for j in (0..vi.len()).step_by(i) {
            for k in j..j+i {
                ans[vi[k]] = cur;
            }
            cur += 1;
        }
    }


    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
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
