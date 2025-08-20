use ac_library::ModInt998244353 as Mint;
use library::{algorithm::aho_corasick::AhoCorasick, utils::input::Input};

fn solve(ip: &mut Input) {
    let (n, l) = ip.pair::<usize, usize>();
    let s = (0..n).map(|_| ip.next::<String>()).collect::<Vec<_>>();
    
    let ahocora = AhoCorasick::new(&s);

    let m = ahocora.node_size();
    let mut dp = vec![vec![Mint::new(0); m]; 1<<n];
    
    dp[0][0] = Mint::new(1);
    
    for _i in 0..l {
        let mut ndp = vec![vec![Mint::new(0); m]; 1<<n];
        for st in 0..1<<n {
            for j in 0..m {
                let next_ids = ahocora.destination_node_ids_from_id(j);
                
                for id in next_ids  {
                    let outputs = ahocora.nodes[id].outputs.iter()
                        .map(|i| 1<<i)
                        .sum::<usize>();
                    
                    ndp[st | outputs][id] += dp[st][j];
                }
            }

        }
        dp = ndp;
    }

    println!("{}", dp[(1<<n)-1].iter().sum::<Mint>());
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
