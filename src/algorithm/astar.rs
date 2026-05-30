//! A*(에이스타) 휴리스틱 최단 경로 구현
//! - 휴리스틱: 유클리드 거리 (egui 좌표 기반)
//! - 목표 노드: graph.nodes의 마지막 노드 (또는 goal_node 파라미터)

use std::collections::BinaryHeap;
use std::cmp::Reverse;
use crate::graph::Graph;
use super::state::{AlgorithmKind, AlgorithmResult, StateSnapshot};

/// 두 노드 간 유클리드 거리를 휴리스틱으로 사용
fn heuristic(graph: &Graph, from: usize, to: usize) -> f64 {
    let a = &graph.nodes[from];
    let b = &graph.nodes[to];
    let dx = (a.x - b.x) as f64;
    let dy = (a.y - b.y) as f64;
    (dx * dx + dy * dy).sqrt()
}

pub fn run(graph: &Graph, start: usize) -> AlgorithmResult {
    run_with_goal(graph, start, graph.nodes.len().saturating_sub(1))
}

pub fn run_with_goal(graph: &Graph, start: usize, goal: usize) -> AlgorithmResult {
    let n = graph.nodes.len();
    if n == 0 {
        return AlgorithmResult { kind: AlgorithmKind::AStar, ..Default::default() };
    }

    let inf = f64::INFINITY;
    let mut g_score = vec![inf; n];  // 시작 → 현재 실제 비용
    let mut f_score = vec![inf; n];  // g + h 추정 총비용
    let mut visited_flags = vec![false; n];
    let mut visited_order = vec![];
    let mut snapshots = vec![];
    let mut edge_traversal_count = 0usize;

    g_score[start] = 0.0;
    f_score[start] = heuristic(graph, start, goal);

    // (f_score * 1000 as u64, node_id)
    let mut heap: BinaryHeap<Reverse<(u64, usize)>> = BinaryHeap::new();
    heap.push(Reverse(((f_score[start] * 1000.0) as u64, start)));

    snapshots.push(StateSnapshot {
        current_line: 0,
        current_node: start,
        queue_or_stack: vec![start],
        visited_order: visited_order.clone(),
        visited_flags: visited_flags.clone(),
        distances: Some(g_score.clone()),
        log_message: format!(
            "시작={} 목표={} | g[{}]=0, h={:.2}, f={:.2}",
            start, goal, start, f_score[start], f_score[start]
        ),
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
            current_line: 1,
            current_node: node,
            queue_or_stack: heap_preview.clone(),
            visited_order: visited_order.clone(),
            visited_flags: visited_flags.clone(),
            distances: Some(g_score.clone()),
            log_message: format!(
                "노드 {} 확정 | g={:.2} f={:.2}",
                node, g_score[node], f_score[node]
            ),
            negative_cycle: false,
        });

        // 목표 도달 시 조기 종료
        if node == goal {
            snapshots.push(StateSnapshot {
                current_line: 2,
                current_node: node,
                queue_or_stack: vec![],
                visited_order: visited_order.clone(),
                visited_flags: visited_flags.clone(),
                distances: Some(g_score.clone()),
                log_message: format!(
                    "🎯 목표 노드 {} 도달! 최단 거리 = {:.2}",
                    goal, g_score[goal]
                ),
                negative_cycle: false,
            });
            break;
        }

        for (neighbor, weight) in graph.neighbors(node) {
            edge_traversal_count += 1;
            let tentative_g = g_score[node] + weight;
            if tentative_g < g_score[neighbor] {
                g_score[neighbor] = tentative_g;
                let h = heuristic(graph, neighbor, goal);
                f_score[neighbor] = tentative_g + h;
                heap.push(Reverse(((f_score[neighbor] * 1000.0) as u64, neighbor)));

                let heap_preview2: Vec<usize> = heap.iter().map(|Reverse((_, n))| *n).collect();
                snapshots.push(StateSnapshot {
                    current_line: 3,
                    current_node: neighbor,
                    queue_or_stack: heap_preview2,
                    visited_order: visited_order.clone(),
                    visited_flags: visited_flags.clone(),
                    distances: Some(g_score.clone()),
                    log_message: format!(
                        "  → 이웃 {} | g={:.2} h={:.2} f={:.2}",
                        neighbor, tentative_g, h, f_score[neighbor]
                    ),
                    negative_cycle: false,
                });
            }
        }
    }

    AlgorithmResult {
        kind: AlgorithmKind::AStar,
        snapshots,
        visit_order: visited_order,
        edge_traversal_count,
    }
}