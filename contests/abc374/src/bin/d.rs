use itertools::Itertools;
use proconio::{input};

fn dist(a: (f64, f64), b: (f64, f64)) -> f64 {
    ((a.0 - b.0).powf(2.) + (a.1 - b.1).powf(2.)).sqrt()
}

fn main() {
    input!{
        n: usize,
        s: f64,
        t: f64,
        l: [[(f64, f64); 2]; n],
    }

    let base = l.iter()
        .map(|v| {
            dist(v[0], v[1])
        })
        .sum::<f64>() / t;
    let mut ans = 1e15;

    for v in l.iter().permutations(n) {
        for i in 0..1<<n {
            let mut time = dist((0., 0.), v[0][1 - (i & 1)]);

            for j in 0..n-1 {
                time += dist(
                    v[j][(i >> j) & 1],
                    v[j+1][1 - ((i >> (j+1)) & 1)]
                );
            }
            time /= s;
            if ans > time {
                ans = time;
            }
        }
    }

    println!("{}", base + ans);
}
