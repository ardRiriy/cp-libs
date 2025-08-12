use library::utils::input::Input;

fn main() {
    let mut input = Input::new();
    let t = input.next::<usize>();
    for _ in 0..t {
        solve(&mut input);
    }
}


fn solve(input: &mut Input) {
    let s = input.next::<String>();
    let mut t = s.chars().collect::<Vec<char>>();
    t.sort();
    t.reverse();
    println!("{}", t.iter().collect::<String>());
}