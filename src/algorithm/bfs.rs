//! BFS(너비 우선 탐색) 구현 (팀원 B: 김고운 담당)

use std::collections::VecDeque;
use crate::graph::Graph;
use super::state::{AlgorithmKind, AlgorithmResult, StateSnapshot};

pub fn run(graph: &Graph, start: usize) -> AlgorithmResult {
    let n = graph.nodes.len();
    if n == 0 {
        return AlgorithmResult { kind: AlgorithmKind::BFS, ..Default::default() };
    }

    let mut snapshots = vec![];
    let mut visited_flags = vec![false; n];
    let mut visited_order = vec![];
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut edge_traversal_count = 0usize;

    visited_flags[start] = true;
    queue.push_back(start);

    // 초기 스냅샷: 큐에 시작 노드 삽입
    snapshots.push(StateSnapshot {
        current_line: 1,
        current_node: start,
        queue_or_stack: queue.iter().cloned().collect(),
        visited_order: visited_order.clone(),
        visited_flags: visited_flags.clone(),
        distances: None,
        log_message: format!("시작 노드 {} 를 큐에 삽입, 방문 표시", start),
        negative_cycle: false,
    });

    while let Some(node) = queue.pop_front() {
        visited_order.push(node);

        snapshots.push(StateSnapshot {
            current_line: 2,
            current_node: node,
            queue_or_stack: queue.iter().cloned().collect(),
            visited_order: visited_order.clone(),
            visited_flags: visited_flags.clone(),
            distances: None,
            log_message: format!("노드 {} 를 큐에서 꺼내 처리", node),
            negative_cycle: false,
        });

        let neighbors = graph.neighbors(node);
        for (neighbor, _weight) in neighbors {
            edge_traversal_count += 1;
            if !visited_flags[neighbor] {
                visited_flags[neighbor] = true;
                queue.push_back(neighbor);

                snapshots.push(StateSnapshot {
                    current_line: 3,
                    current_node: neighbor,
                    queue_or_stack: queue.iter().cloned().collect(),
                    visited_order: visited_order.clone(),
                    visited_flags: visited_flags.clone(),
                    distances: None,
                    log_message: format!(
                        "노드 {} 의 인접 노드 {} 발견 → 큐에 삽입",
                        node, neighbor
                    ),
                    negative_cycle: false,
                });
            }
        }
    }

    AlgorithmResult {
        kind: AlgorithmKind::BFS,
        snapshots,
        visit_order: visited_order,
        edge_traversal_count,
    }
}