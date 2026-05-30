//! 위상 정렬(Topological Sort) 구현 — Kahn's Algorithm + DFS 기반 두 가지 방식 제공
//!
//! # 위상 정렬이란?
//! 방향 비순환 그래프(DAG)에서 각 노드를 선행 조건이 만족된 순서로 나열하는 알고리즘입니다.
//! 대표적인 응용으로는 빌드 시스템의 의존성 해결, 강의 수강 순서 결정 등이 있습니다.
//!
//! ## Kahn's Algorithm (BFS 기반)
//! 1. 모든 노드의 진입 차수(in-degree)를 계산합니다.
//! 2. 진입 차수가 0인 노드를 큐에 삽입합니다.
//! 3. 큐에서 노드를 꺼내 결과 리스트에 추가하고, 해당 노드의 인접 노드 진입 차수를 1 감소시킵니다.
//! 4. 진입 차수가 0이 된 노드를 큐에 삽입합니다.
//! 5. 큐가 빌 때까지 반복합니다. 처리된 노드 수가 전체 노드 수보다 적으면 사이클이 존재합니다.
//!
//! ## 시간 복잡도
//! - O(V + E) — 모든 노드와 간선을 각 1회씩 처리합니다.

use std::collections::VecDeque;
use crate::graph::Graph;
use super::state::{AlgorithmKind, AlgorithmResult, StateSnapshot};

// ────────────────────────────────────────────────────────────────────────────
// Kahn's Algorithm (BFS 기반)
// ────────────────────────────────────────────────────────────────────────────

/// Kahn's Algorithm을 사용한 위상 정렬 실행
///
/// 방향 그래프가 아닌 경우에도 동작하지만, 의미있는 결과를 얻으려면 방향 비순환 그래프(DAG)여야 합니다.
/// 사이클이 감지된 경우 `AlgorithmResult`의 `has_cycle` 관련 스냅샷 메시지로 알립니다.
pub fn run_kahn(graph: &Graph, _start: usize) -> AlgorithmResult {
    let n = graph.nodes.len();
    if n == 0 {
        return AlgorithmResult {
            kind: AlgorithmKind::TopologicalSort,
            ..Default::default()
        };
    }

    // ── 진입 차수 계산 ──────────────────────────────────────────────
    let mut in_degree = vec![0usize; n];
    for edge in &graph.edges {
        if edge.to < n {
            in_degree[edge.to] += 1;
        }
    }

    // ── 초기 큐 구성 (진입 차수 == 0) ──────────────────────────────
    let mut queue: VecDeque<usize> = VecDeque::new();
    for i in 0..n {
        if in_degree[i] == 0 {
            queue.push_back(i);
        }
    }

    let mut snapshots: Vec<StateSnapshot> = Vec::new();
    let mut visited_order: Vec<usize> = Vec::new();
    let mut visited_flags = vec![false; n];
    let mut edge_traversal_count = 0usize;

    // 초기 스냅샷
    snapshots.push(StateSnapshot {
        current_line: 1,
        current_node: queue.front().copied().unwrap_or(0),
        queue_or_stack: queue.iter().cloned().collect(),
        visited_order: visited_order.clone(),
        visited_flags: visited_flags.clone(),
        distances: None,
        log_message: format!(
            "진입 차수 0인 노드들로 큐 초기화: {:?}",
            queue.iter().cloned().collect::<Vec<_>>()
        ),
        negative_cycle: false,
    });

    // ── Kahn's BFS 메인 루프 ────────────────────────────────────────
    while let Some(node) = queue.pop_front() {
        visited_flags[node] = true;
        visited_order.push(node);

        snapshots.push(StateSnapshot {
            current_line: 2,
            current_node: node,
            queue_or_stack: queue.iter().cloned().collect(),
            visited_order: visited_order.clone(),
            visited_flags: visited_flags.clone(),
            distances: None,
            log_message: format!(
                "노드 {} 위상 정렬 결과에 추가 (현재 순서: {:?})",
                node, visited_order
            ),
            negative_cycle: false,
        });

        // 인접 노드의 진입 차수 감소
        for edge in &graph.edges {
            if edge.from == node {
                edge_traversal_count += 1;
                let neighbor = edge.to;
                if neighbor < n {
                    in_degree[neighbor] -= 1;

                    snapshots.push(StateSnapshot {
                        current_line: 3,
                        current_node: neighbor,
                        queue_or_stack: queue.iter().cloned().collect(),
                        visited_order: visited_order.clone(),
                        visited_flags: visited_flags.clone(),
                        distances: None,
                        log_message: format!(
                            "노드 {} → {} 처리: 노드 {}의 진입 차수 = {}",
                            node, neighbor, neighbor, in_degree[neighbor]
                        ),
                        negative_cycle: false,
                    });

                    if in_degree[neighbor] == 0 {
                        queue.push_back(neighbor);

                        snapshots.push(StateSnapshot {
                            current_line: 4,
                            current_node: neighbor,
                            queue_or_stack: queue.iter().cloned().collect(),
                            visited_order: visited_order.clone(),
                            visited_flags: visited_flags.clone(),
                            distances: None,
                            log_message: format!(
                                "노드 {}의 진입 차수가 0이 됨 → 큐에 삽입",
                                neighbor
                            ),
                            negative_cycle: false,
                        });
                    }
                }
            }
        }
    }

    // ── 사이클 감지 ──────────────────────────────────────────────────
    let has_cycle = visited_order.len() < n;
    if has_cycle {
        snapshots.push(StateSnapshot {
            current_line: 5,
            current_node: 0,
            queue_or_stack: vec![],
            visited_order: visited_order.clone(),
            visited_flags: visited_flags.clone(),
            distances: None,
            log_message: format!(
                "⚠️  사이클 감지! 처리된 노드 수({})가 전체 노드 수({})보다 적습니다. \
                 위상 정렬 불가능 (DAG가 아님)",
                visited_order.len(),
                n
            ),
            negative_cycle: true, // 사이클 감지 플래그 재활용
        });
    } else {
        snapshots.push(StateSnapshot {
            current_line: 5,
            current_node: 0,
            queue_or_stack: vec![],
            visited_order: visited_order.clone(),
            visited_flags: visited_flags.clone(),
            distances: None,
            log_message: format!(
                "위상 정렬 완료: {:?}",
                visited_order
            ),
            negative_cycle: false,
        });
    }

    AlgorithmResult {
        kind: AlgorithmKind::TopologicalSort,
        snapshots,
        visit_order: visited_order,
        edge_traversal_count,
    }
}

