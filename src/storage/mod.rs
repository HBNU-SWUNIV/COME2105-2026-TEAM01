pub mod deserializer;
pub mod serializer;
pub mod validator;

pub use deserializer::load_graph;
pub use serializer::save_graph;
// validate_graph_data 포워딩 제거
pub use validator::ValidationError;

use serde::{Deserialize, Serialize};
use crate::graph::{Node, Edge};
use crate::algorithm::AlgorithmKind;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphSaveData {
    pub version: String,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub is_directed: bool,
    pub last_algorithm: Option<AlgorithmKind>,
    pub metadata: SaveMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveMetadata {
    pub saved_at: String,
    pub name: String,
    pub node_count: usize,
    pub edge_count: usize,
}

#[derive(Debug)]
pub enum StorageError {
    IoError(std::io::Error),
    SerdeError(serde_json::Error),
    ValidationFailed(Vec<ValidationError>),
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageError::IoError(e) => write!(f, "IO 에러: {}", e),
            StorageError::SerdeError(e) => write!(f, "JSON 파싱 에러: {}", e),
            StorageError::ValidationFailed(errs) => write!(f, "무결성 검사 실패 ({}건)", errs.len()),
        }
    }
}

pub mod session_history;
