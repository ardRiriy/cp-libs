pub fn warshall_floyd<T>(graph: &Vec<Vec<(usize, T)>>) -> Vec<Vec<T>>
where T: num_traits::Num + num_traits::Bounded + Copy + Ord + std::ops::Shr<i32, Output = T>
{
    // 頂点数V, 辺の本数Eに対してO(|V|^3 + |E|)
    let inf = T::max_value() >> 2;
    let v = graph.len();
    let mut distance = vec![vec![inf; v]; v];
    for i in 0..v {
        for (to, cost) in graph[i].iter() {
            distance[i][*to] = distance[i][*to].min(*cost);
        }
        distance[i][i] = T::zero();
    }

    for k in 0..v {
        for i in 0..v {
            for j in 0..v {
                distance[i][j] = distance[i][j].min(distance[i][k] + distance[k][j]);
            }
        }
    }

    distance
}

pub fn dijkstra<T>(start: usize, graph: &Vec<Vec<(usize, T)>>) -> Vec<T>
where T: num_traits::Num + num_traits::Bounded + Copy + Ord + std::ops::Shr<i32, Output = T>
{
    let inf = T::max_value() >> 2;
    let v = graph.len();
    let mut distance = vec![inf; v];
    let mut pq = std::collections::BinaryHeap::new();
    pq.push(std::cmp::Reverse((T::zero(), start)));

    let mut left = v; // 辺の本数が多い場合の定数倍改善
    while let Some(std::cmp::Reverse((cost, pos))) = pq.pop() {
        if distance[pos] != inf {
            continue;
        }
        distance[pos] = cost;
        left -= 1;
        if left == 0 {
            break;
        }

        for &(ni, w) in &graph[pos] {
            if distance[ni] == inf {
                pq.push(std::cmp::Reverse((w + cost, ni)));
            }
        }
    }

    distance
}
