use proconio::input;

fn main() {
    input!{
        d: String
    }
    let v = vec!["N", "NE", "E", "SE", "S", "SW", "W", "NW"];
    let i = v.iter().position(|&s| s.to_string() == d).unwrap();
    println!("{}", v[(i+4)%v.len()]);
}
