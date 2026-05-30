//! Floyd-Warshall(플로이드-워셜) 전체 쌍 최단 경로 구현
//! - 모든 노드 쌍 간의 최단 거리 계산 O(V³)
//! - 음수 가중치 지원 (음수 사이클 감지 포함)

use crate::graph::Graph;
use super::state::{AlgorithmKind, AlgorithmResult, StateSnapshot};

pub fn run(graph: &Graph, start: usize) -> AlgorithmResult {
    let n = graph.nodes.len();
    if n == 0 {
        return AlgorithmResult { kind: AlgorithmKind::FloydWarshall, ..Default::default() };
    }

    let inf = f64::INFINITY;

    // dist[i][j]: i → j 최단 거리 (1차원으로 직렬화: idx = i*n + j)
    let mut dist = vec![inf; n * n];
    let mut snapshots = vec![];
    let mut edge_traversal_count = 0usize;

    // 자기 자신은 0
    for i in 0..n {
        dist[i * n + i] = 0.0;
    }

    // 간선으로 초기 거리 설정
    for edge in &graph.edges {
        let i = edge.from;
        let j = edge.to;
        if edge.weight < dist[i * n + j] {
            dist[i * n + j] = edge.weight;
        }
        if !graph.is_directed && edge.weight < dist[j * n + i] {
            dist[j * n + i] = edge.weight;
        }
    }

    // start 노드 기준 1D 거리 배열 추출 헬퍼
    let row_dist = |d: &[f64], from: usize| -> Vec<f64> {
        (0..n).map(|j| d[from * n + j]).collect()
    };

    snapshots.push(StateSnapshot {
        current_line: 0,
        current_node: start,
        queue_or_stack: vec![],
        visited_order: vec![],
        visited_flags: vec![false; n],
        distances: Some(row_dist(&dist, start)),
        log_message: "인접 행렬 초기화 완료. 중간 노드 k 순회 시작".to_string(),
        negative_cycle: false,
    });

    // 핵심 삼중 루프
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                edge_traversal_count += 1;
                let via_k = dist[i * n + k].saturating_add_f64(dist[k * n + j]);
                if via_k < dist[i * n + j] {
                    dist[i * n + j] = via_k;

                    // 스냅샷은 start 노드 기준 행만 기록 (전체 n×n 저장은 과부하)
                    if i == start || j == start {
                        snapshots.push(StateSnapshot {
                            current_line: 2,
                            current_node: k,
                            queue_or_stack: vec![k],
                            visited_order: (0..=k).collect(),
                            visited_flags: {
                                let mut f = vec![false; n];
                                for x in 0..=k { f[x] = true; }
                                f
                            },
                            distances: Some(row_dist(&dist, start)),
                            log_message: format!(
                                "[k={}] dist[{}][{}] 갱신 → {:.2} (경유: {})",
                                k, i, j, via_k, k
                            ),
                            negative_cycle: false,
                        });
                    }
                }
            }
        }

        // 중간 노드 k 완료 스냅샷
        snapshots.push(StateSnapshot {
            current_line: 1,
            current_node: k,
            queue_or_stack: vec![],
            visited_order: (0..=k).collect(),
            visited_flags: {
                let mut f = vec![false; n];
                for x in 0..=k { f[x] = true; }
                f
            },
            distances: Some(row_dist(&dist, start)),
            log_message: format!("중간 노드 k={} 처리 완료", k),
            negative_cycle: false,
        });
    }

    // 음수 사이클 감지: dist[i][i] < 0
    let has_negative_cycle = (0..n).any(|i| dist[i * n + i] < 0.0);

    let visited_flags = vec![true; n];
    let visited_order: Vec<usize> = (0..n).collect();

    snapshots.push(StateSnapshot {
        current_line: 3,
        current_node: start,
        queue_or_stack: vec![],
        visited_order: visited_order.clone(),
        visited_flags: visited_flags.clone(),
        distances: Some(row_dist(&dist, start)),
        log_message: if has_negative_cycle {
            "⚠ 음수 사이클 감지됨!".to_string()
        } else {
            format!("✅ Floyd-Warshall 완료 — 노드 {} 기준 최단 거리 표시 중", start)
        },
        negative_cycle: has_negative_cycle,
    });

    AlgorithmResult {
        kind: AlgorithmKind::FloydWarshall,
        snapshots,
        visit_order: visited_order,
        edge_traversal_count,
    }
}

// f64에 None-safe 덧셈 (∞ + 유한 = ∞)
trait SaturatingAddF64 {
    fn saturating_add_f64(self, rhs: f64) -> f64;
}
impl SaturatingAddF64 for f64 {
    fn saturating_add_f64(self, rhs: f64) -> f64 {
        if self.is_infinite() || rhs.is_infinite() { f64::INFINITY } else { self + rhs }
    }
}