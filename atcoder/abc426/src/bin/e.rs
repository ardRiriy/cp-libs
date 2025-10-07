use std::mem::swap;

use library::utils::input::Input;

fn dist(a: (f64, f64), b: (f64, f64)) -> f64 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}

fn f(a: (f64, f64), v: (f64, f64), t: f64) -> (f64, f64) {
    (a.0 + v.0 * t, a.1 + v.1 * t)
}

fn solve(ip: &mut Input) {
    let mut tst = ip.pair();
    let mut tg = ip.pair();
    let mut ast = ip.pair();
    let mut ag = ip.pair();

    let mut t_dist = dist(tst, tg);
    let mut a_dist = dist(ast, ag);

    if t_dist > a_dist {
        swap(&mut tst, &mut ast);
        swap(&mut tg, &mut ag);
        swap(&mut t_dist, &mut a_dist);
    }

    let vt = ((tg.0 - tst.0) / t_dist, (tg.1 - tst.1) / t_dist);
    let va = ((ag.0 - ast.0) / a_dist, (ag.1 - ast.1) / a_dist);

    let mut l = 0.;
    let mut r = t_dist;

    for _ in 0..50 {
        let t1 = l + (r - l) / 3.;
        let t2 = r - (r - l) / 3.;

        let tp1 = f(tst, vt, t1);
        let ap1 = f(ast, va, t1);
        let d1 = dist(tp1, ap1);

        let tp2 = f(tst, vt, t2);
        let ap2 = f(ast, va, t2);
        let d2 = dist(tp2, ap2);

        if d1 > d2 {
            l = t1;
        } else {
            r = t2;
        }
    }
    let ans1 = dist(f(tst, vt, l), f(ast, va, l));

    // tgが固定
    let mut l = t_dist;
    let mut r = a_dist;
    for _ in 0..50 {
        let t1 = l + (r - l) / 3.;
        let t2 = r - (r - l) / 3.;

        let ap1 = f(ast, va, t1);
        let d1 = dist(tg, ap1);
        let ap2 = f(ast, va, t2);
        let d2 = dist(tg, ap2);
        if d1 > d2 {
            l = t1;
        } else {
            r = t2;
        }
    }
    let ans2 = dist(tg, f(ast, va, l));
    println!("{}", ans1.min(ans2));
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
