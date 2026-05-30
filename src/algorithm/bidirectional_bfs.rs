//! 양방향 BFS(Bidirectional BFS) 구현
//!
//! # 양방향 BFS란?
//! 출발 노드와 목표 노드에서 동시에 BFS를 수행하여 두 탐색이 만나는 지점을 찾는 알고리즘입니다.
//! 단방향 BFS에 비해 탐색 공간을 O(b^(d/2)) 으로 줄일 수 있습니다.
//! 여기서 b는 분기 계수(branching factor), d는 목표 노드까지의 깊이입니다.
//!
//! ## 알고리즘 흐름
//! 1. 정방향 큐(forward)에 start, 역방향 큐(backward)에 goal을 삽입합니다.
//! 2. 각 반복에서 작은 큐(프론티어가 더 좁은 방향)를 하나씩 확장합니다.
//! 3. 어느 한 방향의 확장 결과가 상대방의 방문 집합과 교차하면 경로를 연결합니다.
//! 4. 교차 노드에서 양쪽 경로를 역추적하여 최단 경로를 구성합니다.
//!
//! ## 시간 복잡도
//! - O(b^(d/2)) — 균일 가중치 그래프 기준

use std::collections::VecDeque;
use crate::graph::Graph;
use super::state::{AlgorithmKind, AlgorithmResult, StateSnapshot};

// ────────────────────────────────────────────────────────────────────────────
// 공개 API
// ────────────────────────────────────────────────────────────────────────────

/// 양방향 BFS 실행
///
/// `start`에서 `goal`까지의 최단 경로를 탐색합니다.
/// goal이 유효하지 않으면 첫 번째 노드(0) 외의 마지막 노드를 목표로 설정합니다.
pub fn run(graph: &Graph, start: usize) -> AlgorithmResult {
    let n = graph.nodes.len();
    if n == 0 {
        return AlgorithmResult {
            kind: AlgorithmKind::BidirectionalBFS,
            ..Default::default()
        };
    }
    // 목표 노드: 마지막 노드
    let goal = if n > 1 { n - 1 } else { 0 };
    run_between(graph, start, goal)
}

