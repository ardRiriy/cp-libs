use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};
use num_traits::Num;

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

    fn dfs(&self, start: usize) {
        let mut que = VecDeque::new();
        que.push_back(start as isize - 1);

        let mut checked = vec![false; self.graph.len()];
        checked[start-1] = true;

        while let Some(pos) = que.pop_back() {
            if pos >= 0 {
                // 行きがけの処理
                que.push_back(!pos);

                let p = pos as usize;
                for next in &self.graph[p] {
                    if !checked[next.to] {
                        que.push_back(next.to as isize);
                        checked[next.to] = true;
                    }
                }
            }else{
                // 帰りがけの処理
                let p = !pos as usize;

            }
        }
    }

    fn bfs(&self, start: usize) {
        let mut que = VecDeque::new();
        que.push_back(start - 1);

        let mut checked = vec![false; self.graph.len()];
        checked[start-1] = true;

        while let Some(pos) = que.pop_front() {
            for next in &self.graph[pos] {
                if !checked[next.to] {
                    que.push_back(next.to);
                    checked[next.to] = true;
                }
            }
        }
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
