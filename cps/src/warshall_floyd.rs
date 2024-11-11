pub trait WFelm<T> 
where T: Copy
{
    fn min(&self, a: T, b: T) -> T; // コストが軽い方を返却
    fn add(&self, a: T, b: T) -> T; // コストの合算を計算
    fn infinity(&self) -> T;
    fn identity(&self) -> T;
}

pub struct DefaultWFelm;

impl<T> WFelm<T> for DefaultWFelm
where T: Copy + num::PrimInt {
    fn min(&self, a: T, b: T) -> T {
        a.min(b)
    }

    fn add(&self, a: T, b: T) -> T {
        a + b
    }

    fn infinity(&self) -> T {
        T::max_value() >> 2
    }

    fn identity(&self) -> T {
        T::zero()
    }
}

#[allow(dead_code)]
pub struct WarshallFloyd<T, O>
where
    T: Copy,
    O: WFelm<T>,
{
    dist: Vec<Vec<T>>,
    op: O,
}

impl<T, O> WarshallFloyd<T, O>
where
    T: Copy,
    O: WFelm<T>,
{
    pub fn new(graph: &Vec<Vec<(usize, T)>>, op: O) -> Self {
        let n = graph.len();
        let mut dist = vec![vec![op.infinity(); n]; n];
        
        for i in 0..n {
            for (to, cost) in graph[i].iter() {
                dist[i][*to] = op.min(dist[i][*to], *cost);
            }
            dist[i][i] = op.identity();
        }
        
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    dist[i][j] = op.min(dist[i][j], op.add(dist[i][k], dist[k][j]));
                }
            }
        }
        
        Self { dist, op }
    }
    
    pub fn get(&self, from: usize, to: usize) -> T {
        self.dist[from][to]
    }
    
    #[allow(dead_code)]
    fn add(&mut self) {
        todo!();
    }
}