// ────────────────────────────────────────────────────────────────────────────
// DFS 기반 위상 정렬
// ────────────────────────────────────────────────────────────────────────────

/// DFS를 이용한 위상 정렬
///
/// 각 노드를 DFS로 탐색하고, 탐색이 완료된(후처리) 노드를 스택에 삽입합니다.
/// 모든 DFS가 끝난 뒤 스택을 역순으로 읽으면 위상 정렬 순서가 됩니다.
pub fn run_dfs_based(graph: &Graph, _start: usize) -> AlgorithmResult {
    let n = graph.nodes.len();
    if n == 0 {
        return AlgorithmResult {
            kind: AlgorithmKind::TopologicalSort,
            ..Default::default()
        };
    }

    let mut snapshots: Vec<StateSnapshot> = Vec::new();
    let mut visited_flags = vec![false; n];
    let mut on_stack = vec![false; n]; // 현재 DFS 경로에 있는 노드
    let mut result_stack: Vec<usize> = Vec::new(); // 완료 순서
    let mut edge_traversal_count = 0usize;
    let mut has_cycle = false;

    snapshots.push(StateSnapshot {
        current_line: 1,
        current_node: 0,
        queue_or_stack: vec![],
        visited_order: vec![],
        visited_flags: visited_flags.clone(),
        distances: None,
        log_message: "DFS 기반 위상 정렬 시작".to_string(),
        negative_cycle: false,
    });

    for start_node in 0..n {
        if !visited_flags[start_node] {
            dfs_topo_visit(
                graph,
                start_node,
                &mut visited_flags,
                &mut on_stack,
                &mut result_stack,
                &mut snapshots,
                &mut edge_traversal_count,
                &mut has_cycle,
            );
        }
    }

    // 스택 역순이 위상 정렬 결과
    result_stack.reverse();

    AlgorithmResult {
        kind: AlgorithmKind::TopologicalSort,
        snapshots,
        visit_order: result_stack,
        edge_traversal_count,
    }
}

