use std::collections::BTreeMap;

use library::utils::{chlibs::ChLibs, input::Input};

fn solve(ip: &mut Input) {
    let n = ip.next();
    
    let mut m = 0;
    let v = (0..n).map(|_| {
        let k = ip.next();
        m.chmax(k);
        ip.vector::<u64>(k)
    })
    .collect::<Vec<_>>();

    let mut g: Vec<BTreeMap<u64, u64>> = vec![BTreeMap::new(); m];
    for vi in v.iter() {
        for (j, &x) in vi.iter().enumerate() {
            let nxt = if j+1<vi.len() { vi[j+1] } else { 0 };
            if let Some(mic) = g[j].get_mut(&x) {
                mic.chmin(nxt);
            } else {
                g[j].insert(x, nxt);
            }
        } 
    }
    
    let mut ans = vec![];
    while ans.len() < m {
        let (key, nxt) = g[ans.len()].first_key_value().unwrap();
        let mut nxt = *nxt;
        
        ans.push(*key);
        
        while nxt > 0 && ans.len() < m {
            let nxt2= g[ans.len()][&nxt];
            ans.push(nxt);
            nxt = nxt2;
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
