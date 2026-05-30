//! Kruskal(크루스칼) 최소 신장 트리 구현
//! - Union-Find(서로소 집합) 기반 O(E log E)
//! - 간선을 가중치 오름차순으로 정렬 후 사이클 없이 추가

use crate::graph::Graph;
use super::state::{AlgorithmKind, AlgorithmResult, StateSnapshot};

// ── Union-Find ───────────────────────────────────────────────────────────────

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // 경로 압축
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx == ry {
            return false; // 이미 같은 집합 → 사이클
        }
        // rank 기반 합집합
        match self.rank[rx].cmp(&self.rank[ry]) {
            std::cmp::Ordering::Less    => self.parent[rx] = ry,
            std::cmp::Ordering::Greater => self.parent[ry] = rx,
            std::cmp::Ordering::Equal   => { self.parent[ry] = rx; self.rank[rx] += 1; }
        }
        true
    }
}

// ── 알고리즘 본체 ─────────────────────────────────────────────────────────────

pub fn run(graph: &Graph, start: usize) -> AlgorithmResult {
    let n = graph.nodes.len();
    if n == 0 {
        return AlgorithmResult { kind: AlgorithmKind::Kruskal, ..Default::default() };
    }

    // 간선 목록 수집 (무방향이면 중복 제거: from < to 기준)
    let mut edges: Vec<(f64, usize, usize)> = graph.edges.iter()
        .map(|e| (e.weight, e.from, e.to))
        .collect();
    if !graph.is_directed {
        edges.retain(|(_, f, t)| f <= t);
    }

    // 가중치 오름차순 정렬
    edges.sort_by(|a, b| a.0.total_cmp(&b.0));

    let mut uf = UnionFind::new(n);
    let mut visited_flags = vec![false; n];
    let mut visited_order = vec![];
    let mut snapshots = vec![];
    let mut edge_traversal_count = 0usize;
    let mut mst_cost = 0.0f64;
    let mut mst_edges_added = 0usize;

    // 정렬된 간선 목록을 queue_or_stack으로 표현 (from 노드 ID 나열)
    let initial_queue: Vec<usize> = edges.iter().map(|(_, f, _)| *f).collect();

    snapshots.push(StateSnapshot {
        current_line: 0,
        current_node: start,
        queue_or_stack: initial_queue,
        visited_order: visited_order.clone(),
        visited_flags: visited_flags.clone(),
        distances: None,
        log_message: format!(
            "간선 {} 개를 가중치 오름차순 정렬 완료. Union-Find 초기화",
            edges.len()
        ),
        negative_cycle: false,
    });

    for (weight, from, to) in &edges {
        edge_traversal_count += 1;

        let can_add = uf.union(*from, *to);

        if can_add {
            // MST에 포함
            mst_cost += weight;
            mst_edges_added += 1;

            if !visited_flags[*from] {
                visited_flags[*from] = true;
                visited_order.push(*from);
            }
            if !visited_flags[*to] {
                visited_flags[*to] = true;
                visited_order.push(*to);
            }

            snapshots.push(StateSnapshot {
                current_line: 1,
                current_node: *from,
                queue_or_stack: vec![*from, *to],
                visited_order: visited_order.clone(),
                visited_flags: visited_flags.clone(),
                distances: None,
                log_message: format!(
                    "✅ 간선 {}→{} (가중치 {:.2}) MST에 추가 | 누적={:.2} ({}개째)",
                    from, to, weight, mst_cost, mst_edges_added
                ),
                negative_cycle: false,
            });

            // MST 완성 조건: V-1 개 간선
            if mst_edges_added == n - 1 {
                break;
            }
        } else {
            snapshots.push(StateSnapshot {
                current_line: 2,
                current_node: *from,
                queue_or_stack: vec![*from, *to],
                visited_order: visited_order.clone(),
                visited_flags: visited_flags.clone(),
                distances: None,
                log_message: format!(
                    "⛔ 간선 {}→{} (가중치 {:.2}) 사이클 형성 → 건너뜀",
                    from, to, weight
                ),
                negative_cycle: false,
            });
        }
    }

    // 미연결 노드(고립 노드)도 visited 처리
    for i in 0..n {
        if !visited_flags[i] {
            visited_flags[i] = true;
            visited_order.push(i);
        }
    }

    snapshots.push(StateSnapshot {
        current_line: 3,
        current_node: start,
        queue_or_stack: vec![],
        visited_order: visited_order.clone(),
        visited_flags: visited_flags.clone(),
        distances: None,
        log_message: format!(
            "✅ Kruskal MST 완료 — 간선 {} 개, 총 비용: {:.2}",
            mst_edges_added, mst_cost
        ),
        negative_cycle: false,
    });

    AlgorithmResult {
        kind: AlgorithmKind::Kruskal,
        snapshots,
        visit_order: visited_order,
        edge_traversal_count,
    }
}