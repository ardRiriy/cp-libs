use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let q = ip.next();
    let mut cur = Vec::new();
    let mut minus_cnt = 0;
    cur.push(0);
    for _ in 0..q {
        let t = ip.next();
        match t {
            1 => {
                let c = ip.next();
                let v = match c {
                    '(' => 1,
                    ')' => -1,
                    _ => unreachable!(),
                };
                let nxt = cur[cur.len() - 1] + v;
                cur.push(nxt);
                if nxt < 0 {
                    minus_cnt += 1;
                }
            }
            2 => {
                let lst = cur.pop().unwrap();
                if lst < 0 {
                    minus_cnt -= 1;
                }
            }
            _ => {
                unreachable!()
            }
        }
        if minus_cnt == 0 && cur.last().unwrap() == &0 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn main() {
    static IS_MULTI_TEST_CASE: bool = false;
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
