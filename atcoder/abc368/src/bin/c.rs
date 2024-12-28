use proconio::{input};
fn main() {
    input!{
        n: usize,
        a: [u64; n]
    }

    let mut ans = 0;
    let mut left = 0;
    for &x in a.iter() {
        // leftの処理
        let mut health = x;
        if left > 0 {
            if left == 2 {
                // 1減らす
                health -= 1;
                left -= 1;
            }

            if health == 0 {
                continue;
            }

            if left == 1 {
                if health <= 3 {
                    left = 0;
                    continue;
                } else {
                    health -= 3;
                    left = 0;
                }
            }
        }

        let cycle = health / 5;
        ans += cycle * 3;
        let remain = health % 5;
        if remain != 0 {
            ans += 3;
        }

        if remain == 1 || remain == 2 {
            left = 3 - remain;
        } else {
            left = 0;
        }
        // dbg!(ans);
        // dbg!(left);
        // dbg!(remain);
    }

    ans -= left;
    println!("{}", ans);
}
