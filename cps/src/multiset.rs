pub struct MultiSet<T, U> {
    map: std::collections::BTreeMap<T, U>,
}

impl<T, U> MultiSet<T, U> 
where T: Ord + Copy, U: num::PrimInt + std::ops::AddAssign
{
    pub fn new() -> Self {
        use std::collections::BTreeMap;
        MultiSet { map: BTreeMap::new() }
    }

    /* keyをsizeだけ増やして新しい値を返す */
    pub fn add(&mut self, key: T, size: U) -> U {
        *self.map.entry(key).or_insert(U::zero()) += size;
        *self.map.get(&key).unwrap()
    }

    /* sizeだけ減らして新しい値を返す */
    pub fn remove(&mut self, key: T, size: U) -> Option<U> {
        if let Some(&current_size) = self.map.get(&key) {
            if current_size > size {
                let new_size = current_size - size;
                self.map.insert(key, new_size);
                Some(new_size)
            } else {
                self.map.remove(&key);
                Some(U::zero())
            }
        } else {
            None
        }
    }

    pub fn get(&self, key: T) -> Option<U> {
        if let Some(&v) = self.map.get(&key) {
            Some(v)
        } else {
            None
        }
    }

    pub fn size(&self) -> usize {
        self.map.len()
    }
}
