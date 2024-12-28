use proconio::input;

fn main() {
    input!{
        y: u64,
    }

    if y % 4 != 0 {
        println!("365");
    } else if y % 100 != 0 {
        println!("366");
    } else if y % 400 != 0 {
        println!("365");
    } else {
        println!("366");
    }
}
