use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let (n, p) = ip.pair::<usize>();
    let v = ip.vector::<usize>(n);
    

    let mut r = p;
    for i in (p..n).rev() {
        if v[i] == 0 {
            r = i+1;
            break;
        }
    }
    
    let right = (r - p) + v[p..r].iter().filter(|&i| i == &1).sum::<usize>();
    
    let mut l = p;
    for i in 0..p {
        if v[i] == 0 {
            l = i;
            break;
        }
    }
    let left = (p - l) + v[l..p].iter().filter(|&i| i == &1).sum::<usize>();
    println!("{}", left + right);
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