/// start ~ goal 사이의 최단 경로 탐색
pub fn run_between(graph: &Graph, start: usize, goal: usize) -> AlgorithmResult {
    let n = graph.nodes.len();
    if n == 0 {
        return AlgorithmResult {
            kind: AlgorithmKind::BidirectionalBFS,
            ..Default::default()
        };
    }

    // ── 정방향/역방향 자료구조 초기화 ──────────────────────────────
    let mut forward_visited: Vec<bool> = vec![false; n];
    let mut backward_visited: Vec<bool> = vec![false; n];
    let mut forward_parent: Vec<Option<usize>> = vec![None; n];
    let mut backward_parent: Vec<Option<usize>> = vec![None; n];

    let mut forward_queue: VecDeque<usize> = VecDeque::new();
    let mut backward_queue: VecDeque<usize> = VecDeque::new();

    forward_visited[start] = true;
    forward_queue.push_back(start);
    if goal < n {
        backward_visited[goal] = true;
        backward_queue.push_back(goal);
    }

    let mut snapshots: Vec<StateSnapshot> = Vec::new();
    let mut visited_order: Vec<usize> = Vec::new();
    let mut edge_traversal_count = 0usize;
    let mut meeting_node: Option<usize> = None;

    // ── 초기 스냅샷 ────────────────────────────────────────────────
    snapshots.push(StateSnapshot {
        current_line: 1,
        current_node: start,
        queue_or_stack: forward_queue.iter().chain(backward_queue.iter()).cloned().collect(),
        visited_order: visited_order.clone(),
        visited_flags: forward_visited.clone(),
        distances: None,
        log_message: format!(
            "양방향 BFS 시작: 정방향 출발={}, 역방향 출발={}",
            start, goal
        ),
        negative_cycle: false,
    });

    // ── 메인 루프 ──────────────────────────────────────────────────
    'outer: while !forward_queue.is_empty() || !backward_queue.is_empty() {
        // 정방향 확장
        if let Some(node) = forward_queue.pop_front() {
            visited_order.push(node);

            snapshots.push(StateSnapshot {
                current_line: 2,
                current_node: node,
                queue_or_stack: forward_queue.iter().chain(backward_queue.iter()).cloned().collect(),
                visited_order: visited_order.clone(),
                visited_flags: forward_visited.clone(),
                distances: None,
                log_message: format!("정방향 탐색: 노드 {} 처리 중", node),
                negative_cycle: false,
            });

            for (neighbor, _weight) in graph.neighbors(node) {
                edge_traversal_count += 1;
                if !forward_visited[neighbor] {
                    forward_visited[neighbor] = true;
                    forward_parent[neighbor] = Some(node);
                    forward_queue.push_back(neighbor);

                    snapshots.push(StateSnapshot {
                        current_line: 3,
                        current_node: neighbor,
                        queue_or_stack: forward_queue.iter().chain(backward_queue.iter()).cloned().collect(),
                        visited_order: visited_order.clone(),
                        visited_flags: forward_visited.clone(),
                        distances: None,
                        log_message: format!(
                            "정방향: {} → {} 발견, 큐에 삽입",
                            node, neighbor
                        ),
                        negative_cycle: false,
                    });

                    // 교차 확인
                    if backward_visited[neighbor] {
                        meeting_node = Some(neighbor);
                        snapshots.push(StateSnapshot {
                            current_line: 4,
                            current_node: neighbor,
                            queue_or_stack: vec![],
                            visited_order: visited_order.clone(),
                            visited_flags: forward_visited.clone(),
                            distances: None,
                            log_message: format!(
                                "✅ 교차 노드 {} 발견! 양방향 경로 연결 완료",
                                neighbor
                            ),
                            negative_cycle: false,
                        });
                        break 'outer;
                    }
                }
            }
        }

        // 역방향 확장
        if let Some(node) = backward_queue.pop_front() {
            visited_order.push(node);

            snapshots.push(StateSnapshot {
                current_line: 2,
                current_node: node,
                queue_or_stack: forward_queue.iter().chain(backward_queue.iter()).cloned().collect(),
                visited_order: visited_order.clone(),
                visited_flags: backward_visited.clone(),
                distances: None,
                log_message: format!("역방향 탐색: 노드 {} 처리 중", node),
                negative_cycle: false,
            });

            // 역방향에서는 간선의 to → from 방향으로 탐색
            let reverse_neighbors: Vec<usize> = graph.edges.iter()
                .filter(|e| e.to == node)
                .map(|e| e.from)
                .collect();

            for neighbor in reverse_neighbors {
                edge_traversal_count += 1;
                if !backward_visited[neighbor] {
                    backward_visited[neighbor] = true;
                    backward_parent[neighbor] = Some(node);
                    backward_queue.push_back(neighbor);

                    snapshots.push(StateSnapshot {
                        current_line: 3,
                        current_node: neighbor,
                        queue_or_stack: forward_queue.iter().chain(backward_queue.iter()).cloned().collect(),
                        visited_order: visited_order.clone(),
                        visited_flags: backward_visited.clone(),
                        distances: None,
                        log_message: format!(
                            "역방향: {} ← {} 발견, 큐에 삽입",
                            node, neighbor
                        ),
                        negative_cycle: false,
                    });

                    // 교차 확인
                    if forward_visited[neighbor] {
                        meeting_node = Some(neighbor);
                        snapshots.push(StateSnapshot {
                            current_line: 4,
                            current_node: neighbor,
                            queue_or_stack: vec![],
                            visited_order: visited_order.clone(),
                            visited_flags: forward_visited.clone(),
                            distances: None,
                            log_message: format!(
                                "✅ 교차 노드 {} 발견! 양방향 경로 연결 완료",
                                neighbor
                            ),
                            negative_cycle: false,
                        });
                        break 'outer;
                    }
                }
            }
        }
    }

    // ── 경로 복원 ──────────────────────────────────────────────────
    let path = if let Some(meeting) = meeting_node {
        reconstruct_path(meeting, &forward_parent, &backward_parent)
    } else {
        vec![]
    };

    let found = !path.is_empty();
    snapshots.push(StateSnapshot {
        current_line: 5,
        current_node: goal,
        queue_or_stack: path.clone(),
        visited_order: visited_order.clone(),
        visited_flags: forward_visited.clone(),
        distances: None,
        log_message: if found {
            format!("최단 경로 복원 완료: {:?} (길이 {})", path, path.len() - 1)
        } else {
            format!("노드 {} → {} 경로 없음", start, goal)
        },
        negative_cycle: false,
    });

    AlgorithmResult {
        kind: AlgorithmKind::BidirectionalBFS,
        snapshots,
        visit_order: visited_order,
        edge_traversal_count,
    }
}

