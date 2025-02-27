use num_rational::Ratio;
use proconio::input;

fn main() {
    input! {
        n: usize,
        v: [(usize, usize); n],
    }
    // let mut ans = Ratio::new(0, 1);
    let mut ans = 0.;
    for i in 0..n {
        let (l1, r1) = v[i];
        let size_1 = r1-l1+1;

        for j in i+1..n {
            let (l2, r2) = v[j];
            let size_2 = r2 - l2 + 1;
            let mut cnt = 0;
            for x in l1..=r1 {
                for y in l2..=r2 {
                    if x > y { cnt += 1; }
                }
            }

            ans += cnt as f64 / (size_1*size_2) as f64;
        }
    }
    println!("{:.20}", ans);
}