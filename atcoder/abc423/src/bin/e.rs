use library::{cumulative_sum::CumulativeSum, utils::input::Input};

fn solve(ip: &mut Input) {
    let (n, q) = ip.pair();
    let a = ip.vector::<i64>(n);
    
    let b = a.iter()
        .enumerate()
        .map(|(i, &ai)| (i+1) as i64 * ai)
        .collect();

    let c = a.iter()
        .enumerate()
        .map(|(i, &ai)| ((i+1)*(i+1)) as i64 * ai)
        .collect();
    
    let ca = CumulativeSum::new(&a);
    let cb = CumulativeSum::new(&b);
    let cc = CumulativeSum::new(&c);

    for _ in 0..q {
        let (l, r) = ip.pair::<usize>();
        let li64 = l as i64;
        let ri64 = r as i64;
        
        
        let va = cc.get(l-1..=r-1);  
        let vb = cb.get(l-1..=r-1) * (ri64+li64);
        let vc = ca.get(l-1..=r-1) * (ri64+1-li64*ri64-li64);

        println!("{}", vc+vb-va);
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
