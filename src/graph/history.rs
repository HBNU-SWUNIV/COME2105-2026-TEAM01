//! 그래프 편집 히스토리 — Undo/Redo (팀원 B: 김고운 담당)

use crate::graph::{Graph, Node, Edge, NodeVisualState};

// ── 커맨드 열거형 ─────────────────────────────────────────────────────────────
/// 그래프에 적용 가능한 모든 편집 커맨드
#[derive(Debug, Clone)]
pub enum GraphCommand {
    AddNode    { id: usize, x: f32, y: f32, label: String },
    RemoveNode { id: usize, x: f32, y: f32, label: String, removed_edges: Vec<Edge> },
    AddEdge    { from: usize, to: usize, weight: f64 },
    RemoveEdge { from: usize, to: usize, weight: f64, edge_index: usize },
    MoveNode   { id: usize, from_x: f32, from_y: f32, to_x: f32, to_y: f32 },
    RenameNode { id: usize, old_label: String, new_label: String },
    UpdateEdgeWeight { from: usize, to: usize, old_weight: f64, new_weight: f64 },
    ToggleDirected   { was_directed: bool },
    ReplaceGraph     { old_graph: Graph, new_graph: Graph },
    Batch(Vec<GraphCommand>),
}

impl GraphCommand {
    pub fn description(&self) -> String {
        // TODO: 각 커맨드별 사람이 읽기 좋은 설명 반환
        todo!("GraphCommand::description 구현 예정 — 김고운")
    }
}

// ── 히스토리 관리자 ───────────────────────────────────────────────────────────
#[derive(Debug, Default)]
pub struct GraphHistory {
    undo_stack:  Vec<GraphCommand>,
    redo_stack:  Vec<GraphCommand>,
    max_history: usize,
    pub edit_count: usize,
}

impl GraphHistory {
    pub fn new(max_history: usize) -> Self {
        Self { undo_stack: Vec::new(), redo_stack: Vec::new(),
               max_history: max_history.max(1), edit_count: 0 }
    }

    pub fn record(&mut self, cmd: GraphCommand) {
        // TODO: redo 스택 초기화, undo 스택에 push, max_history 초과 시 oldest 제거
        todo!("record 구현 예정 — 김고운")
    }

    pub fn can_undo(&self) -> bool { !self.undo_stack.is_empty() }
    pub fn can_redo(&self) -> bool { !self.redo_stack.is_empty() }

    pub fn undo(&mut self, graph: &mut Graph) -> Option<String> {
        // TODO: undo_stack에서 pop → apply_inverse → redo_stack에 push
        todo!("undo 구현 예정 — 김고운")
    }

    pub fn redo(&mut self, graph: &mut Graph) -> Option<String> {
        // TODO: redo_stack에서 pop → apply_command → undo_stack에 push
        todo!("redo 구현 예정 — 김고운")
    }

    pub fn undo_history(&self) -> Vec<String> {
        self.undo_stack.iter().rev().map(|c| c.description()).collect()
    }

    pub fn redo_history(&self) -> Vec<String> {
        self.redo_stack.iter().rev().map(|c| c.description()).collect()
    }

    pub fn clear(&mut self) { self.undo_stack.clear(); self.redo_stack.clear(); }
    pub fn has_unsaved_changes(&self) -> bool { !self.undo_stack.is_empty() }
    pub fn undo_depth(&self) -> usize { self.undo_stack.len() }
    pub fn redo_depth(&self) -> usize { self.redo_stack.len() }
}

// ── 커맨드 적용 ───────────────────────────────────────────────────────────────
/// 커맨드를 그래프에 정방향으로 적용
pub fn apply_command(graph: &mut Graph, cmd: &GraphCommand) {
    // TODO: 각 GraphCommand variant에 맞는 그래프 변경 로직 구현
    todo!("apply_command 구현 예정 — 김고운")
}

/// 노드 삭제 커맨드 생성 헬퍼 (연결 간선 자동 수집)
pub fn build_remove_node_command(graph: &Graph, node_id: usize) -> Option<GraphCommand> {
    // TODO: 해당 노드의 연결 간선을 모아 RemoveNode 커맨드 생성
    todo!("build_remove_node_command 구현 예정 — 김고운")
}

/// 간선 삭제 커맨드 생성 헬퍼
pub fn build_remove_edge_command(graph: &Graph, from: usize, to: usize) -> Option<GraphCommand> {
    // TODO: 해당 간선의 인덱스를 찾아 RemoveEdge 커맨드 생성
    todo!("build_remove_edge_command 구현 예정 — 김고운")
}
