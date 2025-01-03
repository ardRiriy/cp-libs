use cps::veclibs::VecLibs;
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        d: i64,
        p: [(i64, i64); n],
    }

    static M: i64 = 1e6 as i64; 

    let xs = p.iter().map(|&(x, _)| x).sorted().collect_vec();
    let ys = p.iter().map(|&(_, y)| y).sorted().collect_vec();

    let mut xmap = vec![];
    let mut ymap = vec![];

    let mut x_i = 0;
    let mut y_i = 0;

    let mut f_x = xs.iter().map(|&x| (-(M+d) - x).abs()).sum::<i64>(); 
    let mut g_y = ys.iter().map(|&y| (-(M+d) - y).abs()).sum::<i64>(); 

    for i in -(M+d)..=M+d {
        xmap.push(f_x);
        ymap.push(g_y);

        f_x += x_i as i64;
        f_x -= (n - x_i) as i64;
        g_y += y_i as i64;
        g_y -= (n - y_i) as i64;

        while x_i < n && xs[x_i] == i+1 {
            x_i += 1;
        }
        while y_i < n && ys[y_i] == i+1 {
            y_i += 1;
        }
    }

    ymap.sort_unstable();
    let mut ans = 0;
    for xi in xmap {
        let p = ymap.lower_bound(d-xi+1);
        ans += p;
    }
    println!("{}", ans);
}