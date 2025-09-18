use std::collections::BinaryHeap;

use library::{misc::static_mint::modint998::Modint998, utils::input::Input};

fn solve(ip: &mut Input) {
    let n = ip.next::<usize>();
    let a = ip.vector::<usize>(n).iter().map(|i| *i-1).collect::<Vec<_>>();

    let mut ans = Modint998::new(1);
    
    let mut seen = vec![false; n];

    let mut l = 0;
    let mut r = n-1;
    
    let mut que = BinaryHeap::new();
    for (i, ai) in a.iter().enumerate() {
        que.push((*ai, i));
    }

    while que.len() > 1 {
        let (_ai, i) = que.pop().unwrap();
        if l == i || r == i {
            ans += Modint998::new(que.len() as u64 + 1);
        } else {
            ans += Modint998::new(1);
        }
        seen[i] = true;
        while seen[l] {
            l += 1;
        }
        while seen[r] {
            r -= 1;
        }
    }

    println!("{}", ans.value);
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
