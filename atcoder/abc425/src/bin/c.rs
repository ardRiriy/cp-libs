use library::{cumulative_sum::CumulativeSum, utils::input::Input};

fn solve(ip: &mut Input) {
    let (n, q) = ip.pair();
    let a = ip.vector::<u64>(n);
    let csum = CumulativeSum::new(&a);
    let mut offset = 0;

    for _ in 0..q {
        let t = ip.next::<i32>();
        if t == 1 {
            let c = ip.next::<usize>();
            offset += c;
            offset %= n;
        } else {
            let (l, r) = ip.pair::<usize>();
            let l = l - 1;
            let r = r - 1;
            
            let ol = (l + offset) % n;
            let or = (r + offset) % n;
            
            if ol <= or {
                println!("{}", csum.get(ol..=or));
            } else {
                println!("{}", csum.get(ol..) + csum.get(..=or));
            }
        }

    }

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
