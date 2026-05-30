//! 알고리즘 엔진 모듈

pub mod bfs;
pub mod dfs;
pub mod dijkstra;
pub mod bellman_ford;
pub mod floyd_warshall;
pub mod astar;
pub mod prim;
pub mod kruskal;
pub mod state;
pub mod topological_sort;
pub mod bidirectional_bfs;

pub use state::{AlgorithmKind, AlgorithmResult};

use crate::graph::Graph;

pub fn run_algorithm(graph: &Graph, start_node: usize, kind: AlgorithmKind) -> AlgorithmResult {
    match kind {
        AlgorithmKind::BFS              => bfs::run(graph, start_node),
        AlgorithmKind::DFS              => dfs::run(graph, start_node),
        AlgorithmKind::Dijkstra         => dijkstra::run(graph, start_node),
        AlgorithmKind::BellmanFord      => bellman_ford::run(graph, start_node),
        AlgorithmKind::FloydWarshall    => floyd_warshall::run(graph, start_node),
        AlgorithmKind::AStar            => astar::run(graph, start_node),
        AlgorithmKind::Prim             => prim::run(graph, start_node),
        AlgorithmKind::Kruskal          => kruskal::run(graph, start_node),
        AlgorithmKind::TopologicalSort  => topological_sort::run(graph, start_node),
        AlgorithmKind::BidirectionalBFS => bidirectional_bfs::run(graph, start_node),
    }
}
