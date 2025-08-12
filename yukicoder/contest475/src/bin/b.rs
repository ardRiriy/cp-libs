use library::utils::input::Input;

fn main() {
    let mut input = Input::new();
    let _n = input.next::<usize>();
    let s = input.next::<String>();

    let mut ans = vec![0, 0];
    for c in s.chars() {
        if c=='-' {
            ans[0] += 1;
        } else if c=='0' {
            ans[1] = ans[1].max(ans[0] + 1);
        } else {
            ans[1] = (ans[1]+1).max(ans[0]+1);
        }
    }
    println!("{}", ans.iter().max().unwrap());
}
