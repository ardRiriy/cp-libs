use proconio::{input};
fn main() {
    input!{
        c1: char,
        c2: char,
        c3: char,
    }

    let mut age = [0; 3];
    if c1 == '>' {
        age[0] += 1;
    } else {
        age[1] += 1;
    }
    if c2 == '>' {
        age[0] += 1;
    } else {
        age[2] += 1;
    }
    if c3 == '>' {
        age[1] += 1;
    } else {
        age[2] += 1;
    }

    let secont = if age[0] == 1 {
        'A'
    } else if age[1] == 1 {
        'B'
    } else {
        'C'
    };
    println!("{secont}");
}
