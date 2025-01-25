use proconio::input;

/*
f(L, R)を削除するときに、
a_iを下端として削除する回数(a_iの寄与)を考える。

a_iを下端として削除する回数は、
j<iかつa_j=a_i - 1なるjの最大値(存在しなければ-1), 
k>iかつa_k=a_i - 1なるkの最小値(存在しなければn)を用いて(i-j)*(k-i)で求められる。

ただし、
3 4 4 3
のような場合、4 4を重複して数えてしまうので、
左端についてはl<iかつa_l=a_iなるlの最大値(存在しなければ-1)を求めて、
min(i-j, i-l)*(k-i)で求められる。
*/

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut v = vec![vec![]; n+1];
    let mut idxes = vec![None; n+1];
    for (i, &ai) in a.iter().enumerate() {
        v[ai].push(i);
        if idxes[ai].is_none() {
            idxes[ai] = Some(0);
        }
    }

    let mut ans = 0;
    let mut last_seen = vec![None; n+1];
    for (i, &ai) in a.iter().enumerate() {
        let mut l = if let Some(idx) = last_seen[ai-1] {
            idx as i64
        }  else {
            -1
        };
        l = l.max(
            if let Some(idx) = last_seen[ai] {
                idx
            } else {
                -1
            }
        );
        let r = if let Some(idx) = idxes[ai-1] {
            v[ai-1][idx] as i64
        } else {
            n as i64
        };

        if idxes[ai].unwrap() + 1 == v[ai].len() {
            idxes[ai] = None;
        } else {
            idxes[ai] = Some(idxes[ai].unwrap()+1);
        }

        last_seen[ai] = Some(i as i64);
        ans += (i as i64 -l) * (r-i as i64);

    }
    println!("{}", ans);
}

