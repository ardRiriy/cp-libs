use library::{math::prime::divisors, utils::input::Input};

fn solve(ip: &mut Input) {
    let n = ip.next::<usize>();
    let v = (0..n).map(|_| ip.pair::<u64,u64>()).collect::<Vec<_>>();
    
    let mut seen = vec![None; 1000000+1];
    
    for &(a, b) in v.iter() {
        let d = divisors(b);
        for di in d {
            if let Some(x) = seen[di as usize] {
                if x != a % di {
                    println!("No");
                    return;
                }
            } else {
                seen[di as usize] = Some(a % di);
            }
        }
    }

    println!("Yes");
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
