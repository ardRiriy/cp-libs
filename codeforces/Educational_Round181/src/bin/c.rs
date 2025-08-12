use library::utils::input::Input;

fn main() {
    let mut input = Input::new();
    let t = input.next::<usize>();
    for _ in 0..t {
        solve(&mut input);
    }
}

fn solve(input: &mut Input) {
    let (l, r) = input.pair::<usize, usize>();
    let mut ans = 0;
    ans += get(r, 2) - get(l - 1, 2);
    ans += get(r, 3) - get(l - 1, 3);
    ans += get(r, 5) - get(l - 1, 5);
    ans += get(r, 7) - get(l - 1, 7);

    ans += get(r, 2*3*5) - get(l - 1, 2*3*5);
    ans += get(r, 2*3*7) - get(l - 1, 2*3*7);
    ans += get(r, 2*5*7) - get(l - 1, 2*5*7);
    ans += get(r, 3*5*7) - get(l - 1, 3*5*7);

    ans -= get(r,6) - get(l - 1, 6);
    ans -= get(r, 10) - get(l - 1, 10);
    ans -= get(r, 14) - get(l - 1, 14);
    ans -= get(r, 15) - get(l - 1, 15);
    ans -= get(r, 21) - get(l - 1, 21);
    ans -= get(r, 35) - get(l - 1, 35);

    ans -= get(r, 2*3*5*7) - get(l - 1, 2*3*5*7);
    println!("{}", r-l+1-ans);
}

fn get(n: usize, k: usize) -> usize {
    // N以下の値であって、kの倍数の個数
    n / k
}

