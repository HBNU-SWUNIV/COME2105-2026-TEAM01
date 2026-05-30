//! A*(휴리스틱 최단 경로) 구현 (팀원 B: 김고운 담당)

use crate::graph::Graph;
use super::state::{AlgorithmKind, AlgorithmResult};

pub fn run(graph: &Graph, start: usize) -> AlgorithmResult {
    // TODO: A* 알고리즘 구현
    // - 목표 노드: 마지막 노드 (graph.nodes.len() - 1)
    // - 휴리스틱: 유클리드 거리 h(n) = sqrt((x2-x1)^2 + (y2-y1)^2)
    // - f(n) = g(n) + h(n) 기반 우선순위 큐
    // - 단계별 StateSnapshot 생성
    todo!("A* 구현 예정 — 김고운")
}
