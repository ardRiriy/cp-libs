use itertools::Itertools;
use proconio::{input, marker::Usize1};
fn main() {
    input!{
        n: usize,
        k: usize,
        x: [Usize1; n],
        a: [u64 ; n]
    }

    let mut visited = vec![false; n];
    let mut res = vec![0; n];

    for s in 0..n {
        if visited[s] {
            continue;
        }

        let mut current = s;
        let mut trace = vec![];
        let mut indices = vec![0; n];

        while !visited[current] {
            visited[current] = true;
            indices[current] = trace.len();
            trace.push(current);
            current = x[current];
        }

        let cycle_start = indices[current];
        let cycle_length = trace.len() - cycle_start;
        println!("{:?}", trace);
        for i in 0..trace.len() {
            let pos = if i < cycle_start {
                if (k as usize) < i {
                    trace[k as usize]
                } else {
                    trace[(k - i) % cycle_length]
                }
            } else {
                trace[((k % cycle_length) + i) % cycle_length]
            };
            res[trace[i]] = a[pos];
        }
    }

    println!("{}", res.iter().join(" "));
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
