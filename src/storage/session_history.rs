//! 알고리즘 실행 이력 JSONL 영속화 (팀원 C: 권경빈 담당)

use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use crate::algorithm::AlgorithmKind;
use crate::graph::Graph;

// ── 레코드 타입 ───────────────────────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionRecord {
    pub id:                    u64,
    pub timestamp:             String,
    pub algorithm:             AlgorithmKind,
    pub start_node:            usize,
    pub node_count:            usize,
    pub edge_count:            usize,
    pub elapsed_ms:            f64,
    pub visit_order:           Vec<usize>,
    pub edge_traversal_count:  usize,
    pub note:                  Option<String>,
    pub snapshot_count:        usize,
}

impl SessionRecord {
    pub fn new(id: u64, algorithm: AlgorithmKind, start_node: usize,
               graph: &Graph, elapsed_ms: f64, visit_order: Vec<usize>,
               edge_traversal_count: usize, snapshot_count: usize) -> Self {
        // TODO: 현재 시각 timestamp 생성, 필드 채워서 반환
        todo!("SessionRecord::new 구현 예정 — 권경빈")
    }

    pub fn summary_line(&self) -> String {
        // TODO: 한 줄 요약 문자열 반환
        todo!("summary_line 구현 예정 — 권경빈")
    }
}

// ── 히스토리 관리자 ───────────────────────────────────────────────────────────
#[derive(Debug)]
pub struct SessionHistory {
    path:         PathBuf,
    cache:        Vec<SessionRecord>,
    max_cache:    usize,
    next_id:      u64,
    total_saved:  usize,
}

impl SessionHistory {
    pub fn new() -> Self { Self::with_path("algo_sketch_history.jsonl") }

    pub fn with_path(path: impl Into<PathBuf>) -> Self {
        // TODO: 파일 있으면 load_from_file 호출
        todo!("SessionHistory::with_path 구현 예정 — 권경빈")
    }

    pub fn load_from_file(&mut self) -> Result<usize, HistoryError> {
        // TODO: JSONL 파일 한 줄씩 읽어 파싱, 손상 줄은 건너뜀
        todo!("load_from_file 구현 예정 — 권경빈")
    }

    pub fn append(&mut self, mut record: SessionRecord) -> Result<u64, HistoryError> {
        // TODO: record.id 할당, JSON 직렬화 후 파일에 append, 캐시 갱신
        todo!("append 구현 예정 — 권경빈")
    }

    pub fn recent_records(&self) -> Vec<&SessionRecord> {
        self.cache.iter().rev().collect()
    }

    pub fn records_for_algorithm(&self, kind: AlgorithmKind) -> Vec<&SessionRecord> {
        self.cache.iter().filter(|r| r.algorithm == kind).collect()
    }

    pub fn avg_elapsed_ms(&self, kind: AlgorithmKind) -> Option<f64> {
        // TODO: 해당 알고리즘 레코드들의 elapsed_ms 평균
        todo!("avg_elapsed_ms 구현 예정 — 권경빈")
    }

    pub fn most_used_algorithm(&self) -> Option<AlgorithmKind> {
        // TODO: 캐시에서 가장 많이 등장하는 AlgorithmKind 반환
        todo!("most_used_algorithm 구현 예정 — 권경빈")
    }

    pub fn stats_summary(&self) -> HistoryStats {
        // TODO: record_count, avg/min/max ms, total_edge_traversals, most_used 계산
        todo!("stats_summary 구현 예정 — 권경빈")
    }

    pub fn total_count(&self)  -> usize { self.total_saved }
    pub fn cached_count(&self) -> usize { self.cache.len() }
    pub fn file_path(&self) -> &Path { &self.path }
    pub fn clear_cache(&mut self) { self.cache.clear(); }
}

impl Default for SessionHistory {
    fn default() -> Self { Self::new() }
}

// ── 통계·에러 타입 ────────────────────────────────────────────────────────────
#[derive(Debug, Clone, Default)]
pub struct HistoryStats {
    pub record_count:          usize,
    pub avg_elapsed_ms:        f64,
    pub max_elapsed_ms:        f64,
    pub min_elapsed_ms:        f64,
    pub total_edge_traversals: usize,
    pub most_used:             Option<AlgorithmKind>,
}

#[derive(Debug)]
pub enum HistoryError {
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl std::fmt::Display for HistoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HistoryError::Io(e)   => write!(f, "IO 에러: {}", e),
            HistoryError::Json(e) => write!(f, "JSON 에러: {}", e),
        }
    }
}
