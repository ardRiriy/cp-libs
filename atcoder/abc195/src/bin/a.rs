use library::utils::input::Input;

fn main() {
    let mut input = Input::new();
    let (m, h) = input.pair::<u64,u64>();
    println!("{}", if h % m == 0 { "Yes" } else { "No" });
}
