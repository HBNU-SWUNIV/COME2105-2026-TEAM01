//! Dijkstra(최단 경로) 구현 (팀원 B: 김고운 담당)

use std::collections::BinaryHeap;
use std::cmp::Reverse;
use crate::graph::Graph;
use super::state::{AlgorithmKind, AlgorithmResult, StateSnapshot};

pub fn run(graph: &Graph, start: usize) -> AlgorithmResult {
    let n = graph.nodes.len();
    if n == 0 {
        return AlgorithmResult { kind: AlgorithmKind::Dijkstra, ..Default::default() };
    }

    let inf = f64::INFINITY;
    let mut dist = vec![inf; n];
    let mut visited_flags = vec![false; n];
    let mut visited_order = vec![];
    let mut snapshots = vec![];
    let mut edge_traversal_count = 0usize;

    // (거리 * 1000 as u64 로 정수 변환하여 BinaryHeap 사용, Reverse로 최솟값 우선)
    // (dist_u64, node_id)
    let mut heap: BinaryHeap<Reverse<(u64, usize)>> = BinaryHeap::new();
    dist[start] = 0.0;
    heap.push(Reverse((0, start)));

    snapshots.push(StateSnapshot {
        current_line: 1,
        current_node: start,
        queue_or_stack: vec![start],
        visited_order: visited_order.clone(),
        visited_flags: visited_flags.clone(),
        distances: Some(dist.clone()),
        log_message: format!("시작 노드 {} 거리=0 으로 초기화", start),
        negative_cycle: false,
    });

    while let Some(Reverse((_, node))) = heap.pop() {
        if visited_flags[node] {
            continue;
        }
        visited_flags[node] = true;
        visited_order.push(node);

        let heap_preview: Vec<usize> = heap.iter().map(|Reverse((_, n))| *n).collect();
        snapshots.push(StateSnapshot {
            current_line: 2,
            current_node: node,
            queue_or_stack: heap_preview.clone(),
            visited_order: visited_order.clone(),
            visited_flags: visited_flags.clone(),
            distances: Some(dist.clone()),
            log_message: format!("노드 {} 확정 (거리 = {:.2})", node, dist[node]),
            negative_cycle: false,
        });

        for (neighbor, weight) in graph.neighbors(node) {
            edge_traversal_count += 1;
            let new_dist = dist[node] + weight;
            if new_dist < dist[neighbor] {
                dist[neighbor] = new_dist;
                let dist_u64 = (new_dist * 1000.0) as u64;
                heap.push(Reverse((dist_u64, neighbor)));

                let heap_preview2: Vec<usize> = heap.iter().map(|Reverse((_, n))| *n).collect();
                snapshots.push(StateSnapshot {
                    current_line: 3,
                    current_node: neighbor,
                    queue_or_stack: heap_preview2,
                    visited_order: visited_order.clone(),
                    visited_flags: visited_flags.clone(),
                    distances: Some(dist.clone()),
                    log_message: format!(
                        "노드 {} → {} 거리 갱신: {:.2}",
                        node, neighbor, new_dist
                    ),
                    negative_cycle: false,
                });
            }
        }
    }

    AlgorithmResult {
        kind: AlgorithmKind::Dijkstra,
        snapshots,
        visit_order: visited_order,
        edge_traversal_count,
    }
}