static INF: usize = 1e18 as usize;
use num_traits::Num;

struct SegmentTree<T> where T: From<usize> + Num + Copy + Ord, {
    n: usize,   // 要素数
    dat: Vec<T> // 実際のデータ
}

/* RangeMinQuery。isize or usizeでなければオーバーフローするので注意*/
impl<T> SegmentTree<T> where T:From<usize> + Num + Copy + Ord, {
    pub fn new(n: usize, data: &Vec<T>) -> SegmentTree<T> {
        let mut x = 1;
        while n > x {
            x *= 2;
        }

        let size = 2 * x - 1;
        let mut dat :Vec<T> = vec![T::from(INF); size];
        for i in 0..n {
            dat[size - n + i] = data[i];
        }

        SegmentTree { n: x, dat }
    }

    /* 配列のidx番目の値を更新する */
    pub fn update(&mut self, mut idx: usize, x: T) {
        idx += self.n - 1; // セグ木上でのインデックスに変換
        self.dat[idx] = x;

        while idx > 0 {   // 親ノードの更新
            idx = (idx - 1) / 2;
            self.dat[idx] = self.evaluate(self.dat[idx * 2 + 1], self.dat[idx * 2 + 2]);
        }
    }

    /* 半開区間[a, b)で値を取得する */
    pub fn query(&self, a: usize, b: usize) -> T {
        self.sub_query(a, b, 0, 0, self.n)
    }

    fn sub_query(&self, a: usize, b: usize, node: usize, l: usize, r: usize) -> T {
        return if r <= a || b <= l {
            // 範囲外の処理
            T::from(INF)
        } else if a <= l && r <= b {
            // 範囲内ならば今のノードを返す
            self.dat[node]
        } else {
            // 一部区間がかぶるなら半分に分割してそれぞれの最小値を得る
            let mid = (l + r) / 2;
            let vl = self.sub_query(a, b, node * 2 + 1, l, mid);
            let vr = self.sub_query(a, b, node * 2 + 2, mid, r);

            self.evaluate(vl, vr)
        }
    }

    fn evaluate(&self, a: T, b: T) -> T {
        a.min(b)
    }
}
