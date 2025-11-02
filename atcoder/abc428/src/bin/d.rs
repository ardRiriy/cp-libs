use library::utils::input::Input;
use num::integer::Roots;

fn search(fx: u64) -> u64 {
    fx.sqrt()
}

fn solve(ip: &mut Input) {
    let c = ip.next::<u64>();
    let d = ip.next::<u64>();
    let mut ans = 0;

    for k in 1.. {
        if 10u64.pow(k) <= c {
            continue;
        }
        let up = (10u64.pow(k) - 1 - c).min(d);
        let lw = if 10u64.pow(k - 1) > c {
            10u64.pow(k - 1) - c - 1
        } else {
            0
        };

        if lw >= d {
            break;
        }
        if lw >= up {
            continue;
        }
        let up_fx = c * 10u64.pow(k) + up + c;
        let lw_fx = c * 10u64.pow(k) + lw + c;

        let l = search(lw_fx);
        let r = search(up_fx);
        ans += r - l;
    }
    println!("{}", ans);
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
