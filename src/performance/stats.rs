//! 알고리즘 비교 통계 집계 모듈 (팀원 C: 권경빈 담당)

use crate::algorithm::AlgorithmKind;
use crate::performance::tracker::PerformanceResult;
use crate::performance::predictor::PredictionPoint;

/// 알고리즘 1종에 대한 통합 통계
#[derive(Debug, Clone)]
pub struct AlgorithmStats {
    pub kind: AlgorithmKind,
    pub performance: PerformanceResult,
    pub predictions: Vec<PredictionPoint>,
}

/// 여러 알고리즘 실행 결과를 하나로 묶은 비교 보고서
#[derive(Debug, Default, Clone)]
pub struct ComparisonReport {
    pub entries: Vec<AlgorithmStats>,
}

impl ComparisonReport {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, stats: AlgorithmStats) {
        // 같은 알고리즘이 이미 있으면 교체, 없으면 추가
        if let Some(existing) = self.entries.iter_mut().find(|e| e.kind == stats.kind) {
            *existing = stats;
        } else {
            self.entries.push(stats);
        }
    }

    /// 가장 빠른 알고리즘 반환 (elapsed_ms 기준)
    pub fn fastest(&self) -> Option<&AlgorithmStats> {
        self.entries.iter().min_by(|a, b| {
            a.performance.elapsed_ms.total_cmp(&b.performance.elapsed_ms)
        })
    }

    /// 방문 노드 수가 가장 적은 알고리즘 반환
    pub fn most_efficient(&self) -> Option<&AlgorithmStats> {
        self.entries.iter().min_by_key(|e| e.performance.visited_count)
    }
}
