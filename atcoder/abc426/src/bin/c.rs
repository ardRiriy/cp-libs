use ac_library::{LazySegtree, MapMonoid, Monoid};
use library::utils::input::Input;

struct S;
impl Monoid for S {
    type S = (u64, usize);
    fn identity() -> Self::S {
        (0, 0)
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        (a.0 + b.0, a.1 + b.1)
    }
}

static ID: u64 = 8e18 as u64;
struct F;
impl MapMonoid for F {
    type M = S;

    type F = u64;

    fn identity_map() -> Self::F {
        ID
    }

    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        if *f != ID {
            (f * x.1 as u64, x.1)
        } else {
            *x
        }
    }

    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        if *f == ID {
            *g
        } else {
            *f
        }
    }
}

fn solve(ip: &mut Input) {
    let (n, q) = ip.pair();

    let mut lseg: LazySegtree<F> = LazySegtree::new(n);
    for i in 0..n {
        lseg.set(i, (1, 1));
    }

    for _ in 0..q {
        let (x, y) = ip.pair::<usize>();
        let x = x - 1;
        let y = y - 1;

        let (sum, _) = lseg.prod(..=x);
        lseg.apply_range(..=x, 0);
        let (ys, _) = lseg.get(y);
        lseg.set(y, (ys + sum, 1));
        println!("{}", sum);
    }
}

fn main() {
    static IS_MULTI_TEST_CASE: bool = false;
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
