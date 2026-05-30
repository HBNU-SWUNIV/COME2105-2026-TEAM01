//! 그래프 JSON 직렬화 (팀원 C: 권경빈 담당)

use std::path::Path;
use crate::storage::{GraphSaveData, SaveMetadata, StorageError};
use crate::graph::Graph;

/// 그래프를 JSON 파일로 저장
pub fn save_graph(graph: &Graph, path: &Path, name: &str) -> Result<(), StorageError> {
    // TODO: GraphSaveData 구성 후 serde_json으로 직렬화하여 파일 저장
    todo!("save_graph 구현 예정 — 권경빈")
}
