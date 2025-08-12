use library::{data_structure::segment_tree::{Monoid, SegmentTree}, utils::{chlibs::ChLibs, consts::INF, input::Input}};

struct Sum;
impl Monoid for Sum {
    type S = usize;

    fn op(a: Self::S, b: Self::S) -> Self::S {
        a+b
    }

    fn id() -> Self::S {
        0
    }
}

fn solve(ip: &mut Input) {
    let n = ip.next::<usize>();

    let a = ip.vector::<u32>(n).iter().map(|x| x-1).collect::<Vec<_>>();
    let mut seg :SegmentTree<Sum> = SegmentTree::from_vec(&vec![1; n]);
    let mut a = a.iter()
        .enumerate()
        .map(|(i,&ai)| (ai, i))
        .collect::<Vec<_>>();
    a.sort();
    
    let mut dp = vec![INF; 2];
    dp[0] = 0;
    
    for i in 0..n {
        let (_, p) = a[i];
        let idx = seg.get(..p);
        let jdx = seg.get(p+1..);
        
        let mut ndp = vec![INF; 2];
        for j in 0..2 {
            if dp[j] != INF {
                ndp[0].chmin(dp[j] + idx as u64);
                ndp[1].chmin(dp[j] + jdx as u64);
            }
        }

        dp = ndp; 
        seg.set(p, 0);
    }

    println!("{}", dp.iter().min().unwrap());
}

fn main() {
    static IS_MULTI_TEST_CASE :bool = true;
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
