use proconio::{input, marker::Usize1};

fn compute_3d_cumulative_sum(arr: &Vec<Vec<Vec<i64>>>) -> Vec<Vec<Vec<i64>>> {
    let x_size = arr.len();
    let y_size = arr[0].len();
    let z_size = arr[0][0].len();

    let mut cum_sum = vec![vec![vec![0; z_size]; y_size]; x_size];

    for x in 0..x_size {
        for y in 0..y_size {
            for z in 0..z_size {
                let mut sum = arr[x][y][z];

                if x > 0 {
                    sum += cum_sum[x - 1][y][z];
                }
                if y > 0 {
                    sum += cum_sum[x][y - 1][z];
                }
                if z > 0 {
                    sum += cum_sum[x][y][z - 1];
                }
                if x > 0 && y > 0 {
                    sum -= cum_sum[x - 1][y - 1][z];
                }
                if y > 0 && z > 0 {
                    sum -= cum_sum[x][y - 1][z - 1];
                }
                if z > 0 && x > 0 {
                    sum -= cum_sum[x - 1][y][z - 1];
                }
                if x > 0 && y > 0 && z > 0 {
                    sum += cum_sum[x - 1][y - 1][z - 1];
                }

                cum_sum[x][y][z] = sum;
            }
        }
    }

    cum_sum
}

fn compute_region_sum(
    cum_sum: &Vec<Vec<Vec<i64>>>,
    x1: usize, y1: usize, z1: usize,
    x2: usize, y2: usize, z2: usize,
) -> i64 {
    let mut total = cum_sum[x2][y2][z2];

    if x1 > 0 {
        total -= cum_sum[x1 - 1][y2][z2];
    }
    if y1 > 0 {
        total -= cum_sum[x2][y1 - 1][z2];
    }
    if z1 > 0 {
        total -= cum_sum[x2][y2][z1 - 1];
    }
    if x1 > 0 && y1 > 0 {
        total += cum_sum[x1 - 1][y1 - 1][z2];
    }
    if y1 > 0 && z1 > 0 {
        total += cum_sum[x2][y1 - 1][z1 - 1];
    }
    if z1 > 0 && x1 > 0 {
        total += cum_sum[x1 - 1][y2][z1 - 1];
    }
    if x1 > 0 && y1 > 0 && z1 > 0 {
        total -= cum_sum[x1 - 1][y1 - 1][z1 - 1];
    }

    total
}

fn solve() {
    input!{
        n: usize,
        a: [[[i64; n]; n]; n],
        q: usize,
    }

    let cum_sum = compute_3d_cumulative_sum(&a);
    for _ in 0..q {
        input! {
            x1: Usize1,
            x2: Usize1,
            y1: Usize1,
            y2: Usize1,
            z1: Usize1,
            z2: Usize1,
        }

        let ans = compute_region_sum(&cum_sum, x1, y1, z1, x2, y2, z2);
        println!("{ans}");
    }
}

/*
鹿のardririy, arDeeriy
　　　　　　　　　　　　 　 　 　 　 　 　 ／
　　　　　　　　　　　　　　　　　 　 　 //
　　　　　　　　　 　 　 　 　 　 　 　 //　 　 　 　 |:
　　　　　　　　　　　　　　　　　 　 // 　 　 　 　 .|i
　　　　　　　　　 　 　 　 　 　 　 //　 　 　 　 　 ||
　　　　　　　　　　　　　　　　　　l i　　　　　　 　 ||
　　　　　.　´￣￣｀ｰ　、　　　　 | l　 　 　 　 　 ∥
　　　 ／　 . 　 　 　_＞─- ､ 　 l |　　　　　　 .∥
　　　i　 〆　 　 ／　 　 　 　 ＼.i | 　 　 　 　 ∥
　　　| /　 　 ／ 　 　 　 　 　 　 l |　ﾊ　　　　.′
　 　 ; :　 　 .′　　　　　　　　　 ヾゝi !　　　/.′
　　 《　　　 :　　　 　 　 r─-､　　 i ﾘ:l　　 //　　　　 　 　 、
　 　 |　　　　 　 　 　 　 ｀ヾ.　＼ r‐’:ﾚ=‐' .ゝ...ノ＿＿＼＿))
　　 弋　 　 　 　 　 　 　 　 ∨ソ`ー''`ー---一　‐─‐-､)¨´
　　 　 } 、　　　　　　　　　　 i_..ノ _,　　　　ハ'
　　 　 ∨＞､　iゝ.. 　 　 　 　 　 弋);;,ゞ..ノ　.′
　　　　 ∨::::| 7!:::..ヾ.　　　　.′ヽ.　` 　 　 /
　　　　　∨::|.′::::::ixxr, 　 /　　　:　　　　 I.
　 　 　 　 }:::|..:ｉ..::::::|　i,,　　"'' ´ヾ.i 　 　 　 ﾊ
　　　　 　 ::::|:::|::::::/　 }ヾ.　　　　ﾐゝ.　 ,、..:::::)
　　　　 　 i:::|ヾ:::::i　 /.::::|｀ヾ..,,　..ノ＼ヾ..＿/
　 　 　 　 |:::l　}:. {　 |::::..′　　　　　　 `ー''
　 　 　 　 |:::| /.:/l　 !.:::l
　　　　　 ﾉ..:ﾚ.:::ｉ::l　ﾉ..::|
　　　 　 (人7.::::|:::V.::::::|
　　　　　`''/ .:::::!ｰ:::::::::ﾊ
　　　　　 /..::::::::| (_/＼__)
　　　 　 (__/(,＿)
*/

static INF: u64 = 1e18 as u64;

trait ChLibs<T: std::cmp::Ord> {
    fn chmin(&mut self, elm: T) -> bool;
    fn chmax(&mut self, elm: T) -> bool;
}

impl<T: std::cmp::Ord> ChLibs<T> for T {
    fn chmin(&mut self, elm: T) -> bool {
        if *self > elm {
            *self = elm;
            true
        } else {
            false
        }
    }

    fn chmax(&mut self, elm: T) -> bool {
        if *self < elm {
            *self = elm;
            true
        } else {
            false
        }
    }
}

fn main() {
    // input! { mut i: usize }
    let mut i = 1;
    while i != 0 {
        solve();
        i -= 1;
    }
}
