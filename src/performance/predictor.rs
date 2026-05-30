//! 시간 복잡도 기반 성능 예측 엔진 (팀원 C: 권경빈 담당)

/// 지원 알고리즘별 시간 복잡도 분류
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlgorithmComplexity {
    /// O(V + E)  — BFS, DFS
    BfsOrDfs,
    /// O(E log V) — Dijkstra, A*
    Dijkstra,
    /// O(V · E)  — Bellman-Ford
    BellmanFord,
    /// O(V³)     — Floyd-Warshall
    FloydWarshall,
    /// O(E log E) — Kruskal
    Kruskal,
    /// O(E log V) — Prim
    Prim,
}

/// 단일 예측 포인트 (차트 x축: 노드 수, y축: 예상 실행 시간 ms)
#[derive(Debug, Clone)]
pub struct PredictionPoint {
    pub node_count: usize,
    pub edge_count: usize,
    pub predicted_ms: f64,
}

/// 실측값을 기반으로 대규모 환경 성능을 예측하는 엔진
///
/// # 예측 공식
/// - BFS/DFS:       predicted = measured × (V_t + E_t) / (V_m + E_m)
/// - Dijkstra/A*:   predicted = measured × (E_t × log2(V_t)) / (E_m × log2(V_m))
/// - Bellman-Ford:  predicted = measured × (V_t × E_t) / (V_m × E_m)
/// - Floyd-Warshall:predicted = measured × V_t³ / V_m³
#[derive(Debug, Default)]
pub struct PredictionEngine {
    pub measured_ms: f64,
    pub measured_nodes: usize,
    pub measured_edges: usize,
}

impl PredictionEngine {
    pub fn new(measured_ms: f64, measured_nodes: usize, measured_edges: usize) -> Self {
        Self { measured_ms, measured_nodes, measured_edges }
    }

    /// 목표 규모에서의 예상 실행 시간 계산 (ms)
    pub fn predict(
        &self,
        target_nodes: usize,
        target_edges: usize,
        complexity: AlgorithmComplexity,
    ) -> f64 {
        if self.measured_nodes == 0 || self.measured_ms == 0.0 {
            return 0.0;
        }

        match complexity {
            AlgorithmComplexity::BfsOrDfs => {
                let measured_work = (self.measured_nodes + self.measured_edges) as f64;
                let target_work   = (target_nodes + target_edges) as f64;
                if measured_work == 0.0 { return 0.0; }
                self.measured_ms * target_work / measured_work
            }
            AlgorithmComplexity::Dijkstra => {
                let log_m = (self.measured_nodes as f64).log2().max(1.0);
                let log_t = (target_nodes as f64).log2().max(1.0);
                let measured_work = self.measured_edges as f64 * log_m;
                let target_work   = target_edges as f64 * log_t;
                if measured_work == 0.0 { return 0.0; }
                self.measured_ms * target_work / measured_work
            }
            AlgorithmComplexity::BellmanFord => {
                let measured_work = (self.measured_nodes * self.measured_edges) as f64;
                let target_work   = (target_nodes * target_edges) as f64;
                if measured_work == 0.0 { return 0.0; }
                self.measured_ms * target_work / measured_work
            }
            AlgorithmComplexity::Kruskal | AlgorithmComplexity::Prim => {
                // Kruskal: O(E log E)  Prim: O(E log V) — 둘 다 Dijkstra 공식으로 근사
                let log_m = (self.measured_nodes as f64).log2().max(1.0);
                let log_t = (target_nodes as f64).log2().max(1.0);
                let measured_work = self.measured_edges as f64 * log_m;
                let target_work   = target_edges as f64 * log_t;
                if measured_work == 0.0 { return 0.0; }
                self.measured_ms * target_work / measured_work
            }
            AlgorithmComplexity::FloydWarshall => {
                let mv = self.measured_nodes as f64;
                let tv = target_nodes as f64;
                if mv == 0.0 { return 0.0; }
                self.measured_ms * (tv * tv * tv) / (mv * mv * mv)
            }
        }
    }

    /// 노드 수 시나리오(100·500·1000·5000·10000)에 대한 예측값 일괄 생성
    pub fn generate_scale_predictions(&self, complexity: AlgorithmComplexity) -> Vec<PredictionPoint> {
        let scenarios: [usize; 5] = [100, 500, 1000, 5000, 10000];
        scenarios.iter().map(|&v| {
            let e = v * 2;
            PredictionPoint {
                node_count: v,
                edge_count: e,
                predicted_ms: self.predict(v, e, complexity),
            }
        }).collect()
    }
}