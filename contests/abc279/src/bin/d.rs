use proconio::input;
/*
3分探索の練習問題。
操作をする回数は離散な値しか取れないのでusizeで持って考える。
最後のm1, m2再計算を忘れずに。
*/

fn main() {
    input!{
        a: f64,
        b: f64,
    }

    let mut l = 0;
    let mut h = 1e16 as usize;
    let mut ans = 1e100;

    let calc = |x: f64| -> f64 {
        b * x +  a / (x + 1.).sqrt()
    };

    for _ in 0..1000 {
        let m1 = (l * 2 + h) / 3;
        let m2 = (l + h * 2) / 3;
        if calc(m1 as f64) < calc(m2 as f64) {
            h = m2;
        } else {
            l = m1;
        }
    }
    let m1 = (l * 2 + h) / 3;
    let m2 = (l + h * 2) / 3;
    println!("{}", calc(m1 as f64).min(calc(m2 as f64)));
}
