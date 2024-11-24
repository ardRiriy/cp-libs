#[allow(dead_code)]
pub mod data_structure {
    use core::fmt;
    use std::fmt::Formatter;

    use itertools::Itertools;
    use rand::{thread_rng, Rng};

    // [0, N)の値の集合の管理をO(1)で判定するデータ構造
    // ref: https://topcoder-tomerun.hatenablog.jp/entry/2021/06/12/134643
    #[derive(Clone)]
    pub struct IndexSet {
        que: Vec<usize>,
        pos: Vec<Option<usize>>,
    }

    impl IndexSet {
        pub fn new(n: usize) -> Self {
            Self { que: vec![], pos: vec![None; n] }
        }

        // 値iを集合に追加する
        pub fn add(&mut self, i: usize) -> bool {
            if self.pos[i].is_some() {
                return false;
            }

            self.pos[i] = Some(self.que.len());
            self.que.push(i);
            true
        }

        // 値iを集合から削除する
        pub fn remove(&mut self, i: usize) -> bool {
            if self.pos[i].is_none() {
                return false;
            }

            let p = self.pos[i].unwrap();
            let q = *self.que.last().unwrap();
            self.que[p] = q;
            self.que.pop();
            self.pos[q] = Some(p);
            self.pos[i] = None;
            true
        }

        // 値iが集合に含まれるかどうかを判定
        pub fn contain(&self, i: usize) -> bool {
            self.pos[i].is_some()
        }

        // 集合に含まれる値からランダムに一つ選んで返す 
        pub fn random(&self) -> usize {
            let mut rng = thread_rng();
            self.que[rng.gen_range(0..self.que.len())]
        }

        pub fn size(&self) -> usize {
            self.que.len()
        }
    }

    impl std::fmt::Display for IndexSet {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let str = self.que.iter()
                .sorted()
                .join(" ");
            write!(f, "[ {} ]", str)
        }
    }

    impl std::fmt::Debug for IndexSet {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "{}", self)
        }
    }
}
