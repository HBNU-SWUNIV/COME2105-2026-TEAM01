//! 세션 히스토리 저장/불러오기 모듈
//!
//! 알고리즘 실행 이력을 파일에 영속화하여 앱 재시작 후에도 이전 결과를 불러올 수 있습니다.
//!
//! ## 파일 형식
//! JSON Lines(JSONL) 형식을 사용합니다. 각 줄이 독립적인 JSON 레코드이므로
//! 파일 손상 시에도 부분 복구가 가능합니다.
//!
//! ## 저장 경로
//! 기본값: `./algo_sketch_history.jsonl`
//! 사용자 지정 가능.

use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use crate::algorithm::AlgorithmKind;
use crate::graph::Graph;

// ────────────────────────────────────────────────────────────────────────────
// 세션 레코드 타입
// ────────────────────────────────────────────────────────────────────────────

/// 단일 알고리즘 실행 기록
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionRecord {
    /// 레코드 고유 ID (단조 증가)
    pub id: u64,
    /// 실행 시각 (RFC 3339 문자열)
    pub timestamp: String,
    /// 실행한 알고리즘
    pub algorithm: AlgorithmKind,
    /// 시작 노드 ID
    pub start_node: usize,
    /// 실행 당시 그래프 노드 수
    pub node_count: usize,
    /// 실행 당시 그래프 간선 수
    pub edge_count: usize,
    /// 실행 시간 (ms)
    pub elapsed_ms: f64,
    /// 방문 노드 순서
    pub visit_order: Vec<usize>,
    /// 간선 탐색 횟수
    pub edge_traversal_count: usize,
    /// 사용자 메모 (선택)
    pub note: Option<String>,
    /// 결과 스냅샷 수 (단계 수)
    pub snapshot_count: usize,
}

impl SessionRecord {
    /// 새 레코드 생성 헬퍼
    pub fn new(
        id: u64,
        algorithm: AlgorithmKind,
        start_node: usize,
        graph: &Graph,
        elapsed_ms: f64,
        visit_order: Vec<usize>,
        edge_traversal_count: usize,
        snapshot_count: usize,
    ) -> Self {
        Self {
            id,
            timestamp: current_timestamp(),
            algorithm,
            start_node,
            node_count: graph.nodes.len(),
            edge_count: graph.edges.len(),
            elapsed_ms,
            visit_order,
            edge_traversal_count,
            note: None,
            snapshot_count,
        }
    }

    /// 사람이 읽기 좋은 한 줄 요약
    pub fn summary_line(&self) -> String {
        format!(
            "[{}] #{} {} | 노드:{} 간선:{} | 시작:{} | {:.3}ms | 방문:{}개 | 스냅샷:{}",
            self.timestamp,
            self.id,
            self.algorithm,
            self.node_count,
            self.edge_count,
            self.start_node,
            self.elapsed_ms,
            self.visit_order.len(),
            self.snapshot_count,
        )
    }
}

// ────────────────────────────────────────────────────────────────────────────
// 세션 히스토리 관리자
// ────────────────────────────────────────────────────────────────────────────

/// 세션 히스토리 관리자
///
/// JSONL 파일에 레코드를 추가(append)하며, 불러올 때는 전체를 파싱합니다.
#[derive(Debug)]
pub struct SessionHistory {
    /// 히스토리 파일 경로
    path: PathBuf,
    /// 인메모리 레코드 캐시 (최근 N개)
    cache: Vec<SessionRecord>,
    /// 캐시 최대 크기
    max_cache: usize,
    /// 다음 레코드 ID
    next_id: u64,
    /// 총 저장된 레코드 수 (파일 기준)
    total_saved: usize,
}

impl SessionHistory {
    /// 기본 경로(`./algo_sketch_history.jsonl`)로 세션 히스토리 생성
    pub fn new() -> Self {
        Self::with_path("algo_sketch_history.jsonl")
    }

    /// 지정된 경로로 세션 히스토리 생성 및 기존 파일 로드
    pub fn with_path(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        let mut history = Self {
            path: path.clone(),
            cache: Vec::new(),
            max_cache: 100,
            next_id: 1,
            total_saved: 0,
        };
        // 기존 파일이 있으면 로드
        if path.exists() {
            let _ = history.load_from_file();
        }
        history
    }

    /// 파일에서 레코드 로드 (최근 max_cache개만 캐시에 유지)
    pub fn load_from_file(&mut self) -> Result<usize, HistoryError> {
        let file = File::open(&self.path).map_err(HistoryError::Io)?;
        let reader = BufReader::new(file);
        let mut records: Vec<SessionRecord> = Vec::new();

        for line in reader.lines() {
            let line = line.map_err(HistoryError::Io)?;
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            match serde_json::from_str::<SessionRecord>(trimmed) {
                Ok(record) => records.push(record),
                Err(e) => {
                    // 손상된 줄은 건너뜀
                    eprintln!("[HistoryWarning] 레코드 파싱 실패 (건너뜀): {}", e);
                }
            }
        }

        let total = records.len();
        self.total_saved = total;

        // next_id 업데이트
        if let Some(last) = records.last() {
            self.next_id = last.id + 1;
        }

        // 최근 max_cache개만 캐시에 유지
        let start = total.saturating_sub(self.max_cache);
        self.cache = records[start..].to_vec();

        Ok(total)
    }

