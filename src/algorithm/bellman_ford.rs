//! Bellman-Ford(음수 가중치 최단 경로) 구현 (팀원 B: 김고운 담당)

use crate::graph::Graph;
use super::state::{AlgorithmKind, AlgorithmResult};

pub fn run(graph: &Graph, start: usize) -> AlgorithmResult {
    // TODO: Bellman-Ford 알고리즘 구현
    // - V-1 라운드 간선 완화(Relaxation)
    // - 음수 사이클 감지 시 negative_cycle = true 설정
    // - 단계별 StateSnapshot 생성
    todo!("Bellman-Ford 구현 예정 — 김고운")
}
