use itertools::{iproduct, Itertools};
use proconio::input;
use proconio::marker::Chars;

mod data_structure;
mod simulated_annealing;
mod utils;

static TIME_LIMIT :f64 = 1.96;
static BEAM_WIDTH :usize = 20;

#[inline]
pub fn get_time() -> f64 {  // sec
    static mut STIME: f64 = -1.0;
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    let ms = t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9;
    unsafe {
        if STIME < 0.0 {
            STIME = ms;
        }
        ms - STIME
    }
}

#[derive(Debug, Clone)]
struct Board {
    c: Vec<Vec<char>>,
    ans: Vec<(char, usize)>,
}

impl Board {
    fn new(c: Vec<Vec<char>>, ans: Vec<(char, usize)>) -> Self {
        Self {
            c,
            ans,
        }
    }

    fn score(&self) -> f64 {
        if self.ans.len() == 0 {
            return 0.0;
        }
        // 落とした駒の数 / ターン数 で表す
        let left = self.c.iter()
            .map(|row| row.iter().filter(|&&c| c == 'x').count())
            .sum::<usize>();
        (self.c.len() * 2 - left) as f64 / self.ans.len() as f64
    }
}

fn main() {
    input! {
        n: usize,
        mut c: [Chars; n],
    }
    get_time();

    let mut ans = vec![];

    // 良い順にDFSする
    let mut beam = vec![vec![]; BEAM_WIDTH];
    let mut cur = 0;
    beam[cur].push(Board::new(c.clone(), vec![]));
    while get_time() < TIME_LIMIT {
        // beam[cur]の中で、いいスコア順にBEAM_WIDTH個選ぶ
        beam[cur].sort_by(|a, b| b.score().partial_cmp(&a.score()).unwrap());
        for i in 0..BEAM_WIDTH.min(beam[cur].len()) {
            let cur_board = beam[cur][i].c.clone();
            let cur_ans = beam[cur][i].ans.clone();
            let mut operates = vec![];

            for idx in 0..n {
                // idx行目のコマを左に出す想定
                let mut last_find_idx = 0; // 最後に見つけたindex
                let mut tremove = 0;
                let mut tcost = 1;
                let mut ops = vec![]; // (方向, idx, 回数) の配列

                for j in 0..n {
                    // j列目のコマを下にやったときに、必要な操作回数(不可能ならINF)
                    if cur_board[idx][j] == 'o' {
                        // 福なのでこれ以上は出せない
                        break;
                    }

                    if cur_board[idx][j] == 'x' {
                        // 操作の必要なし
                        tremove += 1;
                        last_find_idx = j;
                        continue;
                    }

                    // 上に押すか下に押すかでどちらか選ぶ
                    // 選択できることが優先で、上下どちらでも選択できる場合は
                    // 移動距離が小さい方を選ぶ
                    let mut tmp_remove = 0;
                    let mut tmp_cost = 10000000;
                    let mut tmp_dir = '@';
                    // 上に押し上げる場合
                    for i in idx + 1..n {
                        if cur_board[i][j] == 'x' {
                            // 上に(i-idx)マス押すので、そこに福があるとダメ
                            let mut flag = true;
                            let mut cnt = 0;
                            for k in 0..i - idx {
                                if cur_board[k][j] == 'o' {
                                    flag = false;
                                    break;
                                } else if cur_board[k][j] == 'x' {
                                    cnt += 1;
                                }
                            }
                            if flag {
                                if tmp_cost > i - idx {
                                    tmp_cost = i - idx;
                                    tmp_remove = cnt + 1; // 上に押したときに消すコマの数+左に押して消すコマ
                                    tmp_dir = 'U';
                                }
                            }
                            break;
                        }
                    }
                    // 下に押し下げる場合
                    for i in (0..idx).rev() {
                        if cur_board[i][j] == 'x' {
                            // 下に(idx-i)マス押すので、そこに福があるとダメ
                            let mut flag = true;
                            let mut cnt = 0;
                            for k in n - (idx - i)..n {
                                if cur_board[k][j] == 'o' {
                                    flag = false;
                                    break;
                                } else if cur_board[k][j] == 'x' {
                                    cnt += 1;
                                }
                            }
                            if flag {
                                if tmp_cost > idx - i {
                                    tmp_cost = idx - i;
                                    tmp_remove = cnt + 1; // 下に押したときに消すコマの数+左に押して消すコマ
                                    tmp_dir = 'D';
                                }
                            }
                            break;
                        }
                    }

                    if tmp_dir != '@' {
                        tremove += tmp_remove;
                        tcost += tmp_cost;
                        ops.push((tmp_dir, j, tmp_cost));
                        last_find_idx = j;
                    }
                }

                // 効率がより良いならば更新
                // if remove as f64 / (cost as f64).sqrt() < tremove as f64 / (tcost as f64).sqrt() {
                //     remove = tremove;
                //     cost = tcost;
                //     operates = ops;
                //     operates.push(('L', idx, last_find_idx + 1));
                // }
                ops.push(('L', idx, last_find_idx + 1));
                operates.push((tremove as f64 / (tcost as f64).sqrt(), ops));



                // idx行目のコマをに右に出す想定
                let mut last_find_idx = n; // 最後に見つけたindex
                let mut tremove = 0;
                let mut tcost = 1;
                let mut ops = vec![]; // (方向, idx, 回数) の配列

                for j in (0..n).rev() {
                    if cur_board[idx][j] == 'o' {
                        // 福なのでこれ以上は出せない
                        break;
                    }

                    if cur_board[idx][j] == 'x' {
                        // 操作の必要なし
                        tremove += 1;
                        last_find_idx = j;
                        continue;
                    }

                    // 上に押すか下に押すかでどちらか選ぶ
                    // 選択できることが優先で、上下どちらでも選択できる場合は
                    // 移動距離が小さい方を選ぶ
                    let mut tmp_remove = 0;
                    let mut tmp_cost = 10000000;
                    let mut tmp_dir = '@';
                    // 上に押し上げる場合

                    for i in idx + 1..n {
                        if cur_board[i][j] == 'x' {
                            // 上に(i-idx)マス押すので、そこに福があるとダメ
                            let mut flag = true;
                            let mut cnt = 0;
                            for k in 0..i - idx {
                                if cur_board[k][j] == 'o' {
                                    flag = false;
                                    break;
                                } else if cur_board[k][j] == 'x' {
                                    cnt += 1;
                                }
                            }
                            if flag {
                                if tmp_cost > i - idx {
                                    tmp_cost = i - idx;
                                    tmp_remove = cnt + 1; // 上に押したときに消すコマの数+左に押して消すコマ
                                    tmp_dir = 'U';
                                }
                            }
                            break;
                        }
                    }

                    // 下に押し下げる場合
                    for i in (0..idx).rev() {
                        if cur_board[i][j] == 'x' {
                            // 下に(idx-i)マス押すので、そこに福があるとダメ
                            let mut flag = true;
                            let mut cnt = 0;
                            for k in n - (idx - i)..n {
                                if cur_board[k][j] == 'o' {
                                    flag = false;
                                    break;
                                } else if cur_board[k][j] == 'x' {
                                    cnt += 1;
                                }
                            }
                            if flag {
                                if tmp_cost > idx - i {
                                    tmp_cost = idx - i;
                                    tmp_remove = cnt + 1; // 下に押したときに消すコマの数+左に押して消すコマ
                                    tmp_dir = 'D';
                                }
                            }
                            break;
                        }
                    }

                    if tmp_dir != '@' {
                        tremove += tmp_remove;
                        tcost += tmp_cost;
                        ops.push((tmp_dir, j, tmp_cost));
                        last_find_idx = j;
                    }
                }

                ops.push(('R', idx, n - last_find_idx));
                operates.push((tremove as f64 / (tcost as f64).sqrt(), ops));

                // idx列目のコマを上に出す想定
                let mut last_find_idx = 0; // 最後に見つけたindex
                let mut tremove = 0;
                let mut tcost = 1;
                let mut ops = vec![]; // (方向, idx, 回数) の配列

                for i in 0..n {
                    if cur_board[i][idx] == 'o' {
                        // 福なのでこれ以上は出せない
                        break;
                    }

                    if cur_board[i][idx] == 'x' {
                        // 操作の必要なし
                        tremove += 1;
                        last_find_idx = i;
                        continue;
                    }

                    // 左に押すか右に押すかでどちらか選ぶ
                    // 選択できることが優先で、左右どちらでも選択できる場合は
                    // 移動距離が小さい方を選ぶ
                    let mut tmp_remove = 0;
                    let mut tmp_cost = 10000000;
                    let mut tmp_dir = '@';
                    // 左に押し上げる場合
                    for j in idx + 1..n {
                        if cur_board[i][j] == 'x' {
                            // 左に(j-idx)マス押すので、そこに福があるとダメ
                            let mut flag = true;
                            let mut cnt = 0;
                            for k in 0..j - idx {
                                if cur_board[i][k] == 'o' {
                                    flag = false;
                                    break;
                                } else if cur_board[i][k] == 'x' {
                                    cnt += 1;
                                }
                            }
                            if flag {
                                if tmp_cost > j - idx {
                                    tmp_cost = j - idx;
                                    tmp_remove = cnt + 1; // 左に押したときに消すコマの数+上に押して消すコマ
                                    tmp_dir = 'L';
                                }
                            }
                            break;
                        }
                    }

                    // 右に押し下げる場合
                    for j in (0..idx).rev() {
                        if cur_board[i][j] == 'x' {
                            // 右に(idx-j)マス押すので、そこに福があるとダメ
                            let mut flag = true;
                            let mut cnt = 0;
                            for k in n - (idx - j)..n {
                                if cur_board[i][k] == 'o' {
                                    flag = false;
                                    break;
                                } else if cur_board[i][k] == 'x' {
                                    cnt += 1;
                                }
                            }
                            if flag {
                                if tmp_cost > idx - j {
                                    tmp_cost = idx - j;
                                    tmp_remove = cnt + 1; // 右に押したときに消すコマの数+上に押して消すコマ
                                    tmp_dir = 'R';
                                }
                            }
                            break;
                        }
                    }

                    if tmp_dir != '@' {
                        tremove += tmp_remove;
                        tcost += tmp_cost;
                        ops.push((tmp_dir, i, tmp_cost));
                        last_find_idx = i;
                    }
                }

                ops.push(('U', idx, last_find_idx + 1));
                operates.push((tremove as f64 / (tcost as f64).sqrt(), ops));

                // idx列目のコマを下に出す想定
                let mut last_find_idx = n; // 最後に見つけたindex
                let mut tremove = 0;
                let mut tcost = 1;
                let mut ops = vec![]; // (方向, idx, 回数) の配列

                for i in (0..n).rev() {
                    if cur_board[i][idx] == 'o' {
                        // 福なのでこれ以上は出せない
                        break;
                    }

                    if cur_board[i][idx] == 'x' {
                        // 操作の必要なし
                        tremove += 1;
                        last_find_idx = i;
                        continue;
                    }

                    // 左に押すか右に押すかでどちらか選ぶ
                    // 選択できることが優先で、左右どちらでも選択できる場合は
                    // 移動距離が小さい方を選ぶ
                    let mut tmp_remove = 0;
                    let mut tmp_cost = 10000000;
                    let mut tmp_dir = '@';
                    // 左に押し上げる場合
                    for j in idx + 1..n {
                        if cur_board[i][j] == 'x' {
                            // 左に(j-idx)マス押すので、そこに福があるとダメ
                            let mut flag = true;
                            let mut cnt = 0;
                            for k in 0..j - idx {
                                if cur_board[i][k] == 'o' {
                                    flag = false;
                                    break;
                                } else if cur_board[i][k] == 'x' {
                                    cnt += 1;
                                }
                            }
                            if flag {
                                if tmp_cost > j - idx {
                                    tmp_cost = j - idx;
                                    tmp_remove = cnt + 1; // 左に押したときに消すコマの数+上に押して消すコマ
                                    tmp_dir = 'L';
                                }
                            }
                            break;
                        }
                    }

                    // 右に押し下げる場合
                    for j in (0..idx).rev() {
                        if cur_board[i][j] == 'x' {
                            // 右に(idx-j)マス押すので、そこに福があるとダメ
                            let mut flag = true;
                            let mut cnt = 0;
                            for k in n - (idx - j)..n {
                                if cur_board[i][k] == 'o' {
                                    flag = false;
                                    break;
                                } else if cur_board[i][k] == 'x' {
                                    cnt += 1;
                                }
                            }
                            if flag {
                                if tmp_cost > idx - j {
                                    tmp_cost = idx - j;
                                    tmp_remove = cnt + 1; // 右に押したときに消すコマの数+上に押して消すコマ
                                    tmp_dir = 'R';
                                }
                            }
                            break;
                        }
                    }

                    if tmp_dir != '@' {
                        tremove += tmp_remove;
                        tcost += tmp_cost;
                        ops.push((tmp_dir, i, tmp_cost));
                        last_find_idx = i;
                    }
                }

                ops.push(('D', idx, n - last_find_idx));
                operates.push((tremove as f64 / (tcost as f64).sqrt(), ops));
            }

            // opsを効率の高い順にソート(indexだけ取る)
            let indicates = (0..operates.len()).sorted_unstable_by(|i, j| operates[*j].0.partial_cmp(&operates[*i].0).unwrap()).collect_vec();

            for k in indicates[0..BEAM_WIDTH.min(operates.len())].iter().rev() {
                let mut new_board = cur_board.clone();
                let mut new_ans = cur_ans.clone();
                for &(dir, idx, cnt) in &operates[*k].1 {
                    for _ in 0..cnt {
                        new_ans.push((dir, idx));
                    }
                    match dir {
                        'U' => {
                            // c[i][idx]を上にcnt回押す
                            for i in 0..n-cnt {
                                new_board[i][idx] = new_board[i + cnt][idx];
                            }
                            for i in n-cnt..n {
                                new_board[i][idx] = '.';
                            }
                        },
                        'D' => {
                            // c[i][idx]を下にcnt回押す
                            for i in (0..n-cnt).rev() {
                                new_board[i + cnt][idx] = new_board[i][idx];
                            }
                            for i in 0..cnt {
                                new_board[i][idx] = '.';
                            }
                        },
                        'L' => {
                            // c[idx][j]を左にcnt回押す
                            for j in 0..n-cnt {
                                new_board[idx][j] = new_board[idx][j + cnt];
                            }
                            for j in n-cnt..n {
                                new_board[idx][j] = '.';
                            }
                        },
                        'R' => {
                            // c[idx][j]を右にcnt回押す
                            for j in (0..n-cnt).rev() {
                                new_board[idx][j + cnt] = new_board[idx][j];
                            }
                            for j in 0..cnt {
                                new_board[idx][j] = '.';
                            }
                        },
                        _ => {
                            panic!("Invalid direction");
                        }
                    }
                }
                // もし盤面から'x'が消えていたら、答えの更新を確認
                if new_board.iter().flatten().any(|&c| c == 'x') {
                    beam[cur ^ 1].push(Board::new(new_board, new_ans));
                } else {
                    if ans.is_empty() || ans.len() > new_ans.len() {
                        ans = new_ans;
                    }
                }
            }
        }
        beam[cur].clear();
        cur ^= 1;
    }

    for (dir, idx) in ans {
        println!("{} {}", dir, idx);
    }
}