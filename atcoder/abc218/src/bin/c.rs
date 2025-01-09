use itertools::{iproduct, Itertools};
use proconio::{input, marker::Chars};
fn main() {
    input! {
        n: usize,
        s: [Chars; n],
        mut t: [Chars; n]
    }

    let sp = get_pos(&s);

    let jg = |v: &Vec<(i64, i64)>| -> bool {
        let first = v[0];
        v.iter().all(|&x| x == first)
    };

    for _ in 0..4 {
        let tp = get_pos(&t);
        if sp.len() != tp.len() { break; }

        let ans = tp.iter()
            .enumerate()
            .map(|(idx, x)| (sp[idx].0 - x.0, sp[idx].1 - x.1))
            .collect_vec();

        if jg(&ans) {
            println!("Yes");
            return;
        }

        t = rotate(&t);
    }

    println!("No");
}

fn get_pos(matrix: &Vec<Vec<char>>) -> Vec<(i64, i64)> {
    let mut res = vec![];
    let n = matrix.len();
    for (i, j) in iproduct!(0..n, 0..n) {
        if matrix[i][j] == '#' {
            res.push((i as i64, j as i64));
        }
    }
    res
}

fn rotate(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n = matrix.len();
    let mut rotated_matrix = vec![vec!['.'; n]; n];

    for i in 0..n {
        for j in 0..n {
            rotated_matrix[j][n - 1 - i] = matrix[i][j];
        }
    }

    rotated_matrix
}
