
use library::utils::input::Input;
use ac_library::{Monoid, Segtree};

struct Mx;
impl Monoid for Mx {
    type S = usize;

    fn identity() -> Self::S {
        0
    }
    
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a.max(b)
    }
}

struct Mn;
impl Monoid for Mn {
    type S = usize;
    fn identity() -> Self::S {
        1<<60
    }
    
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a.min(b)
    }
}

fn solve(ip: &mut Input) {
    let (n, q) = ip.pair();
    let mut seg :Segtree<Mx> = Segtree::new(n);
    let mut seg2 :Segtree<Mn> = Segtree::new(n);

    for _ in 0..q {
        let (a, b) = ip.pair::<usize>();
        let a = a-1;
        let b= b-1;
        let val1 = seg2.prod(a+1..b);
        if val1 < a {
            println!("No");
            continue;
        } 
        
        let val2 = seg.prod(a+1..b);
        if b < val2 {
            println!("No");
            continue;
        }
        println!("Yes");
        seg.set(a, b);
        seg2.set(b, a);
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