    /// 새 레코드를 파일에 추가하고 캐시에도 삽입
    pub fn append(&mut self, mut record: SessionRecord) -> Result<u64, HistoryError> {
        record.id = self.next_id;
        self.next_id += 1;

        let json = serde_json::to_string(&record).map_err(HistoryError::Json)?;

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .map_err(HistoryError::Io)?;

        writeln!(file, "{}", json).map_err(HistoryError::Io)?;
        self.total_saved += 1;

        // 캐시 업데이트
        let id = record.id;
        self.cache.push(record);
        if self.cache.len() > self.max_cache {
            self.cache.remove(0);
        }

        Ok(id)
    }

    /// 캐시된 레코드 목록 반환 (최신 순)
    pub fn recent_records(&self) -> Vec<&SessionRecord> {
        self.cache.iter().rev().collect()
    }

    /// 특정 알고리즘의 레코드만 필터링
    pub fn records_for_algorithm(&self, kind: AlgorithmKind) -> Vec<&SessionRecord> {
        self.cache.iter().filter(|r| r.algorithm == kind).collect()
    }

    /// 특정 알고리즘의 평균 실행 시간 계산
    pub fn avg_elapsed_ms(&self, kind: AlgorithmKind) -> Option<f64> {
        let filtered: Vec<f64> = self.cache.iter()
            .filter(|r| r.algorithm == kind)
            .map(|r| r.elapsed_ms)
            .collect();
        if filtered.is_empty() {
            return None;
        }
        Some(filtered.iter().sum::<f64>() / filtered.len() as f64)
    }

    /// 전체 저장된 레코드 수
    pub fn total_count(&self) -> usize {
        self.total_saved
    }

    /// 캐시된 레코드 수
    pub fn cached_count(&self) -> usize {
        self.cache.len()
    }

    /// 가장 자주 실행된 알고리즘
    pub fn most_used_algorithm(&self) -> Option<AlgorithmKind> {
        use std::collections::HashMap;
        let mut counts: HashMap<String, (AlgorithmKind, usize)> = HashMap::new();
        for r in &self.cache {
            let key = format!("{}", r.algorithm);
            counts.entry(key).and_modify(|(_, c)| *c += 1).or_insert((r.algorithm, 1));
        }
        counts.values().max_by_key(|(_, c)| *c).map(|(k, _)| *k)
    }

    /// 히스토리 파일 경로 반환
    pub fn file_path(&self) -> &Path {
        &self.path
    }

    /// 캐시 초기화 (파일은 유지)
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// 히스토리 파일 삭제 (주의: 복구 불가)
    pub fn delete_file(&self) -> Result<(), HistoryError> {
        if self.path.exists() {
            std::fs::remove_file(&self.path).map_err(HistoryError::Io)?;
        }
        Ok(())
    }

    /// 히스토리 통계 요약
    pub fn stats_summary(&self) -> HistoryStats {
        let total = self.cache.len();
        if total == 0 {
            return HistoryStats::default();
        }

        let avg_ms = self.cache.iter().map(|r| r.elapsed_ms).sum::<f64>() / total as f64;
        let max_ms = self.cache.iter().map(|r| r.elapsed_ms).fold(0.0_f64, f64::max);
        let min_ms = self.cache.iter().map(|r| r.elapsed_ms).fold(f64::INFINITY, f64::min);
        let total_edge_traversals: usize = self.cache.iter().map(|r| r.edge_traversal_count).sum();

        HistoryStats {
            record_count: total,
            avg_elapsed_ms: avg_ms,
            max_elapsed_ms: max_ms,
            min_elapsed_ms: min_ms,
            total_edge_traversals,
            most_used: self.most_used_algorithm(),
        }
    }
}

impl Default for SessionHistory {
    fn default() -> Self {
        Self::new()
    }
}

// ────────────────────────────────────────────────────────────────────────────
// 통계 구조체
// ────────────────────────────────────────────────────────────────────────────

/// 세션 히스토리 집계 통계
#[derive(Debug, Clone, Default)]
pub struct HistoryStats {
    pub record_count: usize,
    pub avg_elapsed_ms: f64,
    pub max_elapsed_ms: f64,
    pub min_elapsed_ms: f64,
    pub total_edge_traversals: usize,
    pub most_used: Option<AlgorithmKind>,
}

impl HistoryStats {
    pub fn summary(&self) -> String {
        format!(
            "기록 수={} | 평균={:.3}ms | 최소={:.3}ms | 최대={:.3}ms | 총 탐색={} | 최다 사용={}",
            self.record_count,
            self.avg_elapsed_ms,
            self.min_elapsed_ms,
            self.max_elapsed_ms,
            self.total_edge_traversals,
            self.most_used.map(|k| format!("{}", k)).unwrap_or_else(|| "없음".into()),
        )
    }
}

