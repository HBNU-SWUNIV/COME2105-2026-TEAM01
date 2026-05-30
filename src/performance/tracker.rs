//! 알고리즘 실행 시간 및 방문 노드 수 측정기 (팀원 C: 권경빈 담당)

use std::time::Instant;
use crate::algorithm::AlgorithmResult;

/// 알고리즘 1회 실행에 대한 성능 측정 결과
#[derive(Debug, Clone, Default)]
pub struct PerformanceResult {
    /// 실행 시간 (밀리초, 소수점 3자리)
    pub elapsed_ms: f64,
    /// 방문한 노드 수
    pub visited_count: usize,
    /// 탐색한 간선 수
    pub edge_traversal_count: usize,
}

/// 알고리즘 실행을 감싸서 성능을 측정하는 트래커
#[derive(Debug, Default)]
pub struct PerformanceTracker {
    start_time: Option<Instant>,
}

impl PerformanceTracker {
    pub fn new() -> Self {
        Self::default()
    }

    /// 측정 시작 — run_algorithm() 직전에 호출
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    /// 측정 종료 및 결과 반환 — run_algorithm() 직후에 호출
    pub fn stop(&mut self, result: &AlgorithmResult) -> PerformanceResult {
        let elapsed_ms = match self.start_time.take() {
            Some(t) => {
                let ms = t.elapsed().as_secs_f64() * 1000.0;
                // 소수점 3자리 반올림
                (ms * 1000.0).round() / 1000.0
            }
            None => 0.0,
        };

        PerformanceResult {
            elapsed_ms,
            visited_count: result.visit_order.len(),
            edge_traversal_count: result.edge_traversal_count,
        }
    }
}
