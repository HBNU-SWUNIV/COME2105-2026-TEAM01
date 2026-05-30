//! 알고리즘 성능 벤치마크 실행기
//!
//! 각 알고리즘을 동일한 그래프에 반복 실행하여 평균 실행 시간, 표준편차,
//! 최솟값/최댓값을 측정합니다. 결과는 `BenchmarkReport`로 반환되며
//! 대시보드 패널에서 시각화됩니다.
//!
//! ## 측정 방식
//! - `std::time::Instant`를 활용한 wall-clock 시간 측정
//! - 동일 조건에서 N회 반복 후 통계 계산
//! - 측정 전 warm-up 실행으로 CPU 캐시 효과 제거
//!
//! ## 주의
//! - 현재 구현은 단일 스레드 기준입니다.
//! - 매우 큰 그래프(노드 10만+)는 Floyd-Warshall 실행 시 수분이 걸릴 수 있습니다.

use std::time::{Duration, Instant};
use crate::graph::Graph;
use crate::algorithm::{run_algorithm, AlgorithmKind};

// ────────────────────────────────────────────────────────────────────────────
// 결과 구조체
// ────────────────────────────────────────────────────────────────────────────

/// 단일 알고리즘의 벤치마크 결과
#[derive(Debug, Clone)]
pub struct AlgoBenchResult {
    /// 측정된 알고리즘
    pub kind: AlgorithmKind,
    /// 반복 횟수
    pub iterations: usize,
    /// 평균 실행 시간 (ms)
    pub mean_ms: f64,
    /// 표준 편차 (ms)
    pub std_dev_ms: f64,
    /// 최솟값 (ms)
    pub min_ms: f64,
    /// 최댓값 (ms)
    pub max_ms: f64,
    /// 중앙값 (ms)
    pub median_ms: f64,
    /// 전체 측정 샘플 (ms)
    pub samples: Vec<f64>,
    /// 벤치마크 실행 시점 노드/간선 수
    pub node_count: usize,
    pub edge_count: usize,
}

impl AlgoBenchResult {
    /// 95번째 백분위수 (P95) 지연 시간 반환
    pub fn p95_ms(&self) -> f64 {
        let mut sorted = self.samples.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let idx = ((sorted.len() as f64 * 0.95) as usize).min(sorted.len().saturating_sub(1));
        sorted.get(idx).copied().unwrap_or(0.0)
    }

    /// 99번째 백분위수 (P99) 지연 시간 반환
    pub fn p99_ms(&self) -> f64 {
        let mut sorted = self.samples.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let idx = ((sorted.len() as f64 * 0.99) as usize).min(sorted.len().saturating_sub(1));
        sorted.get(idx).copied().unwrap_or(0.0)
    }

    /// 처리량 (iterations per second)
    pub fn throughput_per_sec(&self) -> f64 {
        if self.mean_ms == 0.0 { return f64::INFINITY; }
        1000.0 / self.mean_ms
    }

    /// 가독성 있는 요약 문자열 반환
    pub fn summary(&self) -> String {
        format!(
            "[{}] mean={:.3}ms ± {:.3}ms  min={:.3}ms  max={:.3}ms  p95={:.3}ms  (n={})",
            self.kind,
            self.mean_ms,
            self.std_dev_ms,
            self.min_ms,
            self.max_ms,
            self.p95_ms(),
            self.iterations
        )
    }
}

/// 여러 알고리즘에 대한 전체 비교 보고서
#[derive(Debug, Clone, Default)]
pub struct BenchmarkReport {
    pub results: Vec<AlgoBenchResult>,
    /// 보고서 생성 그래프 정보
    pub graph_node_count: usize,
    pub graph_edge_count: usize,
    /// 총 소요 시간 (모든 벤치마크 합산)
    pub total_elapsed_ms: f64,
}

impl BenchmarkReport {
    /// 가장 빠른 알고리즘 반환
    pub fn fastest(&self) -> Option<&AlgoBenchResult> {
        self.results.iter().min_by(|a, b| a.mean_ms.partial_cmp(&b.mean_ms).unwrap())
    }

    /// 가장 느린 알고리즘 반환
    pub fn slowest(&self) -> Option<&AlgoBenchResult> {
        self.results.iter().max_by(|a, b| a.mean_ms.partial_cmp(&b.mean_ms).unwrap())
    }

    /// 특정 알고리즘 결과 조회
    pub fn get(&self, kind: AlgorithmKind) -> Option<&AlgoBenchResult> {
        self.results.iter().find(|r| r.kind == kind)
    }

    /// 결과를 평균 시간 기준으로 정렬된 목록 반환
    pub fn sorted_by_mean(&self) -> Vec<&AlgoBenchResult> {
        let mut v: Vec<&AlgoBenchResult> = self.results.iter().collect();
        v.sort_by(|a, b| a.mean_ms.partial_cmp(&b.mean_ms).unwrap());
        v
    }

