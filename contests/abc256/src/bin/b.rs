use proconio::{input};
fn main() {
    input!{
        n: usize,
        a: [usize; n]
    }
    let mut mass = [false; 4];
    let mut res = 0;

    for &x in a.iter() {
        mass[0] = true;

        for i in (0..4).rev() {
            if !mass[i] {
                continue;
            }
            mass[i] = false;
            if i + x >= 4 {
                res += 1;
            } else {
                mass[i+x] = true;
            }
        }
    }
    println!("{}", res);
}
