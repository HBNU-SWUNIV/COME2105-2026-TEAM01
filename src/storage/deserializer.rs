//! 그래프 JSON 역직렬화 (팀원 C: 권경빈 담당)

use std::path::Path;
use crate::storage::{GraphSaveData, StorageError};
use crate::graph::Graph;

/// JSON 파일에서 그래프 불러오기
pub fn load_graph(path: &Path) -> Result<(Graph, GraphSaveData), StorageError> {
    // TODO: 파일 읽기 → serde_json 파싱 → 무결성 검증 → Graph 반환
    todo!("load_graph 구현 예정 — 권경빈")
}
