use cps::warshall_floyd::{DefaultWFelm, WFelm, WarshallFloyd};
use proconio::{input, marker::Usize1};
fn main() {
    input!{
        n: usize,
        m: usize,
        q: usize,
        edges: [(Usize1, Usize1, u64); m],
    }
    
    // クエリを逆順から見ると辺追加クエリとみなせて、O(N^2)でできる

}

