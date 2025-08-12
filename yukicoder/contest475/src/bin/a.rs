use library::utils::input::Input;

fn main() {
    let mut input = Input::new();
    let (a, b) = input.pair::<u64,u64>();
    println!("{} {}", 10-a, b+a-10);
}
