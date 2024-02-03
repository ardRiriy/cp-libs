use std::cmp::Reverse;
use std::collections::BinaryHeap;
use num_traits::Num;

#[derive(Clone)]
struct Edge<T> where T: Num {
    to: usize,
    cost: T
}

struct Graph<T> where T: From<usize> + Num{
    graph: Vec<Vec<Edge<T>>>,
}

impl<T> Graph<T> where T: From<usize> + Num + Clone + Ord {
    fn new(size: usize) -> Graph<T> {
        let graph :Vec<Vec<Edge<T>>> = vec![vec![]; size];
        Graph{ graph }
    }

    // 1-index で与えられることを想定
    fn add(&mut self, from: usize, to: usize, cost: T) {
        self.graph[from - 1].push(Edge{to: to - 1, cost});
    }

    fn dijkstra(&mut self, start: usize) -> Vec<T> {
        let mut distance = vec![INF.into(); self.graph.len()];
        let mut heap:BinaryHeap<Reverse<(T, usize)>> = BinaryHeap::new();
        heap.push(Reverse((0.into(), start)));

        while let Some(Reverse((cost, pos))) = heap.pop() {
            if distance[pos] != INF.into() { continue; }
            distance[pos] = cost.clone();

            for edge in &self.graph[pos] {
                if distance[edge.to] != INF.into() { continue; }
                heap.push(Reverse((cost.clone() + edge.cost.clone().into() , edge.to)));
            }
        }
        distance
    }
}
