//! 알고리즘 실행 단계별 상태 스냅샷 타입 정의

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub enum AlgorithmKind {
    #[default]
    BFS,
    DFS,
    Dijkstra,
    BellmanFord,
    FloydWarshall,
    AStar,
    Prim,
    Kruskal,
    TopologicalSort,
    BidirectionalBFS,
}

impl std::fmt::Display for AlgorithmKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlgorithmKind::BFS              => write!(f, "BFS"),
            AlgorithmKind::DFS              => write!(f, "DFS"),
            AlgorithmKind::Dijkstra         => write!(f, "Dijkstra"),
            AlgorithmKind::BellmanFord      => write!(f, "Bellman-Ford"),
            AlgorithmKind::FloydWarshall    => write!(f, "Floyd-Warshall"),
            AlgorithmKind::AStar            => write!(f, "A*"),
            AlgorithmKind::Prim             => write!(f, "Prim"),
            AlgorithmKind::Kruskal          => write!(f, "Kruskal"),
            AlgorithmKind::TopologicalSort  => write!(f, "위상 정렬"),
            AlgorithmKind::BidirectionalBFS => write!(f, "양방향 BFS"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSnapshot {
    /// 수도코드 하이라이트 라인 번호
    pub current_line: usize,
    /// 현재 처리 중인 노드 ID
    pub current_node: usize,
    /// 큐(BFS) / 스택(DFS) / 우선순위 큐 상태
    pub queue_or_stack: Vec<usize>,
    /// 방문 완료 순서
    pub visited_order: Vec<usize>,
    /// 방문 여부 배열
    pub visited_flags: Vec<bool>,
    /// 거리 배열 (Dijkstra / Bellman-Ford / Floyd-Warshall / A*)
    pub distances: Option<Vec<f64>>,
    /// 로그 메시지
    pub log_message: String,
    /// 음수 사이클 감지 여부 (Bellman-Ford / Floyd-Warshall) 또는 일반 사이클 (TopologicalSort)
    pub negative_cycle: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AlgorithmResult {
    pub kind: AlgorithmKind,
    pub snapshots: Vec<StateSnapshot>,
    pub visit_order: Vec<usize>,
    pub edge_traversal_count: usize,
}
