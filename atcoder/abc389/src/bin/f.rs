use ac_library::{LazySegtree, MapMonoid, Max};
#[allow(unused_imports)]
use cps::debug::*;
use proconio::input;

struct Mn;
impl MapMonoid for Mn {
    type M = Max<usize>;
    type F = usize;
    
    fn identity_map() -> Self::F {
        0
    }
    
    fn mapping(f: &Self::F, x: &<Self::M as ac_library::Monoid>::S) -> <Self::M as ac_library::Monoid>::S {
        *f+x
    }
    
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        *f+g
    } 
}

fn main() {
    input!{
        n: usize,
        r: [(usize, usize); n],
        q: usize,
        qv: [usize; q],
    }

    static N :usize = 5e5 as usize + 5;
    let mut seg :LazySegtree<Mn> = LazySegtree::new(N);
    for i in 0..N {
        seg.set(i, i);
    }

    for (l, r) in r {
        let li = seg.max_right(0, |x| x < l);
        let ri = seg.max_right(0, |x| x < r+1);
        if li < N && ri < N {
            seg.apply_range(li..ri, 1);
        }
    }
    for q in qv {
        println!("{}", seg.get(q));
    }
}
