use library::utils::input::Input;
use rand::{thread_rng, Rng};

fn solve(ip: &mut Input) {
    let n = ip.next();
    let v = (0..n).map(|_|ip.pair::<i64>()).collect::<Vec<_>>();
    
    let mut rng = thread_rng();
    
    let check = |i: usize, j: usize| -> bool {
        
        let cnt = v.iter()
            .filter(|(x, y)| (v[j].0 - v[i].0) * (*y - v[i].1) == (v[j].1 - v[i].1)*(*x - v[i].0))
            .count();
        2*cnt > n
    };
    
    for _ in 0..100 {
        let i = rng.gen_range(0..n);
        let mut j = rng.gen_range(0..n);
        while i==j {
            j = rng.gen_range(0..n);
        }
        
        if check(i,j) {
            println!("Yes");
            let b = v[j].0 - v[i].0;
            let a = v[i].1 - v[j].1;
            let c = v[j].1 * v[i].0 - v[j].0 * v[i].1;
            println!("{} {} {}", a, b, c);
            return;
        }
    }

    println!("No");
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
