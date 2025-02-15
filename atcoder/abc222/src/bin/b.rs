use proconio::input;
fn main() {
    input!{
        n: usize,
        p: i32,
        a: [i32; n],
    }
    println!("{}", a.iter().filter(|&ai| *ai < p).count());
}
