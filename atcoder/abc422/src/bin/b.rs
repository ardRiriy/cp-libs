use library::utils::{consts::{DI, DJ}, input::Input};

fn solve(ip: &mut Input) {
    let (h, w) = ip.pair();
    let s = (0..h).map(|_| ip.chars()).collect::<Vec<_>>();
    
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' { continue; }
            let cnt = (0..4).map(|r| {
                let di = i.wrapping_add(DI[r]);
                let dj = j.wrapping_add(DJ[r]);
                if di >= h || dj >= w {
                    return 0;
                }
                if s[di][dj] == '#' { 1 } else { 0 }
            })
            .sum::<i32>();
            if cnt != 2 && cnt != 4 {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
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
