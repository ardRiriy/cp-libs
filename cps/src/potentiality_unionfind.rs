pub struct PotentialityUnionfind<T> {
    vertex: Vec<usize>,
    diff_weights: Vec<T>, 
}

impl<T: num::PrimInt> PotentialityUnionfind<T> {
    pub fn new(size: usize) -> Self {
        Self {
            vertex: vec![!1; size],
            diff_weights: vec![T::zero(); size],
        }
    }

    pub fn leader(&mut self, u: usize) -> usize {
        let elm = self.vertex[u];
        if elm > self.vertex.len() {
            u
        } else {
            let parent = self.leader(elm);
            // TODO: ここの結合則は後から変えられるようにする 
            self.diff_weights[u] = self.diff_weights[u] + self.diff_weights[self.vertex[u]];
            self.vertex[u] = parent;
            self.vertex[u]
        }
    }

    pub fn same(&mut self, u: usize, v: usize) -> bool {
        self.leader(u) == self.leader(v)
    }

    // マージして重みを返す
    // すでに同じ場合は「既存の情報と矛盾しないか？」を判定して、
    // 矛盾する場合はErrを返す
    pub fn merge(&mut self, u: usize, v: usize, w: T) -> Result<T, T> {
        if let Ok(diff) = self.diff(u, v) {
            return if w == diff {
                Ok(diff)
            } else {
                Err(diff)
            }
        }

        // size(u) >= size(v) となるようにswap
        if self.size(u) < self.size(v) {
            self.merge(v, u, w)?;
            return Ok(w);
        }

        // 重みの差分を修正(ここも結合則によって変わる)
        let w_sub = w + self.weight(u) - self.weight(v);

        // vをuにマージ
        let u_leader = self.leader(u);
        let v_leader = self.leader(v);
        let merged_size = !(self.size(u) + self.size(v));
        self.vertex[v_leader] = merged_size;
        self.vertex[u_leader] = v_leader;
        self.diff_weights[u_leader] = w_sub;

        Ok(w)
    }

    pub fn size(&mut self, u: usize) -> usize {
        let idx = self.leader(u);
        !self.vertex[idx]
    }

    // u, vが違う集合の場合はErrとして返す
    pub fn diff(&mut self, u: usize, v: usize) -> Result<T, ()> {
        if self.same(u, v) {
            Ok(self.weight(u) - self.weight(v))
        } else {
            Err(())
        }
    }

    fn weight(&mut self, u: usize) -> T {
        self.leader(u);
        self.diff_weights[u]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_potentiality_unionfind() {
        let mut uf = PotentialityUnionfind::new(6);
        assert_eq!(uf.merge(0, 1, 1), Ok(1));
        assert_eq!(uf.merge(1, 2, 3), Ok(3));
        assert_eq!(uf.merge(2, 3, 1), Ok(1));
        assert_eq!(uf.merge(1, 3, 4), Ok(4));
        assert_eq!(uf.merge(3, 4, 2), Ok(2));
        assert_eq!(uf.merge(0, 4, 3), Err(7));
        assert!(uf.same(0, 4));
        assert_eq!(uf.diff(0, 4), Ok(7));
        assert_eq!(uf.diff(0, 5), Err(()));
    }
}