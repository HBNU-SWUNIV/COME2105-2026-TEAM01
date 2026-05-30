//! 그래프 공용 데이터 구조 모듈
//! 팀원 A·B·C 전원이 공유하는 Node, Edge, Graph 타입을 정의합니다.

use serde::{Deserialize, Serialize};

// ─── 노드 ────────────────────────────────────────────────────────────────────

/// 그래프의 노드 하나를 표현합니다.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    /// 노드 고유 ID (0-indexed)
    pub id: usize,
    /// 화면 좌표 (egui 렌더링용)
    pub x: f32,
    pub y: f32,
    /// 노드 라벨 (예: "A", "1")
    pub label: String,
    /// 현재 시각화 상태 (팀원 A 렌더링용)
    pub visual_state: NodeVisualState,
}

/// 알고리즘 단계에 따른 노드 색상 상태
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodeVisualState {
    /// 미방문 (흰색)
    Unvisited,
    /// 현재 방문 중 (노란색)
    Current,
    /// 방문 완료 (초록색)
    Visited,
}

impl Default for NodeVisualState {
    fn default() -> Self {
        NodeVisualState::Unvisited
    }
}

// ─── 간선 ────────────────────────────────────────────────────────────────────

/// 두 노드를 잇는 간선 하나를 표현합니다.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    /// 출발 노드 ID
    pub from: usize,
    /// 도착 노드 ID
    pub to: usize,
    /// 가중치 (기본값 1)
    pub weight: f64,
}

// ─── 그래프 ──────────────────────────────────────────────────────────────────

/// 전체 그래프 구조를 담는 메인 자료구조.
/// 팀원 A가 생성·편집하고, 팀원 B가 탐색하며, 팀원 C가 저장합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    /// 방향 그래프 여부
    pub is_directed: bool,
}

impl Graph {
    pub fn new() -> Self {
        Self::default()
    }

    /// 노드 추가 후 부여된 ID 반환
    pub fn add_node(&mut self, x: f32, y: f32, label: String) -> usize {
        let id = self.nodes.len();
        self.nodes.push(Node {
            id,
            x,
            y,
            label,
            visual_state: NodeVisualState::Unvisited,
        });
        id
    }

    /// 간선 추가
    pub fn add_edge(&mut self, from: usize, to: usize, weight: f64) {
        self.edges.push(Edge { from, to, weight });
    }

    /// 모든 노드의 시각화 상태를 초기화
    pub fn reset_visual_states(&mut self) {
        for node in &mut self.nodes {
            node.visual_state = NodeVisualState::Unvisited;
        }
    }

    /// 인접 노드 목록 반환 (ID, 가중치)
    pub fn neighbors(&self, node_id: usize) -> Vec<(usize, f64)> {
        let mut result = vec![];
        for edge in &self.edges {
            if edge.from == node_id {
                result.push((edge.to, edge.weight));
            }
            if !self.is_directed && edge.to == node_id {
                result.push((edge.from, edge.weight));
            }
        }
        result
    }
}

// ─── 히스토리 모듈 ────────────────────────────────────────────────────────
pub mod history;
