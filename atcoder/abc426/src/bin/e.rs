use std::mem::swap;

use library::utils::input::Input;

fn dist(a: (f64, f64), b: (f64, f64)) -> f64 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}

fn ternary_search(alpha: (f64, f64), beta: (f64, f64), sl: f64, sr: f64) -> f64 {
    let func = |t: f64| -> f64 { (t * alpha.0 + beta.0).powi(2) + (t * alpha.1 + beta.1).powi(2) };
    let mut l = sl;
    let mut r = sr;

    for _ in 0..50 {
        let t1 = l + (r - l) / 3.;
        let t2 = r - (r - l) / 3.;
        let d1 = func(t1);
        let d2 = func(t2);
        if d1 > d2 {
            l = t1;
        } else {
            r = t2;
        }
    }
    return func(l).sqrt();
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

    let alpha = (va.0 - vt.0, va.1 - vt.1);
    let beta = (ast.0 - tst.0, ast.1 - tst.1);

    let ans1 = ternary_search(alpha, beta, 0., t_dist.min(a_dist));

    let beta = (ast.0 - tg.0, ast.1 - tg.1);
    let ans2 = ternary_search(va, beta, t_dist.min(a_dist), t_dist.max(a_dist));
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