    /// 텍스트 요약
    pub fn text_summary(&self) -> String {
        let mut lines = vec![
            format!(
                "벤치마크 보고서 (노드={}, 간선={}, 총 소요={:.1}ms)",
                self.graph_node_count, self.graph_edge_count, self.total_elapsed_ms
            )
        ];
        for r in self.sorted_by_mean() {
            lines.push(format!("  {}", r.summary()));
        }
        lines.join("\n")
    }
}

// ────────────────────────────────────────────────────────────────────────────
// 벤치마크 실행기
// ────────────────────────────────────────────────────────────────────────────

/// 벤치마크 설정
#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    /// 워밍업 횟수 (결과에 포함되지 않음)
    pub warmup: usize,
    /// 측정 반복 횟수
    pub iterations: usize,
    /// Floyd-Warshall 등 무거운 알고리즘에 대한 제한 시간 (초)
    pub timeout_sec: f64,
    /// 측정에 포함할 알고리즘 목록
    pub algorithms: Vec<AlgorithmKind>,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            warmup: 2,
            iterations: 10,
            timeout_sec: 5.0,
            algorithms: vec![
                AlgorithmKind::BFS,
                AlgorithmKind::DFS,
                AlgorithmKind::Dijkstra,
                AlgorithmKind::BellmanFord,
                AlgorithmKind::FloydWarshall,
                AlgorithmKind::AStar,
                AlgorithmKind::Prim,
                AlgorithmKind::Kruskal,
            ],
        }
    }
}

/// 벤치마크 실행기
pub struct BenchmarkRunner {
    pub config: BenchmarkConfig,
}

impl BenchmarkRunner {
    pub fn new(config: BenchmarkConfig) -> Self {
        Self { config }
    }

    pub fn with_default() -> Self {
        Self::new(BenchmarkConfig::default())
    }

    /// 그래프에 대해 설정된 알고리즘 전체를 벤치마크합니다.
    pub fn run(&self, graph: &Graph, start_node: usize) -> BenchmarkReport {
        let bench_start = Instant::now();
        let mut results = Vec::new();

        for &kind in &self.config.algorithms {
            if let Some(result) = self.bench_single(graph, start_node, kind) {
                results.push(result);
            }
        }

        BenchmarkReport {
            results,
            graph_node_count: graph.nodes.len(),
            graph_edge_count: graph.edges.len(),
            total_elapsed_ms: bench_start.elapsed().as_secs_f64() * 1000.0,
        }
    }

    /// 단일 알고리즘 벤치마크 실행
    pub fn bench_single(
        &self,
        graph: &Graph,
        start_node: usize,
        kind: AlgorithmKind,
    ) -> Option<AlgoBenchResult> {
        let timeout = Duration::from_secs_f64(self.config.timeout_sec);
        let bench_start = Instant::now();

        // 워밍업
        for _ in 0..self.config.warmup {
            let _ = run_algorithm(graph, start_node, kind);
            if bench_start.elapsed() > timeout {
                return None; // 워밍업 단계에서 타임아웃
            }
        }

        // 측정
        let mut samples: Vec<f64> = Vec::with_capacity(self.config.iterations);
        for _ in 0..self.config.iterations {
            if bench_start.elapsed() > timeout {
                break; // 부분 결과라도 반환
            }
            let iter_start = Instant::now();
            let _ = run_algorithm(graph, start_node, kind);
            let elapsed_ms = iter_start.elapsed().as_secs_f64() * 1000.0;
            samples.push(elapsed_ms);
        }

        if samples.is_empty() {
            return None;
        }

        Some(compute_stats(kind, samples, graph.nodes.len(), graph.edges.len()))
    }
}

// ────────────────────────────────────────────────────────────────────────────
// 통계 계산 헬퍼
// ────────────────────────────────────────────────────────────────────────────

