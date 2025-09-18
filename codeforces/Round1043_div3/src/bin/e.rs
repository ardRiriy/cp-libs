use library::{cumulative_sum::CumulativeSum, utils::input::Input};

fn solve(ip: &mut Input) {
    let (n, m, q) = ip.triple::<usize, usize, usize>();
    let mut a = ip.vector::<u64>(n);
    a.sort();
    a.reverse();
    let mut b = ip.vector::<u64>(m);
    b.sort();
    b.reverse();

    let acs = CumulativeSum::new(&a);
    let bcs = CumulativeSum::new(&b);
    
    let mut v = vec![];
    let mut ab = vec![vec![]; 2];

    let mut idxs = vec![0; 2];
    
    for _ in 0..n+m {
        if a[idxs[0]] >= b[idxs[1]] {
            v.push(a[idxs[0]]);
            idxs[0] += 1;
            ab[0].push(idxs[0]);

            ab[1].push(idxs[1]);
        } else {
            v.push(b[idxs[1]]);
            idxs[1] += 1;
            ab[1].push(idxs[1]);

            ab[0].push(idxs[0]);
        }
        
        if idxs[0] == n {
            for i in idxs[1]..m {
                ab[1].push(i+1);
                v.push(b[i]);

                ab[0].push(idxs[0]);
            }
            break;
        } else if idxs[1] == m {
            for i in idxs[0]..n {
                ab[0].push(i+1);
                v.push(a[i]);

                ab[1].push(idxs[1]);
            }
            break;
        }
    }

    let zcs = CumulativeSum::new(&v);
    
    for _ in 0..q {
        let (x,y,z) = ip.triple::<usize, usize, usize>();
        if z == 0 {
            println!("0");
            continue;
        }
        
        if ab[0][z-1] <= x && ab[1][z-1] <= y {
            println!("{}", zcs.get(..z));
        } else {
            let xs = acs.get(..x.min(z)) + bcs.get(..z-x.min(z));
            let ys = acs.get(..z-y.min(z)) + bcs.get(..y.min(z));
            println!("{}", xs.max(ys));
        }

    }
    
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
