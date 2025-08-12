use library::{cumulative_sum::CumulativeSum, utils::{input::Input, veclibs::VecLibs}};

fn solve(ip: &mut Input) {
    let n = ip.next::<usize>();
    let q = ip.next::<usize>();
    let mut a = ip.vector::<u64>(n);
    a.sort();

    let csum = CumulativeSum::new(&a);
    
    for _ in 0..q {
        let bi = ip.next::<u64>();

        let i = a.lower_bound(bi);
        let score = csum.get(..i) + (bi - 1) * (n - i) as u64;
        
        //dbg!(i, score);

        let mut ng = bi - 1;
        let mut ok = csum.get(..);
        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            
            if score < mid {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        println!("{}", if score < ok { ok.to_string() } else { "-1".to_string() });
    }

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
