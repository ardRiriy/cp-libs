use itertools::Itertools;
use proconio::input;

fn check1(x: usize, y: usize, area: &Vec<usize>) -> bool {
    // 横xで固定したときの、高さの和がy以下か？
    area.iter()
        .map(|s| *s / x + if *s % x == 0 { 0 } else { 1 })
        .sum::<usize>() <= y
}

fn check2(x: usize, y: usize, area: &Vec<usize>) -> bool {
    // area[0]を右においたときに、のこりの2つが収まるか？
    let used_x = area[0] / y + if area[0] % y == 0 { 0 } else { 1 };
    if x <= used_x {
        return false;
    }

    let left_x = x - used_x;

    area.iter()
        .skip(1)
        .map(|s| *s / left_x + if *s % left_x == 0 { 0 } else { 1 })
        .sum::<usize>() <= y
}

fn main() {
    input!{
        x: usize,
        y: usize,
        area: [usize; 3]
    }
    if check1(x, y, &area) || 
        check1(y, x, &area) ||
        area.iter().copied().permutations(3).any(|v| check2(x, y, &v) || check2(y, x, &v)) {
            println!("Yes");
        } else {
            println!("No");
        }
}

