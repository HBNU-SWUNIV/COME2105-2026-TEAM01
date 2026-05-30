//! 위상 정렬(Topological Sort) 구현 (팀원 B: 김고운 담당)

use crate::graph::Graph;
use super::state::{AlgorithmKind, AlgorithmResult};

/// Kahn's Algorithm (BFS 기반) 위상 정렬 — 기본 실행 함수
pub fn run(graph: &Graph, start: usize) -> AlgorithmResult {
    // TODO: 위상 정렬 구현 (Kahn's Algorithm)
    // - 진입 차수(in_degree) 계산
    // - 진입 차수 0인 노드를 큐에 삽입
    // - 큐에서 꺼내며 인접 노드 진입 차수 감소
    // - 처리 노드 수 < V 이면 사이클 존재 (negative_cycle = true)
    // - 단계별 StateSnapshot 생성
    todo!("위상 정렬 구현 예정 — 김고운")
}

/// DAG 여부 빠른 판별
pub fn is_dag(graph: &Graph) -> bool {
    // TODO: 사이클 없는 방향 그래프인지 확인
    todo!("is_dag 구현 예정 — 김고운")
}

/// 위상 정렬 결과로부터 레벨(층) 계산
pub fn compute_levels(graph: &Graph) -> Vec<usize> {
    // TODO: 각 노드의 레벨 반환
    todo!("compute_levels 구현 예정 — 김고운")
}
