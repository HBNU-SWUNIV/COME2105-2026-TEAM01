//! JSON 파일 → 그래프 복원 (팀원 C: 권경빈 담당)

use std::path::Path;
use crate::graph::Graph;
use super::{GraphSaveData, StorageError};
use super::validator::validate_graph_data;

/// JSON 파일을 읽어 Graph 구조체로 복원합니다.
pub fn load_graph(path: &Path) -> Result<(Graph, GraphSaveData), StorageError> {
    // 1. 파일 읽기
    let content = std::fs::read_to_string(path)
        .map_err(StorageError::IoError)?;

    // 2. JSON 역직렬화
    let data: GraphSaveData = serde_json::from_str(&content)
        .map_err(StorageError::SerdeError)?;

    // 3. 무결성 검사
    validate_graph_data(&data)
        .map_err(StorageError::ValidationFailed)?;

    // 4. GraphSaveData → Graph 변환
    let mut graph = Graph::new();
    graph.nodes = data.nodes.clone();
    graph.edges = data.edges.clone();
    graph.is_directed = data.is_directed;

    Ok((graph, data))
}
