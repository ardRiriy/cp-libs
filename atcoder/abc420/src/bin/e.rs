use ac_library::Dsu;
use library::{data_structure::unionfind::UnionFind, utils::input::Input};

// ???????????
// UFバグってるし普通にミスするしで最悪ね

fn solve(ip: &mut Input) {
    let (n, q) = ip.pair::<usize, usize>();


    let mut uf = Dsu::new(n);
    let mut cur = vec![0u64; n];
    let mut sum = vec![0; n];

    for _ in 0..q {
        let t :u8 = ip.next();
        if t == 1 {
            let (u, v) = ip.pair::<usize, usize>();
            let pu = sum[uf.leader(u-1)];
            let pv = sum[uf.leader(v-1)];
            if uf.same(u-1, v-1) {
                continue;
            }
            sum[uf.leader(u-1)] = 0;
            sum[uf.leader(v-1)] = 0;

            uf.merge(u-1, v-1);
            
            sum[uf.leader(u-1)] = pu + pv;
        } else if t == 2 {

            let u = ip.next::<usize>() - 1;
            sum[uf.leader(u)] -= cur[u];

            cur[u] = 1 - cur[u];
            sum[uf.leader(u)] += cur[u];

        } else {
            let u = ip.next::<usize>() - 1;
            println!("{}", if sum[uf.leader(u)] == 0 { "No" } else { "Yes" });
        }
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