/// DFS 재귀 방문 — 반복적(iterative) 구현으로 스택 오버플로우 방지
fn dfs_topo_visit(
    graph: &Graph,
    start: usize,
    visited_flags: &mut Vec<bool>,
    on_stack: &mut Vec<bool>,
    result_stack: &mut Vec<usize>,
    snapshots: &mut Vec<StateSnapshot>,
    edge_traversal_count: &mut usize,
    has_cycle: &mut bool,
) {
    // (node, neighbor_index) — 반복적 DFS를 위한 명시적 스택
    let mut call_stack: Vec<(usize, usize)> = vec![(start, 0)];
    visited_flags[start] = true;
    on_stack[start] = true;

    snapshots.push(StateSnapshot {
        current_line: 2,
        current_node: start,
        queue_or_stack: call_stack.iter().map(|(n, _)| *n).collect(),
        visited_order: result_stack.clone(),
        visited_flags: visited_flags.clone(),
        distances: None,
        log_message: format!("노드 {} DFS 방문 시작", start),
        negative_cycle: false,
    });

    while let Some((node, neighbor_idx)) = call_stack.last_mut() {
        let node = *node;
        let neighbors: Vec<usize> = graph.edges.iter()
            .filter(|e| e.from == node)
            .map(|e| e.to)
            .collect();

        if *neighbor_idx < neighbors.len() {
            let next = neighbors[*neighbor_idx];
            *neighbor_idx += 1;
            *edge_traversal_count += 1;

            if on_stack[next] {
                // 사이클 감지
                *has_cycle = true;
                snapshots.push(StateSnapshot {
                    current_line: 3,
                    current_node: next,
                    queue_or_stack: call_stack.iter().map(|(n, _)| *n).collect(),
                    visited_order: result_stack.clone(),
                    visited_flags: visited_flags.clone(),
                    distances: None,
                    log_message: format!(
                        "⚠️  사이클 감지: 노드 {} → {} (이미 현재 경로에 존재)",
                        node, next
                    ),
                    negative_cycle: true,
                });
            } else if !visited_flags[next] {
                visited_flags[next] = true;
                on_stack[next] = true;
                call_stack.push((next, 0));

                snapshots.push(StateSnapshot {
                    current_line: 4,
                    current_node: next,
                    queue_or_stack: call_stack.iter().map(|(n, _)| *n).collect(),
                    visited_order: result_stack.clone(),
                    visited_flags: visited_flags.clone(),
                    distances: None,
                    log_message: format!("노드 {} → {} 재귀 방문", node, next),
                    negative_cycle: false,
                });
            }
        } else {
            // 현재 노드의 모든 이웃 처리 완료 → 결과 스택에 추가
            call_stack.pop();
            on_stack[node] = false;
            result_stack.push(node);

            snapshots.push(StateSnapshot {
                current_line: 5,
                current_node: node,
                queue_or_stack: call_stack.iter().map(|(n, _)| *n).collect(),
                visited_order: result_stack.clone(),
                visited_flags: visited_flags.clone(),
                distances: None,
                log_message: format!(
                    "노드 {} 처리 완료 → 결과 스택에 추가 (현재 스택: {:?})",
                    node, result_stack
                ),
                negative_cycle: false,
            });
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// 공개 API — AlgorithmResult 반환
// ────────────────────────────────────────────────────────────────────────────

/// 위상 정렬 실행 (기본: Kahn's Algorithm)
pub fn run(graph: &Graph, start: usize) -> AlgorithmResult {
    run_kahn(graph, start)
}

// ────────────────────────────────────────────────────────────────────────────
// 유틸리티 함수
// ────────────────────────────────────────────────────────────────────────────

/// 그래프가 DAG인지 빠르게 확인 (사이클 없는 방향 그래프)
pub fn is_dag(graph: &Graph) -> bool {
    let n = graph.nodes.len();
    if n == 0 {
        return true;
    }

    let mut in_degree = vec![0usize; n];
    for edge in &graph.edges {
        if edge.to < n {
            in_degree[edge.to] += 1;
        }
    }

    let mut queue: VecDeque<usize> = (0..n).filter(|&i| in_degree[i] == 0).collect();
    let mut processed = 0usize;

    while let Some(node) = queue.pop_front() {
        processed += 1;
        for edge in &graph.edges {
            if edge.from == node && edge.to < n {
                in_degree[edge.to] -= 1;
                if in_degree[edge.to] == 0 {
                    queue.push_back(edge.to);
                }
            }
        }
    }

    processed == n
}

/// 위상 정렬 결과로부터 각 노드의 레벨(층) 계산
///
/// 같은 레벨의 노드들은 병렬로 처리될 수 있습니다.
pub fn compute_levels(graph: &Graph) -> Vec<usize> {
    let n = graph.nodes.len();
    if n == 0 {
        return vec![];
    }

    let mut levels = vec![0usize; n];
    let mut in_degree = vec![0usize; n];

    for edge in &graph.edges {
        if edge.to < n {
            in_degree[edge.to] += 1;
        }
    }

    let mut queue: VecDeque<usize> = (0..n).filter(|&i| in_degree[i] == 0).collect();

    while let Some(node) = queue.pop_front() {
        for edge in &graph.edges {
            if edge.from == node && edge.to < n {
                let next = edge.to;
                levels[next] = levels[next].max(levels[node] + 1);
                in_degree[next] -= 1;
                if in_degree[next] == 0 {
                    queue.push_back(next);
                }
            }
        }
    }

    levels
}

// ────────────────────────────────────────────────────────────────────────────
// 수도코드 텍스트 (UI 표시용)
// ────────────────────────────────────────────────────────────────────────────

/// Kahn's Algorithm 수도코드 (UI 하이라이트용)
pub fn kahn_pseudocode() -> Vec<&'static str> {
    vec![
        "1: in_degree[] 계산; 진입차수=0 노드 → queue",
        "2: node = queue.pop(); result.append(node)",
        "3: for neighbor in adj[node]: in_degree[neighbor] -= 1",
        "4: if in_degree[neighbor] == 0: queue.push(neighbor)",
        "5: if len(result) < V: 사이클 존재",
    ]
}

/// DFS 기반 수도코드 (UI 하이라이트용)
pub fn dfs_pseudocode() -> Vec<&'static str> {
    vec![
        "1: for node in V: if not visited: dfs(node)",
        "2: visited[node] = true; on_stack[node] = true",
        "3: if neighbor on_stack: 사이클 감지",
        "4: if not visited: dfs(neighbor)",
        "5: on_stack[node] = false; result_stack.push(node)",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Graph;

    fn make_dag() -> Graph {
        let mut g = Graph::new();
        // 노드 0,1,2,3,4 추가
        for i in 0..5 {
            g.add_node(i as f32 * 50.0, 100.0, format!("{}", i));
        }
        // DAG: 0→1, 0→2, 1→3, 2→3, 3→4
        g.is_directed = true;
        g.add_edge(0, 1, 1.0);
        g.add_edge(0, 2, 1.0);
        g.add_edge(1, 3, 1.0);
        g.add_edge(2, 3, 1.0);
        g.add_edge(3, 4, 1.0);
        g
    }

    fn make_cyclic() -> Graph {
        let mut g = Graph::new();
        for i in 0..3 {
            g.add_node(i as f32 * 50.0, 100.0, format!("{}", i));
        }
        g.is_directed = true;
        g.add_edge(0, 1, 1.0);
        g.add_edge(1, 2, 1.0);
        g.add_edge(2, 0, 1.0); // 사이클
        g
    }

    #[test]
    fn test_kahn_dag() {
        let g = make_dag();
        let result = run_kahn(&g, 0);
        assert_eq!(result.visit_order.len(), 5);
        // 4가 항상 마지막이어야 함
        assert_eq!(*result.visit_order.last().unwrap(), 4);
    }

    #[test]
    fn test_is_dag_true() {
        let g = make_dag();
        assert!(is_dag(&g));
    }

    #[test]
    fn test_is_dag_false() {
        let g = make_cyclic();
        assert!(!is_dag(&g));
    }

    #[test]
    fn test_compute_levels() {
        let g = make_dag();
        let levels = compute_levels(&g);
        assert_eq!(levels[4], 3); // 노드 4는 레벨 3
        assert_eq!(levels[0], 0); // 노드 0은 레벨 0
    }

    #[test]
    fn test_dfs_based_dag() {
        let g = make_dag();
        let result = run_dfs_based(&g, 0);
        assert_eq!(result.visit_order.len(), 5);
        assert_eq!(*result.visit_order.last().unwrap(), 4);
    }
}