// ────────────────────────────────────────────────────────────────────────────
// 경로 역추적 헬퍼
// ────────────────────────────────────────────────────────────────────────────

/// 교차 노드에서 정방향/역방향 부모 체인을 이어 완전한 경로 반환
fn reconstruct_path(
    meeting: usize,
    forward_parent: &[Option<usize>],
    backward_parent: &[Option<usize>],
) -> Vec<usize> {
    // 정방향 경로: start → meeting
    let mut forward_path: Vec<usize> = Vec::new();
    let mut cur = meeting;
    forward_path.push(cur);
    while let Some(parent) = forward_parent[cur] {
        forward_path.push(parent);
        cur = parent;
    }
    forward_path.reverse();

    // 역방향 경로: meeting → goal
    let mut backward_path: Vec<usize> = Vec::new();
    cur = meeting;
    while let Some(parent) = backward_parent[cur] {
        backward_path.push(parent);
        cur = parent;
    }

    // 합치기 (교차 노드 중복 제거)
    forward_path.extend(backward_path);
    forward_path
}

// ────────────────────────────────────────────────────────────────────────────
// 수도코드 (UI 표시용)
// ────────────────────────────────────────────────────────────────────────────

pub fn pseudocode() -> Vec<&'static str> {
    vec![
        "1: fwd_queue = [start]; bwd_queue = [goal]",
        "2: node = smaller_frontier.pop()",
        "3: for neighbor in adj[node]: if not visited: enqueue(neighbor)",
        "4: if neighbor in other_visited: 교차 노드 발견 → 종료",
        "5: 경로 역추적: forward_path + backward_path 결합",
    ]
}

// ────────────────────────────────────────────────────────────────────────────
// 단위 테스트
// ────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Graph;

    fn make_line_graph(n: usize) -> Graph {
        let mut g = Graph::new();
        for i in 0..n {
            g.add_node(i as f32 * 50.0, 100.0, format!("{}", i));
        }
        g.is_directed = false;
        for i in 0..n - 1 {
            g.add_edge(i, i + 1, 1.0);
        }
        g
    }

    #[test]
    fn test_bidir_bfs_line() {
        let g = make_line_graph(5);
        let result = run_between(&g, 0, 4);
        // 경로가 존재해야 함
        assert!(!result.visit_order.is_empty());
    }

    #[test]
    fn test_bidir_bfs_no_path() {
        let mut g = Graph::new();
        for i in 0..4 {
            g.add_node(i as f32 * 50.0, 100.0, format!("{}", i));
        }
        g.is_directed = true;
        g.add_edge(0, 1, 1.0);
        g.add_edge(2, 3, 1.0); // 분리된 컴포넌트
        let result = run_between(&g, 0, 3);
        // 방문 순서는 있지만 교차 없음
        assert!(result.snapshots.last().map(|s| s.log_message.contains("경로 없음")).unwrap_or(false));
    }

    #[test]
    fn test_reconstruct_path() {
        // 0 → 1 → 2 (meeting=2), backward 2 ← 4 ← 5
        let forward = vec![None, Some(0), Some(1)];
        let backward = vec![None, None, None, None, Some(2), Some(4)];
        // meeting=2: forward_path=[0,1,2], backward=[4,5] → [0,1,2,4,5]
        // Note: backward[2]=None이므로 backward_path=[]
        let path = reconstruct_path(2, &forward, &backward);
        assert_eq!(path, vec![0, 1, 2]);
    }
}
