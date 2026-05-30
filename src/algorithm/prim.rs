//! Prim(프림) 최소 신장 트리 구현
//! - 우선순위 큐 기반 O(E log V)
//! - 시작 노드에서 MST를 탐욕적으로 확장

use std::collections::BinaryHeap;
use std::cmp::Reverse;
use crate::graph::Graph;
use super::state::{AlgorithmKind, AlgorithmResult, StateSnapshot};

pub fn run(graph: &Graph, start: usize) -> AlgorithmResult {
    let n = graph.nodes.len();
    if n == 0 {
        return AlgorithmResult { kind: AlgorithmKind::Prim, ..Default::default() };
    }

    let inf = f64::INFINITY;
    let mut in_mst = vec![false; n];
    let mut key = vec![inf; n];          // 현재 노드를 MST에 연결하는 최소 가중치
    let mut visited_flags = vec![false; n];
    let mut visited_order = vec![];
    let mut snapshots = vec![];
    let mut edge_traversal_count = 0usize;
    let mut mst_cost = 0.0f64;

    key[start] = 0.0;

    // (가중치 * 1000 as u64, node_id)
    let mut heap: BinaryHeap<Reverse<(u64, usize)>> = BinaryHeap::new();
    heap.push(Reverse((0, start)));

    snapshots.push(StateSnapshot {
        current_line: 0,
        current_node: start,
        queue_or_stack: vec![start],
        visited_order: visited_order.clone(),
        visited_flags: visited_flags.clone(),
        distances: Some(key.clone()),
        log_message: format!("시작 노드 {} key=0 으로 초기화, 나머지=∞", start),
        negative_cycle: false,
    });

    while let Some(Reverse((_, node))) = heap.pop() {
        if in_mst[node] {
            continue;
        }
        in_mst[node] = true;
        visited_flags[node] = true;
        visited_order.push(node);
        mst_cost += key[node];

        let heap_preview: Vec<usize> = heap.iter().map(|Reverse((_, n))| *n).collect();
        snapshots.push(StateSnapshot {
            current_line: 1,
            current_node: node,
            queue_or_stack: heap_preview.clone(),
            visited_order: visited_order.clone(),
            visited_flags: visited_flags.clone(),
            distances: Some(key.clone()),
            log_message: format!(
                "노드 {} MST에 추가 (key={:.2}) | 누적 비용={:.2}",
                node, key[node], mst_cost
            ),
            negative_cycle: false,
        });

        for (neighbor, weight) in graph.neighbors(node) {
            edge_traversal_count += 1;
            if !in_mst[neighbor] && weight < key[neighbor] {
                key[neighbor] = weight;
                heap.push(Reverse(((weight * 1000.0) as u64, neighbor)));

                let heap_preview2: Vec<usize> = heap.iter().map(|Reverse((_, n))| *n).collect();
                snapshots.push(StateSnapshot {
                    current_line: 2,
                    current_node: neighbor,
                    queue_or_stack: heap_preview2,
                    visited_order: visited_order.clone(),
                    visited_flags: visited_flags.clone(),
                    distances: Some(key.clone()),
                    log_message: format!(
                        "  → 이웃 {} key 갱신: {:.2} ({}→{})",
                        neighbor, weight, node, neighbor
                    ),
                    negative_cycle: false,
                });
            }
        }
    }

    snapshots.push(StateSnapshot {
        current_line: 3,
        current_node: start,
        queue_or_stack: vec![],
        visited_order: visited_order.clone(),
        visited_flags: visited_flags.clone(),
        distances: Some(key.clone()),
        log_message: format!("✅ Prim MST 완료 — 총 비용: {:.2}", mst_cost),
        negative_cycle: false,
    });

    AlgorithmResult {
        kind: AlgorithmKind::Prim,
        snapshots,
        visit_order: visited_order,
        edge_traversal_count,
    }
}