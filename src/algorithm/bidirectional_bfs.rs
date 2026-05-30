//! 양방향 BFS(Bidirectional BFS) 구현 (팀원 B: 김고운 담당)

use crate::graph::Graph;
use super::state::{AlgorithmKind, AlgorithmResult};

/// 양방향 BFS — 기본 실행 함수 (목표: 마지막 노드)
pub fn run(graph: &Graph, start: usize) -> AlgorithmResult {
    let goal = if graph.nodes.len() > 1 { graph.nodes.len() - 1 } else { 0 };
    run_between(graph, start, goal)
}

/// start ~ goal 사이 최단 경로 탐색
pub fn run_between(graph: &Graph, start: usize, goal: usize) -> AlgorithmResult {
    // TODO: 양방향 BFS 구현
    // - 정방향 큐(start 출발), 역방향 큐(goal 출발) 동시 확장
    // - 어느 한 쪽이 상대방 visited 집합과 교차 시 종료
    // - reconstruct_path로 경로 복원
    // - 단계별 StateSnapshot 생성
    todo!("양방향 BFS 구현 예정 — 김고운")
}
