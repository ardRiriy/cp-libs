use library::{graph::warshall_floyd::{DefaultWFelm, WFelm, WarshallFloyd}, utils::input::Input};

fn solve(ip: &mut Input) {
    let (n, m) = ip.pair::<usize, usize>();
    let mut g = ip.weighted_graph::<u64>(n, m, false, true);
    g.push(vec![]);
    
    let (k, t) = ip.pair::<usize, u64>();
    let d = ip.vector::<usize>(k).iter().map(|i| *i-1).collect::<Vec<_>>();
    for i in 0..k {
        g[d[i]].push((n, t));
        g[n].push((d[i], 0));
    }

    let mut wf = WarshallFloyd::new(&g, DefaultWFelm);
    let elm= DefaultWFelm;

    let q = ip.next();
    for _ in 0..q {
        let qt :i32 = ip.next();

        if qt == 1 {
            let (x,y,w) = ip.triple::<usize,usize,u64>();
            wf.add(x-1, y-1, w);
            wf.add(y-1, x-1, w);
        } else if qt == 2 {
            let x :usize = ip.next();
            let x = x-1;

            wf.add(x, n, t);
            wf.add(n, x, 0);
        } else {
            let mut res = 0;

            for i in 0..n {
                for j in 0..n {
                    let ret = wf.get(i, j);
                    if ret != elm.infinity() {
                        res += ret;
                    }
                }
            }
            println!("{}", res);
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
