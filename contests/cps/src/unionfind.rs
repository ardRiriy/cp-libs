#[derive(Debug)]
pub struct UnionFind {
    vertex: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> UnionFind {
        UnionFind { vertex: vec![!1; size] }
    }

    pub fn leader(&mut self, u: usize) -> usize {
        let elm: usize = self.vertex[u];
        if elm > self.vertex.len() {
            // 親ノードなので返却
            return u;
        } else {
            // 子ノードなので親ノードを取得したのち、圧縮
            self.vertex[u] = self.leader(elm);
            return self.vertex[u];
        }
    }

    pub fn same(&mut self, u: usize, v: usize) -> bool {
        return self.leader(u) == self.leader(v);
    }

    pub fn size(&mut self, u: usize) -> usize {
        let idx: usize = self.leader(u);
        return !self.vertex[idx];
    }

    pub fn merge(&mut self, u: usize, v: usize) -> bool {
        if self.same(u, v) {
            // すでに親が同じ場合は何もせず、「マージされなかった」ことを報告
            return false;
        }
        // vをuの子要素にする
        let u_leader = self.leader(u);
        let v_leader = self.leader(v);
        let merged_size = !(self.size(u) + self.size(v));
        self.vertex[u_leader] = merged_size;
        self.vertex[v_leader] = u_leader;
        return true;
    }
}

mod uf_test {
    #[test]
    fn uf_test() {
        use super::UnionFind;
        let mut uf = UnionFind::new(6);
        assert!(uf.merge(0, 1));
        assert!(uf.merge(2, 0));
        assert!(!uf.merge(1, 2));
        assert!(uf.merge(3, 4));

        assert!(uf.same(1, 2));
        assert!(uf.same(3, 4));
        assert!(!uf.same(0, 3));
        assert!(!uf.same(3, 5));

        assert_eq!(uf.size(0), 3);
        assert_eq!(uf.size(4), 2);
        assert_eq!(uf.size(5), 1);

        assert_eq!(uf.leader(0), uf.leader(1));
    }
}
