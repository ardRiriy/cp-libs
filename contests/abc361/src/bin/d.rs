use std::collections::{BTreeMap, VecDeque};
use proconio::{input, marker::Chars};


fn main() {
    input!{
        n: usize,
        s: Chars,
        t: Chars,
    }

    // B -> 1 | W -> 2
    let mut state = vec![0; n+2];
    for (idx, &x) in s.iter().enumerate() {
        state[idx] = if x == 'B' { 1 } else { 2 };
    }

    let mut target = vec![0; n+2];
    for (idx, &x) in t.iter().enumerate() {
        target[idx] = if x == 'B' { 1 } else { 2 };
    }


    let mut seen :BTreeMap<Vec<i32>, u64> = BTreeMap::new();
    let mut que = VecDeque::new();
    que.push_back((state, 0));
    while let Some((v, cnt)) = que.pop_front() {
        if seen.get(&v).is_some() {
            continue;
        }
        seen.insert(v.clone(), cnt);

        let mut empty = 0;
        for i in 0..n+2 {
            if v[i] == 0 {
                empty = i;
                break;
            }
        }

        for i in 0..n+1 {
            if i != empty && i+1 != empty && empty + 1 != i {
                let mut nv = v.clone();
                // println!("{:?} {:?}", v, nv);
                // println!("{} {} {} {}", nv[i], nv[i+1], nv[empty], nv[empty + 1]);
                nv[empty] = nv[i];
                nv[empty+1] = nv[i+1];
                nv[i] = 0;
                nv[i+1] = 0;
                
                que.push_back((nv, cnt+1));
            }
        }
    } 

    // println!("{}", target);
    // rintln!("{:?}", seen);
    if let Some(x) = seen.get(&target) {
        println!("{}", x);
    } else {
        println!("-1");
    }


}
