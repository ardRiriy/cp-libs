use library::utils::{chlibs::ChLibs, consts::INF, input::Input};

fn solve(ip: &mut Input) {
    let (n, m, l) = ip.triple::<usize, usize, usize>();
    let a = ip.vector::<u64>(n);
    
    // dp_{i, j} := a_{i%L}のあまりをjにするために必要な操作回数の総和
    let mut dp = vec![INF; m];
    dp[0] = 0;
    
    for i in 0..l {
        let mut ndp = vec![INF; m];
        
        let costs = (0..m).map(|k| {
            a
                .iter()
                .skip(i)
                .step_by(l)
                .map(|ai| if k as u64 >= *ai { k as u64 - *ai} else { k as u64 + (m as u64 - *ai) })
                .sum::<u64>()
            })
            .collect::<Vec<_>>();

        for j in 0..m {
            if dp[j] == INF { continue; }
            for k in 0..m {
                ndp[(j+k)%m].chmin(dp[j] + costs[k]);
            }
        }
        dp = ndp;
    }
    
    println!("{}", dp[0]);
    
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