// ────────────────────────────────────────────────────────────────────────────
// 에러 타입
// ────────────────────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum HistoryError {
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl std::fmt::Display for HistoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HistoryError::Io(e) => write!(f, "IO 에러: {}", e),
            HistoryError::Json(e) => write!(f, "JSON 에러: {}", e),
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// 유틸리티
// ────────────────────────────────────────────────────────────────────────────

/// 현재 시각을 ISO 8601 문자열로 반환 (외부 크레이트 없이 구현)
fn current_timestamp() -> String {
    // std::time::SystemTime을 RFC 3339에 가깝게 포맷
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let s = secs % 60;
    let m = (secs / 60) % 60;
    let h = (secs / 3600) % 24;
    let days = secs / 86400;
    // 간략한 날짜 계산 (UTC 기준, 윤년 미지원)
    let year = 1970 + days / 365;
    let day_of_year = days % 365;
    let month = day_of_year / 30 + 1;
    let day = day_of_year % 30 + 1;

    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        year, month.min(12), day.min(31), h, m, s
    )
}

/// JSON 파일에서 레코드 배치 임포트 (다른 인스턴스에서 내보낸 파일 병합용)
pub fn import_records(path: &Path) -> Result<Vec<SessionRecord>, HistoryError> {
    let file = File::open(path).map_err(HistoryError::Io)?;
    let reader = BufReader::new(file);
    let mut records = Vec::new();

    for line in reader.lines() {
        let line = line.map_err(HistoryError::Io)?;
        let trimmed = line.trim();
        if trimmed.is_empty() { continue; }
        if let Ok(record) = serde_json::from_str::<SessionRecord>(trimmed) {
            records.push(record);
        }
    }

    Ok(records)
}

// ────────────────────────────────────────────────────────────────────────────
// 단위 테스트
// ────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Graph;
    use std::fs;

    fn temp_path() -> PathBuf {
        PathBuf::from(format!("/tmp/test_history_{}.jsonl", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap().subsec_nanos()))
    }

    fn make_record(id: u64, kind: AlgorithmKind) -> SessionRecord {
        let g = Graph::new();
        SessionRecord::new(id, kind, 0, &g, 1.5, vec![0, 1, 2], 3, 5)
    }

    #[test]
    fn test_append_and_load() {
        let path = temp_path();
        let mut hist = SessionHistory::with_path(&path);

        hist.append(make_record(0, AlgorithmKind::BFS)).unwrap();
        hist.append(make_record(0, AlgorithmKind::DFS)).unwrap();
        assert_eq!(hist.cached_count(), 2);
        assert_eq!(hist.total_count(), 2);

        // 새 인스턴스에서 로드
        let mut hist2 = SessionHistory::with_path(&path);
        assert_eq!(hist2.total_count(), 2);
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn test_filter_by_algorithm() {
        let path = temp_path();
        let mut hist = SessionHistory::with_path(&path);

        hist.append(make_record(0, AlgorithmKind::BFS)).unwrap();
        hist.append(make_record(0, AlgorithmKind::BFS)).unwrap();
        hist.append(make_record(0, AlgorithmKind::Dijkstra)).unwrap();

        assert_eq!(hist.records_for_algorithm(AlgorithmKind::BFS).len(), 2);
        assert_eq!(hist.records_for_algorithm(AlgorithmKind::Dijkstra).len(), 1);

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn test_avg_elapsed() {
        let path = temp_path();
        let mut hist = SessionHistory::with_path(&path);
        let g = Graph::new();

        for ms in [1.0, 3.0, 5.0] {
            let r = SessionRecord::new(0, AlgorithmKind::BFS, 0, &g, ms, vec![], 0, 0);
            hist.append(r).unwrap();
        }

        let avg = hist.avg_elapsed_ms(AlgorithmKind::BFS).unwrap();
        assert!((avg - 3.0).abs() < 0.001);
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn test_most_used_algorithm() {
        let path = temp_path();
        let mut hist = SessionHistory::with_path(&path);

        for _ in 0..3 { hist.append(make_record(0, AlgorithmKind::BFS)).unwrap(); }
        for _ in 0..1 { hist.append(make_record(0, AlgorithmKind::DFS)).unwrap(); }

        assert_eq!(hist.most_used_algorithm(), Some(AlgorithmKind::BFS));
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn test_stats_summary() {
        let path = temp_path();
        let mut hist = SessionHistory::with_path(&path);
        hist.append(make_record(0, AlgorithmKind::BFS)).unwrap();
        let stats = hist.stats_summary();
        assert_eq!(stats.record_count, 1);
        assert!(stats.avg_elapsed_ms > 0.0);
        let _ = fs::remove_file(&path);
    }
}
