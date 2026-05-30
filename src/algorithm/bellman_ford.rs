//! Bellman-Ford(벨만-포드) 최단 경로 구현
//! - 음수 가중치 간선 지원
//! - 음수 사이클 감지 기능

use crate::graph::Graph;
use super::state::{AlgorithmKind, AlgorithmResult, StateSnapshot};

pub fn run(graph: &Graph, start: usize) -> AlgorithmResult {
    let n = graph.nodes.len();
    if n == 0 {
        return AlgorithmResult { kind: AlgorithmKind::BellmanFord, ..Default::default() };
    }

    let inf = f64::INFINITY;
    let mut dist = vec![inf; n];
    let mut visited_flags = vec![false; n];
    let mut visited_order = vec![];
    let mut snapshots = vec![];
    let mut edge_traversal_count = 0usize;

    dist[start] = 0.0;

    snapshots.push(StateSnapshot {
        current_line: 0,
        current_node: start,
        queue_or_stack: vec![start],
        visited_order: visited_order.clone(),
        visited_flags: visited_flags.clone(),
        distances: Some(dist.clone()),
        log_message: format!("시작 노드 {} 거리=0 으로 초기화, 나머지=∞", start),
        negative_cycle: false,
    });

    // (V-1)번 반복 완화
    for iter in 0..(n.saturating_sub(1)) {
        let mut updated = false;

        for edge in &graph.edges {
            edge_traversal_count += 1;
            if dist[edge.from].is_finite() {
                let new_dist = dist[edge.from] + edge.weight;
                if new_dist < dist[edge.to] {
                    dist[edge.to] = new_dist;
                    updated = true;

                    snapshots.push(StateSnapshot {
                        current_line: 2,
                        current_node: edge.to,
                        queue_or_stack: vec![],
                        visited_order: visited_order.clone(),
                        visited_flags: visited_flags.clone(),
                        distances: Some(dist.clone()),
                        log_message: format!(
                            "[반복 {}] 간선 {}→{} 완화: 거리 {:.2}",
                            iter + 1, edge.from, edge.to, new_dist
                        ),
                        negative_cycle: false,
                    });
                }
            }
            // 무방향 그래프면 역방향도 완화
            if !graph.is_directed && dist[edge.to].is_finite() {
                let new_dist = dist[edge.to] + edge.weight;
                if new_dist < dist[edge.from] {
                    dist[edge.from] = new_dist;
                    updated = true;

                    snapshots.push(StateSnapshot {
                        current_line: 2,
                        current_node: edge.from,
                        queue_or_stack: vec![],
                        visited_order: visited_order.clone(),
                        visited_flags: visited_flags.clone(),
                        distances: Some(dist.clone()),
                        log_message: format!(
                            "[반복 {}] 간선 {}→{} 완화(역): 거리 {:.2}",
                            iter + 1, edge.to, edge.from, new_dist
                        ),
                        negative_cycle: false,
                    });
                }
            }
        }

        if !updated {
            snapshots.push(StateSnapshot {
                current_line: 3,
                current_node: start,
                queue_or_stack: vec![],
                visited_order: visited_order.clone(),
                visited_flags: visited_flags.clone(),
                distances: Some(dist.clone()),
                log_message: format!("[반복 {}] 갱신 없음 → 조기 종료", iter + 1),
                negative_cycle: false,
            });
            break;
        }
    }

    // 음수 사이클 감지 (V번째 반복)
    let mut has_negative_cycle = false;
    for edge in &graph.edges {
        if dist[edge.from].is_finite() && dist[edge.from] + edge.weight < dist[edge.to] {
            has_negative_cycle = true;
            break;
        }
    }

    // 최종 방문 처리 (도달 가능한 노드)
    for (i, &d) in dist.iter().enumerate() {
        if d.is_finite() {
            visited_flags[i] = true;
            visited_order.push(i);
        }
    }

    snapshots.push(StateSnapshot {
        current_line: 4,
        current_node: start,
        queue_or_stack: vec![],
        visited_order: visited_order.clone(),
        visited_flags: visited_flags.clone(),
        distances: Some(dist.clone()),
        log_message: if has_negative_cycle {
            "⚠ 음수 사이클 감지됨! 최단 경로가 존재하지 않습니다.".to_string()
        } else {
            "✅ Bellman-Ford 완료 — 모든 최단 거리 확정".to_string()
        },
        negative_cycle: has_negative_cycle,
    });

    AlgorithmResult {
        kind: AlgorithmKind::BellmanFord,
        snapshots,
        visit_order: visited_order,
        edge_traversal_count,
    }
}