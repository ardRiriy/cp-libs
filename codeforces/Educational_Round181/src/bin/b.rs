use library::utils::input::Input;

fn main() {
    let mut input = Input::new();
    let t = input.next::<usize>();
    for _ in 0..t {
        solve(&mut input);
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn solve(input: &mut Input) {
    let (a,b,k) = input.triple::<u64,u64,u64>();

    let gcd_ab = gcd(a, b);
    println!("{}", if a/gcd_ab <= k && b/gcd_ab <= k {
        1
    } else {
        2
    });
}

