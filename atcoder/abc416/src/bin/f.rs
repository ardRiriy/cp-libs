use library::utils::{chlibs::ChLibs, input::Input};

static K :usize = 5;

// res.0[i] := 折り返しを一度も「使っていない」場合に、i個のパスの和の最大値
// res.1[i] := 使ったver
fn dfs(p: usize, seen: &mut Vec<bool>, a: &[u64], g: &Vec<Vec<usize>>) -> (Vec<u64>, Vec<u64>) {
    seen[p] = true;
    let mut res = (vec![0; K+1],vec![0; K+1]);

    for &ni in g[p].iter() {
        if seen[ni] { continue; }
 
        let (ta, tb) = dfs(ni, seen, a, g);
        for (idx, val) in ta.iter().enumerate() {
            if *val == 0 { continue; }
            
            
            // 折り返しなし
            for j in (0..K).rev() {
                if idx+j > K { continue; }
                let nxt_val = *val + res.0[j];
                res.0[idx+j].chmax(nxt_val);
            }

            
        }
    }

    for i in 0..K+1 {
        if res.0[i] != 0 { res.0[i] += a[p]; }
    }

    if res.0[1] == 0 { res.0[1] += a[p]; }


    res
}

fn solve(ip: &mut Input) {
    let (n, k) = ip.pair::<usize,usize>();
    let a = ip.vector::<u64>(n);
    let g = ip.graph(n,n-1, false);


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
