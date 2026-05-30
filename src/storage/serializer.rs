//! 그래프 → JSON 파일 저장 (팀원 C: 권경빈 담당)

use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::graph::Graph;
use super::{GraphSaveData, SaveMetadata, StorageError};

/// 현재 시각을 ISO 8601 형식 문자열로 반환 (chrono 없이 std::time 사용)
fn current_timestamp() -> String {
    // SystemTime → 초 단위 UNIX timestamp → 간단한 포맷팅
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    // 초를 날짜/시간으로 변환 (간단 구현, UTC 기준)
    let s = secs % 60;
    let m = (secs / 60) % 60;
    let h = (secs / 3600) % 24;
    let days = secs / 86400;
    // 1970-01-01 기준 일수로 연/월/일 계산
    let year  = 1970 + days / 365;
    let month = (days % 365) / 30 + 1;
    let day   = (days % 365) % 30 + 1;
    format!("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z", year, month, day, h, m, s)
}

/// 그래프를 JSON 파일로 저장합니다.
pub fn save_graph(graph: &Graph, path: &Path, name: &str) -> Result<(), StorageError> {
    let metadata = SaveMetadata {
        saved_at: current_timestamp(),
        name: name.to_string(),
        node_count: graph.nodes.len(),
        edge_count: graph.edges.len(),
    };

    let save_data = GraphSaveData {
        version: "1.0.0".to_string(),
        nodes: graph.nodes.clone(),
        edges: graph.edges.clone(),
        is_directed: graph.is_directed,
        last_algorithm: None,
        metadata,
    };

    let json = serde_json::to_string_pretty(&save_data)
        .map_err(StorageError::SerdeError)?;

    std::fs::write(path, json)
        .map_err(StorageError::IoError)?;

    Ok(())
}
