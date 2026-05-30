//! JSON 데이터 무결성 검증 (팀원 C: 권경빈 담당)

use super::GraphSaveData;

const SUPPORTED_VERSION: &str = "1.0.0";

/// 데이터 검증 실패 유형
#[derive(Debug, Clone)]
pub enum ValidationError {
    InvalidEdgeReference { from: usize, to: usize },
    NegativeWeight { from: usize, to: usize, weight: f64 },
    VersionMismatch { found: String, expected: String },
    CountMismatch { field: String, meta: usize, actual: usize },
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::InvalidEdgeReference { from, to } =>
                write!(f, "간선 ({}->{}) 이 존재하지 않는 노드를 참조합니다", from, to),
            ValidationError::NegativeWeight { from, to, weight } =>
                write!(f, "간선 ({}->{}) 의 가중치 {} 가 음수입니다", from, to, weight),
            ValidationError::VersionMismatch { found, expected } =>
                write!(f, "버전 불일치: 파일={}, 지원={}", found, expected),
            ValidationError::CountMismatch { field, meta, actual } =>
                write!(f, "{} 수 불일치: 메타데이터={}, 실제={}", field, meta, actual),
        }
    }
}

/// GraphSaveData 전체 무결성 검사
pub fn validate_graph_data(data: &GraphSaveData) -> Result<(), Vec<ValidationError>> {
    let mut errors: Vec<ValidationError> = vec![];
    let node_count = data.nodes.len();

    // 1. 버전 호환성 검사
    if data.version != SUPPORTED_VERSION {
        errors.push(ValidationError::VersionMismatch {
            found: data.version.clone(),
            expected: SUPPORTED_VERSION.to_string(),
        });
    }

    // 2. 메타데이터 카운트 일치 여부
    if data.metadata.node_count != node_count {
        errors.push(ValidationError::CountMismatch {
            field: "노드".to_string(),
            meta: data.metadata.node_count,
            actual: node_count,
        });
    }
    if data.metadata.edge_count != data.edges.len() {
        errors.push(ValidationError::CountMismatch {
            field: "간선".to_string(),
            meta: data.metadata.edge_count,
            actual: data.edges.len(),
        });
    }

    // 3. 간선 참조 유효성
    for edge in &data.edges {
        if edge.from >= node_count || edge.to >= node_count {
            errors.push(ValidationError::InvalidEdgeReference {
                from: edge.from,
                to: edge.to,
            });
        }
    }

    // 4. 음수 가중치 검사
    for edge in &data.edges {
        if edge.weight < 0.0 {
            errors.push(ValidationError::NegativeWeight {
                from: edge.from,
                to: edge.to,
                weight: edge.weight,
            });
        }
    }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}
