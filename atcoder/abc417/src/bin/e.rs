use library::{data_structure::unionfind::UnionFind, utils::input::Input};

fn ch(n: usize, s: usize, t: usize, used: &[bool], edges: &Vec<(usize, usize)>) -> bool {
    let mut uf = UnionFind::new(n, |_x, _y| 0);

    for &(u, v) in edges.iter() {
        if !used[u] && !used[v] {
            uf.merge(u, v);
        }
    }

    uf.same(s, t)
}

fn solve(ip: &mut Input) {
    let (n, m) = ip.pair::<usize, usize>();
    let (x, y) = ip.pair::<usize, usize>();
    let x = x - 1;
    let y = y - 1;

    let e = (0..m)
        .map(|_| {
            let (x, y) = ip.pair::<usize, usize>();
            (x - 1, y - 1)
        })
        .collect::<Vec<_>>();
    let mut g = e.iter().fold(vec![vec![]; n], |mut g, &(u, v)| {
        g[u].push(v);
        g[v].push(u);
        g
    });

    let mut used = vec![false; n];
    let mut checked = vec![false; n];
    used[x] = true;
    let mut ans = vec![x];
    let mut cur = x;

    for i in 0..n {
        g[i].sort();
    }
    while cur != y {
        for &ni in g[cur].iter() {
            if checked[ni] {
                continue;
            }
            if ch(n, ni, y, &used, &e) {
                cur = ni;
                ans.push(ni);
                used[ni] = true;
                break;
            } else {
                checked[ni] = true;
            }
        }
    }

    println!(
        "{}",
        ans.iter()
            .map(|x| (x + 1).to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

fn main() {
    static IS_MULTI_TEST_CASE: bool = true;
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
