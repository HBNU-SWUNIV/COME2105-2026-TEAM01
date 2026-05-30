//! Floyd-Warshall(전체 쌍 최단 경로) 구현 (팀원 B: 김고운 담당)

use crate::graph::Graph;
use super::state::{AlgorithmKind, AlgorithmResult};

pub fn run(graph: &Graph, start: usize) -> AlgorithmResult {
    // TODO: Floyd-Warshall 알고리즘 구현
    // - 3중 반복문으로 dist[i][k] + dist[k][j] 비교
    // - 음수 사이클 감지 (dist[i][i] < 0)
    // - 단계별 StateSnapshot 생성 (중간 노드 k 기록)
    todo!("Floyd-Warshall 구현 예정 — 김고운")
}
