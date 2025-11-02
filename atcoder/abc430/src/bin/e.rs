use library::{strings::rollinghash::RollingHash, utils::input::Input};

fn solve(ip: &mut Input) {
    let mut a = ip.next::<String>();
    let b = ip.next::<String>();
    a.push_str(&a.clone());

    let n = b.len();
    let ra = RollingHash::new(&a, None);
    let rb = RollingHash::new(&b, Some(ra.base));

    for i in 0..n {
        if ra.hash(i..i + n) == rb.hash(0..n) {
            println!("{}", i);
            return;
        }
    }
    println!("-1");
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
