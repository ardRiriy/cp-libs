use library::utils::{input::Input, veclibs::VecLibs};

fn solve(ip: &mut Input) {
    let (n, m) = ip.pair::<usize, usize>();
    let mut a = ip.vector::<u64>(n);
    let mut b = ip.vector::<u64>(m);

    a.sort();
    b.sort(); 

    let mut ok = 1e9 as u64 + 1;
    let mut ng = 0;
    
    while ok-ng>1 {
        let mid = (ok + ng) / 2;

    }
    println!("{}", ok);

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
