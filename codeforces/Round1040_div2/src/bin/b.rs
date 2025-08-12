use std::collections::VecDeque;

use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let (n, s) = ip.pair::<usize, usize>();
    let a = ip.vector::<usize>(n);
    
    let mut v = a.iter()
        .map(|x| if x==&0 { (0, 0) } else if x == &1 { (2, 1) } else { (1, 2) })
        .collect::<Vec<_>>();
    v.sort();
    let v = v.iter()
        .map(|i| i.1)
        .collect::<Vec<_>>();
    
    let mut seen = vec![vec![false; s+1]; n];
    seen[0][0] = true;
    let mut que = VecDeque::from([(0,0)]);
    while let Some((p, psum)) = que.pop_front() {
        // left
        if p > 0 {
            let nxt = psum + v[p-1];
            if nxt <= s && !seen[p-1][nxt] {
                seen[p-1][nxt] = true;
                que.push_back((p-1, nxt));
            }
        }
        if p+1 < n {
            let nxt = psum + v[p+1];
            if nxt <= s && !seen[p+1][nxt] {
                seen[p+1][nxt] = true;
                que.push_back((p+1, nxt));
            }
        }
    }
    let vs = v.iter().map(|i|i.to_string()).collect::<Vec<_>>().join(" ");
    println!("{}", if seen[n-1][s] { "-1".to_string() } else { vs });
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