fn compute_stats(
    kind: AlgorithmKind,
    samples: Vec<f64>,
    node_count: usize,
    edge_count: usize,
) -> AlgoBenchResult {
    let n = samples.len();
    let mean_ms = samples.iter().sum::<f64>() / n as f64;

    let variance = samples.iter().map(|x| (x - mean_ms).powi(2)).sum::<f64>() / n as f64;
    let std_dev_ms = variance.sqrt();

    let min_ms = samples.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_ms = samples.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    let mut sorted = samples.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median_ms = if n % 2 == 0 {
        (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0
    } else {
        sorted[n / 2]
    };

    AlgoBenchResult {
        kind,
        iterations: n,
        mean_ms,
        std_dev_ms,
        min_ms,
        max_ms,
        median_ms,
        samples,
        node_count,
        edge_count,
    }
}

// ────────────────────────────────────────────────────────────────────────────
// 랜덤 그래프 생성 (벤치마크용)
// ────────────────────────────────────────────────────────────────────────────

/// 랜덤 그래프 생성 (벤치마크 테스트용 유틸리티)
///
/// 단순한 XOR 시프트 기반 의사난수 생성기를 사용합니다 (외부 의존성 없음).
pub fn generate_random_graph(node_count: usize, edge_density: f64, directed: bool, seed: u64) -> Graph {
    let mut graph = Graph::new();
    let mut rng = XorShift64::new(seed);

    // 노드 추가
    for i in 0..node_count {
        let x = (rng.next() % 800) as f32;
        let y = (rng.next() % 600) as f32;
        graph.add_node(x, y, format!("{}", i));
    }
    graph.is_directed = directed;

    // 간선 추가 (edge_density: 0.0 ~ 1.0)
    let max_edges = node_count * (node_count - 1) / 2;
    let target_edges = (max_edges as f64 * edge_density.clamp(0.0, 1.0)) as usize;

    let mut added = 0;
    let mut attempts = 0;
    while added < target_edges && attempts < target_edges * 10 {
        attempts += 1;
        let from = (rng.next() as usize) % node_count;
        let to = (rng.next() as usize) % node_count;
        if from == to {
            continue;
        }
        let weight = (rng.next() % 100) as f64 + 1.0;
        // 중복 간선 방지
        let exists = graph.edges.iter().any(|e| e.from == from && e.to == to);
        if !exists {
            graph.add_edge(from, to, weight);
            added += 1;
        }
    }

    graph
}

/// 완전 연결 그래프 생성 (모든 노드 쌍이 연결된 그래프)
pub fn generate_complete_graph(node_count: usize) -> Graph {
    let mut graph = Graph::new();
    for i in 0..node_count {
        let angle = (i as f64 * std::f64::consts::TAU) / node_count as f64;
        let x = (angle.cos() * 200.0 + 400.0) as f32;
        let y = (angle.sin() * 200.0 + 300.0) as f32;
        graph.add_node(x, y, format!("{}", i));
    }
    graph.is_directed = false;
    for i in 0..node_count {
        for j in (i + 1)..node_count {
            graph.add_edge(i, j, 1.0);
        }
    }
    graph
}

/// 격자 그래프 생성 (n×n)
pub fn generate_grid_graph(rows: usize, cols: usize) -> Graph {
    let mut graph = Graph::new();
    for r in 0..rows {
        for c in 0..cols {
            let x = c as f32 * 60.0 + 50.0;
            let y = r as f32 * 60.0 + 50.0;
            graph.add_node(x, y, format!("({},{})", r, c));
        }
    }
    graph.is_directed = false;

    for r in 0..rows {
        for c in 0..cols {
            let id = r * cols + c;
            if c + 1 < cols { graph.add_edge(id, id + 1, 1.0); }
            if r + 1 < rows { graph.add_edge(id, id + cols, 1.0); }
        }
    }
    graph
}

// ────────────────────────────────────────────────────────────────────────────
// 간단한 XorShift64 의사난수 생성기
// ────────────────────────────────────────────────────────────────────────────

struct XorShift64 {
    state: u64,
}

impl XorShift64 {
    fn new(seed: u64) -> Self {
        Self { state: if seed == 0 { 12345 } else { seed } }
    }

    fn next(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }
}

// ────────────────────────────────────────────────────────────────────────────
// 단위 테스트
// ────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_graph() {
        let g = generate_random_graph(10, 0.3, false, 42);
        assert_eq!(g.nodes.len(), 10);
        assert!(!g.edges.is_empty());
    }

    #[test]
    fn test_generate_complete_graph() {
        let g = generate_complete_graph(5);
        assert_eq!(g.nodes.len(), 5);
        assert_eq!(g.edges.len(), 10); // 5C2 = 10
    }

    #[test]
    fn test_generate_grid_graph() {
        let g = generate_grid_graph(3, 3);
        assert_eq!(g.nodes.len(), 9);
        // 격자 간선 수: rows*(cols-1) + (rows-1)*cols = 3*2 + 2*3 = 12
        assert_eq!(g.edges.len(), 12);
    }

    #[test]
    fn test_bench_single_bfs() {
        let g = generate_random_graph(15, 0.4, false, 99);
        let runner = BenchmarkRunner::with_default();
        let mut config = BenchmarkConfig::default();
        config.iterations = 3;
        config.warmup = 1;
        let runner2 = BenchmarkRunner::new(config);
        let result = runner2.bench_single(&g, 0, AlgorithmKind::BFS);
        assert!(result.is_some());
        let r = result.unwrap();
        assert_eq!(r.iterations, 3);
        assert!(r.mean_ms >= 0.0);
    }

    #[test]
    fn test_algo_bench_result_stats() {
        let samples = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = compute_stats(AlgorithmKind::BFS, samples, 10, 15);
        assert!((result.mean_ms - 3.0).abs() < 0.001);
        assert!((result.median_ms - 3.0).abs() < 0.001);
        assert!((result.min_ms - 1.0).abs() < 0.001);
        assert!((result.max_ms - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_p95_p99() {
        let samples: Vec<f64> = (1..=100).map(|x| x as f64).collect();
        let result = compute_stats(AlgorithmKind::Dijkstra, samples, 5, 8);
        assert!(result.p95_ms() >= 95.0);
        assert!(result.p99_ms() >= 99.0);
    }

    #[test]
    fn test_xorshift_unique() {
        let mut rng = XorShift64::new(1);
        let values: Vec<u64> = (0..100).map(|_| rng.next()).collect();
        let unique: std::collections::HashSet<u64> = values.iter().cloned().collect();
        assert_eq!(unique.len(), 100); // 모두 달라야 함
    }
}
