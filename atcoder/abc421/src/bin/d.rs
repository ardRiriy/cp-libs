use std::collections::VecDeque;

use library::utils::{consts::DC, input::Input, iterlibs::dedup::RleItertor};
use rand::{thread_rng, Rng};

fn check_kukan_kasanari(a: i64, b: i64, c: i64, d: i64) -> bool {
    // [a, b], [c, d]が重なるか判定
    return (a<=d) && (c<=b);
}

fn check_point_in_kukan(a: i64, b: i64, c: i64) -> bool {
    // [a, b] に点 cが含まれるか？
    return a <= c && c <= b;
}

struct Ins {
    takahashi: (i64, i64),
    aoki: (i64, i64),
    n: usize,
    m: usize,
    l: usize,
    tv: Vec<(char, i64)>,
    av: Vec<(char, i64)>,
}

impl Ins {
    fn new(ip: &mut Input) -> Self {
        let takahashi = ip.pair::<i64, i64>();
        let aoki = ip.pair::<i64, i64>();
        
        let (n, m, l) = ip.triple::<usize, usize, usize>();
        let tv = (0..m).map(|_| ip.pair::<char, i64>()).collect::<Vec<_>>();
        let av = (0..l).map(|_| ip.pair::<char, i64>()).collect::<Vec<_>>();

        Self { takahashi, aoki, n, m, l, tv, av }
    }
    
    #[allow(dead_code)]
    fn gen() -> Self {
        let mut rng = thread_rng();
        
        let takahashi = (rng.gen_range(0..100), rng.gen_range(0..100));
        let aoki = (rng.gen_range(0..100), rng.gen_range(0..100));
        
        let n = rng.gen_range(0..30);
        
        let tv = (0..n).map(|_| {
                let c = match rng.gen_range(0..4) {
                    0 => 'L',
                    1 => 'R',
                    2 => 'U',
                    _ => 'D',
                };
                c
            })
            .rle()
            .map(|(a,b)| (b, a as i64))
            .collect::<Vec<_>>();
        let m = tv.len();
        
        let av = (0..n).map(|_| {
                let c = match rng.gen_range(0..4) {
                    0 => 'L',
                    1 => 'R',
                    2 => 'U',
                    _ => 'D',
                };
                c
            })
            .rle()
            .map(|(a,b)| (b, a as i64))
            .collect::<Vec<_>>();
        let l = av.len();
        
        Self { takahashi, aoki, n, m, l, tv, av }
    }
}


