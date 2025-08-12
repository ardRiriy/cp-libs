use ac_library::ModInt998244353 as Mint;
use ac_library::{LazySegtree, MapMonoid, Monoid};
use library::utils::input::Input;

#[derive(Clone, Debug)]
struct C {
    val: Option<Mint>,
    size: usize,
}

struct A;

impl Monoid for A {
    type S = C;

    fn identity() -> Self::S {
        C { val: None, size: 0 }
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        let l = if let Some(a) = a.val { a } else { Mint::new(0) };
        let r = if let Some(a) = b.val { a } else { Mint::new(0) };

        let val = if a.val.is_none() && b.val.is_none() {
            None
        } else {
            Some(l + r)
        };

        C {
            val,
            size: a.size + b.size,
        }
    }
}

struct B;
impl MapMonoid for B {
    type M = A;
    type F = Option<Mint>;

    fn identity_map() -> Self::F {
        None
    }

    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        let mut res = x.clone();
        if let Some(a) = f {
            res.val = Some(*a * res.size as u64);
        }

        res
    }

    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        if f.is_some() {
            *f
        } else {
            *g
        }
    }
}

fn solve(ip: &mut Input) {
    let (n, m) = ip.pair::<usize, usize>();
    let a = ip.vector::<u64>(n);

    let mut seg: LazySegtree<B> = LazySegtree::new(n);
    for i in 0..n {
        seg.set(
            i,
            C {
                val: Some(Mint::new(a[i])),
                size: 1,
            },
        );
    }

    for _ in 0..m {
        let (l, r) = ip.pair::<usize, usize>();
        let l = l - 1;
        let r = r;

        let s = seg.prod(l..r);
        seg.apply_range(l..r, Some(s.val.unwrap() / (r - l) as u64));
    }

    let ans = (0..n)
        .map(|x| seg.get(x).val.unwrap().val().to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", ans);
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
