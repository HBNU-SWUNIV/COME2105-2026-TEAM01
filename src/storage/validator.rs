//! 저장 데이터 무결성 검증 (팀원 C: 권경빈 담당)

use crate::storage::GraphSaveData;

#[derive(Debug, Clone)]
pub enum ValidationError {
    EmptyName,
    InvalidNodeId { id: usize },
    InvalidEdgeEndpoint { from: usize, to: usize },
    DuplicateNodeId { id: usize },
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: 각 variant 에러 메시지 반환
        todo!("ValidationError::fmt 구현 예정 — 권경빈")
    }
}

/// 저장 데이터의 무결성을 검증하고 에러 목록 반환
pub fn validate_graph_data(data: &GraphSaveData) -> Vec<ValidationError> {
    // TODO: name 비어있는지, 노드 ID 중복·범위 초과, 간선 참조 유효성 검사
    todo!("validate_graph_data 구현 예정 — 권경빈")
}
