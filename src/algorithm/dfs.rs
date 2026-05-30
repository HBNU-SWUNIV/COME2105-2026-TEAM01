//! DFS(깊이 우선 탐색) 구현 (팀원 B: 김고운 담당)

use crate::graph::Graph;
use super::state::{AlgorithmKind, AlgorithmResult, StateSnapshot};

pub fn run(graph: &Graph, start: usize) -> AlgorithmResult {
    let n = graph.nodes.len();
    if n == 0 {
        return AlgorithmResult { kind: AlgorithmKind::DFS, ..Default::default() };
    }

    let mut snapshots = vec![];
    let mut visited_flags = vec![false; n];
    let mut visited_order = vec![];
    let mut stack: Vec<usize> = vec![];
    let mut edge_traversal_count = 0usize;

    stack.push(start);

    snapshots.push(StateSnapshot {
        current_line: 1,
        current_node: start,
        queue_or_stack: stack.clone(),
        visited_order: visited_order.clone(),
        visited_flags: visited_flags.clone(),
        distances: None,
        log_message: format!("시작 노드 {} 를 스택에 삽입", start),
        negative_cycle: false,
    });

    while let Some(node) = stack.pop() {
        if visited_flags[node] {
            continue;
        }
        visited_flags[node] = true;
        visited_order.push(node);

        snapshots.push(StateSnapshot {
            current_line: 2,
            current_node: node,
            queue_or_stack: stack.clone(),
            visited_order: visited_order.clone(),
            visited_flags: visited_flags.clone(),
            distances: None,
            log_message: format!("노드 {} 를 스택에서 꺼내 방문", node),
            negative_cycle: false,
        });

        // 인접 노드를 역순으로 push (방문 순서 유지)
        let mut neighbors = graph.neighbors(node);
        neighbors.reverse();
        for (neighbor, _weight) in neighbors {
            edge_traversal_count += 1;
            if !visited_flags[neighbor] {
                stack.push(neighbor);

                snapshots.push(StateSnapshot {
                    current_line: 3,
                    current_node: neighbor,
                    queue_or_stack: stack.clone(),
                    visited_order: visited_order.clone(),
                    visited_flags: visited_flags.clone(),
                    distances: None,
                    log_message: format!(
                        "노드 {} 의 인접 노드 {} → 스택에 삽입",
                        node, neighbor
                    ),
                    negative_cycle: false,
                });
            }
        }
    }

    AlgorithmResult {
        kind: AlgorithmKind::DFS,
        snapshots,
        visit_order: visited_order,
        edge_traversal_count,
    }
}