fn solve(ip: &Ins) -> i64 {
    let mut takahashi = ip.takahashi;
    let mut aoki = ip.aoki;

    let (_n, m, l) = (ip.n, ip.m, ip.l);

    let mut t_que = VecDeque::new();
    let mut a_que = VecDeque::new();
    for i in 0..m {
        let (c, cnt) = ip.tv[i];
        let idx = DC.iter().position(|dc| dc==&c).unwrap();
        t_que.push_back((idx, cnt));
    }

    for i in 0..l {
        let (c, cnt) = ip.av[i];
        let idx = DC.iter().position(|dc| dc==&c).unwrap();
        a_que.push_back((idx, cnt));
    }
    
    let mut ans = 0;

    while t_que.len()>0 {
        let (t_dir, t_c) = t_que.pop_front().unwrap();
        let (a_dir, a_c) = a_que.pop_front().unwrap();

        let times = t_c.min(a_c);
        if t_c > times {
            t_que.push_front((t_dir, t_c - times));
        }
        if a_c > times {
            a_que.push_front((a_dir, a_c - times));
        }
        
        if t_dir % 4 == a_dir % 4 {
            // 同じ方向
            // 始点が同じ => 移動回数だけ
            ans += if takahashi == aoki { times } else { 0 };
            // 変更無しでOK
        } else if t_dir % 2 == a_dir % 2 {
            // すれ違う可能性あり
            if t_dir % 2 == 0 {
                // 左右移動
                // .0が固定
                let after_takahashi = takahashi.1 + if t_dir == 0 { -times } else { times };
                let after_aoki = aoki.1 + if a_dir == 0 { -times } else { times };

                if takahashi.0 == aoki.0 && takahashi.1.abs() % 2 == aoki.1.abs() % 2 && takahashi != aoki &&
                    check_kukan_kasanari(
                        takahashi.1.min(after_takahashi),
                        takahashi.1.max(after_takahashi),
                        aoki.1.min(after_aoki),
                        aoki.1.max(after_aoki)) 
                    {
                        ans += 1;
                    };
                    
                takahashi.1 = after_takahashi;
                aoki.1 = after_aoki;
            } else {
                let after_takahashi = takahashi.0 + if t_dir == 1 { -times } else { times };
                let after_aoki = aoki.0 + if a_dir == 1 { -times } else { times };

                if takahashi.1 == aoki.1 && takahashi.0.abs() % 2 == aoki.0.abs() % 2 && takahashi != aoki &&
                    check_kukan_kasanari(
                        takahashi.0.min(after_takahashi),
                        takahashi.0.max(after_takahashi),
                        aoki.0.min(after_aoki),
                        aoki.0.max(after_aoki)) 
                    {
                        ans += 1;
                    };
                    
                takahashi.0 = after_takahashi;
                aoki.0 = after_aoki;
            }
        } else {
            // 十字で交わるか
            if t_dir % 2 == 0 {
                let after_takahashi = takahashi.1 + if t_dir == 0 { -times } else { times };
                let after_aoki = aoki.0 + if a_dir == 1 { -times } else { times };
                
                let pos = (takahashi.0, aoki.1);
                if check_point_in_kukan(
                        takahashi.1.min(after_takahashi),
                        takahashi.1.max(after_takahashi),
                        aoki.1
                    ) &&
                    check_point_in_kukan(
                        aoki.0.min(after_aoki),
                        aoki.0.max(after_aoki),
                        takahashi.0
                    ) &&
                    (pos.0-aoki.0).abs() == (pos.1-takahashi.1).abs() && 
                    takahashi != aoki
                    {
                        ans += 1;
                    }
                takahashi.1 = after_takahashi;
                aoki.0 = after_aoki;
            } else {
                let after_takahashi = takahashi.0 + if t_dir == 1 { -times } else { times };
                let after_aoki = aoki.1 + if a_dir == 0 { -times } else { times };
                let pos = (aoki.0, takahashi.1);
                if check_point_in_kukan(
                        takahashi.0.min(after_takahashi),
                        takahashi.0.max(after_takahashi),
                        aoki.0
                    ) &&
                    check_point_in_kukan(
                        aoki.1.min(after_aoki),
                        aoki.1.max(after_aoki),
                        takahashi.1
                    ) &&
                    (pos.0 - takahashi.0).abs() == (pos.1 - aoki.1).abs() &&
                    takahashi != aoki
                    {
                        ans += 1;
                    }
                takahashi.0 = after_takahashi;
                aoki.1 = after_aoki;
            }
        }
    }
    return ans;
}


#[allow(dead_code)]
fn naive(ip: &Ins) -> i64 {
    let mut takahashi = ip.takahashi;
    let mut aoki = ip.aoki;
    let (_n, _m, _l) = (ip.n, ip.m, ip.l);
    
    let mut t_que = VecDeque::new();
    let mut a_que = VecDeque::new();
    
    for (c, cnt) in ip.tv.iter() {
        for _ in 0..*cnt {
            t_que.push_back(*c);
        }
    }

    for (c, cnt) in ip.av.iter() {
        for _ in 0..*cnt {
            a_que.push_back(*c);
        }
    }

    let mut k = 0;
    let mut ans = 0;

    while !t_que.is_empty() {
        k += 1;
        let t_dir = t_que.pop_front().unwrap();
        let a_dir = a_que.pop_front().unwrap();

        match t_dir {
            'L' => takahashi.1 -= 1,
            'R' => takahashi.1 += 1,
            'U' => takahashi.0 -= 1,
            'D' => takahashi.0 += 1,
            _ => unreachable!(),
        }
        
        match a_dir {
            'L' => aoki.1 -= 1,
            'R' => aoki.1 += 1,
            'U' => aoki.0 -= 1,
            'D' => aoki.0 += 1,
            _ => unreachable!(),
        }
        
        if aoki == takahashi {
            ans += 1;
            dbg!(k);
            dbg!(takahashi);
        }
    }
    return ans;
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
        let ins = Ins::new(&mut ip);
        println!("{}", solve(&ins));
    }
    
    // loop {
    //     let ins = Ins::gen();
    //     if solve(&ins) != naive(&ins) {
    //         println!("WA");
    //         println!("{} {} {} {}", ins.takahashi.0, ins.takahashi.1, ins.aoki.0, ins.aoki.1);
    //         println!("{} {} {}", ins.n, ins.m, ins.l);
    //         for i in 0..ins.m {
    //             println!("{} {}", ins.tv[i].0, ins.tv[i].1);
    //         }

    //         for i in 0..ins.l {
    //             println!("{} {}", ins.av[i].0, ins.av[i].1);
    //         }
    //         println!("solve: {}", solve(&ins));
    //         println!("naive: {}", naive(&ins));
    //         break;
    //     } else {
    //     }

    // }

}
