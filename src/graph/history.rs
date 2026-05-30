//! 그래프 편집 히스토리 — Undo/Redo 기능 제공
//!
//! Command Pattern을 활용하여 그래프에 가해진 모든 편집 작업을 기록하고,
//! Undo(되돌리기) / Redo(다시 실행)를 지원합니다.
//!
//! ## 지원하는 편집 커맨드
//! - 노드 추가/삭제
//! - 간선 추가/삭제
//! - 노드 이동(드래그)
//! - 그래프 전체 초기화
//! - 노드 라벨 변경
//! - 간선 가중치 변경
//! - 방향성 토글
//!
//! ## 사용 예시
//! ```rust
//! let mut history = GraphHistory::new(50); // 최대 50단계 저장
//! history.record(GraphCommand::AddNode { id: 0, x: 100.0, y: 200.0, label: "A".into() });
//! history.undo(&mut graph); // 노드 추가 취소
//! history.redo(&mut graph); // 다시 노드 추가
//! ```

use crate::graph::{Graph, Node, Edge, NodeVisualState};

// ────────────────────────────────────────────────────────────────────────────
// 커맨드 열거형 — 모든 편집 작업의 기록 단위
// ────────────────────────────────────────────────────────────────────────────

/// 그래프에 적용 가능한 편집 커맨드
#[derive(Debug, Clone)]
pub enum GraphCommand {
    /// 노드 추가
    AddNode {
        id: usize,
        x: f32,
        y: f32,
        label: String,
    },
    /// 노드 삭제 (연결된 간선 포함)
    RemoveNode {
        id: usize,
        x: f32,
        y: f32,
        label: String,
        /// 삭제 시 함께 제거된 간선 목록 (복원용)
        removed_edges: Vec<Edge>,
    },
    /// 간선 추가
    AddEdge {
        from: usize,
        to: usize,
        weight: f64,
    },
    /// 간선 삭제
    RemoveEdge {
        from: usize,
        to: usize,
        weight: f64,
        /// 해당 간선이 edges 벡터에서 차지하던 인덱스 (복원 위치 유지용)
        edge_index: usize,
    },
    /// 노드 이동
    MoveNode {
        id: usize,
        from_x: f32,
        from_y: f32,
        to_x: f32,
        to_y: f32,
    },
    /// 노드 라벨 변경
    RenameNode {
        id: usize,
        old_label: String,
        new_label: String,
    },
    /// 간선 가중치 변경
    UpdateEdgeWeight {
        from: usize,
        to: usize,
        old_weight: f64,
        new_weight: f64,
    },
    /// 방향성 토글
    ToggleDirected {
        was_directed: bool,
    },
    /// 그래프 전체 교체 (초기화/불러오기 등)
    ReplaceGraph {
        old_graph: Graph,
        new_graph: Graph,
    },
    /// 다중 커맨드 묶음 (배치 작업)
    Batch(Vec<GraphCommand>),
}

impl GraphCommand {
    /// 커맨드의 사람이 읽기 좋은 설명 반환
    pub fn description(&self) -> String {
        match self {
            GraphCommand::AddNode { label, .. } => format!("노드 '{}' 추가", label),
            GraphCommand::RemoveNode { label, .. } => format!("노드 '{}' 삭제", label),
            GraphCommand::AddEdge { from, to, weight } => {
                format!("간선 {} → {} (가중치 {:.2}) 추가", from, to, weight)
            }
            GraphCommand::RemoveEdge { from, to, weight, .. } => {
                format!("간선 {} → {} (가중치 {:.2}) 삭제", from, to, weight)
            }
            GraphCommand::MoveNode { id, .. } => format!("노드 {} 이동", id),
            GraphCommand::RenameNode { old_label, new_label, .. } => {
                format!("노드 이름 변경: '{}' → '{}'", old_label, new_label)
            }
            GraphCommand::UpdateEdgeWeight { from, to, old_weight, new_weight } => {
                format!(
                    "간선 {}→{} 가중치 변경: {:.2} → {:.2}",
                    from, to, old_weight, new_weight
                )
            }
            GraphCommand::ToggleDirected { was_directed } => {
                if *was_directed {
                    "방향 그래프 → 무방향 그래프".to_string()
                } else {
                    "무방향 그래프 → 방향 그래프".to_string()
                }
            }
            GraphCommand::ReplaceGraph { .. } => "그래프 교체".to_string(),
            GraphCommand::Batch(cmds) => format!("{}개 작업 묶음", cmds.len()),
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// 히스토리 관리자
// ────────────────────────────────────────────────────────────────────────────

/// 그래프 편집 히스토리 관리자
///
/// `max_history` 개를 초과하면 가장 오래된 커맨드를 삭제합니다.
#[derive(Debug, Default)]
pub struct GraphHistory {
    /// 실행된 커맨드 스택 (undo 시 pop)
    undo_stack: Vec<GraphCommand>,
    /// Undo된 커맨드 스택 (redo 시 pop)
    redo_stack: Vec<GraphCommand>,
    /// 최대 유지 히스토리 수
    max_history: usize,
    /// 편집 횟수 카운터
    pub edit_count: usize,
}

impl GraphHistory {
    /// 새 히스토리 관리자 생성
    pub fn new(max_history: usize) -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_history: max_history.max(1),
            edit_count: 0,
        }
    }

    /// 커맨드를 기록합니다. Redo 스택은 초기화됩니다.
    pub fn record(&mut self, cmd: GraphCommand) {
        self.redo_stack.clear();
        self.undo_stack.push(cmd);
        self.edit_count += 1;

        // 최대 히스토리 초과 시 가장 오래된 것 제거
        if self.undo_stack.len() > self.max_history {
            self.undo_stack.remove(0);
        }
    }

    /// Undo 가능 여부
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Redo 가능 여부
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// 마지막 커맨드를 취소합니다.
    pub fn undo(&mut self, graph: &mut Graph) -> Option<String> {
        let cmd = self.undo_stack.pop()?;
        let desc = cmd.description();
        apply_inverse(graph, &cmd);
        self.redo_stack.push(cmd);
        Some(desc)
    }

    /// 마지막으로 취소된 커맨드를 다시 실행합니다.
    pub fn redo(&mut self, graph: &mut Graph) -> Option<String> {
        let cmd = self.redo_stack.pop()?;
        let desc = cmd.description();
        apply_command(graph, &cmd);
        self.undo_stack.push(cmd);
        self.edit_count += 1;
        Some(desc)
    }

    /// Undo 스택의 커맨드 설명 목록 (최신 → 오래된 순)
    pub fn undo_history(&self) -> Vec<String> {
        self.undo_stack.iter().rev().map(|c| c.description()).collect()
    }

    /// Redo 스택의 커맨드 설명 목록
    pub fn redo_history(&self) -> Vec<String> {
        self.redo_stack.iter().rev().map(|c| c.description()).collect()
    }

    /// 히스토리 전체 초기화 (저장 후 등)
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    /// 현재 저장되지 않은 변경 사항이 있는지 확인
    pub fn has_unsaved_changes(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Undo 스택 깊이
    pub fn undo_depth(&self) -> usize {
        self.undo_stack.len()
    }

    /// Redo 스택 깊이
    pub fn redo_depth(&self) -> usize {
        self.redo_stack.len()
    }
}

// ────────────────────────────────────────────────────────────────────────────
// 커맨드 적용 (정방향)
// ────────────────────────────────────────────────────────────────────────────

/// 커맨드를 그래프에 정방향으로 적용
pub fn apply_command(graph: &mut Graph, cmd: &GraphCommand) {
    match cmd {
        GraphCommand::AddNode { id, x, y, label } => {
            // ID가 이미 존재하면 덮어쓰기 방지
            if *id >= graph.nodes.len() {
                graph.nodes.push(Node {
                    id: *id,
                    x: *x,
                    y: *y,
                    label: label.clone(),
                    visual_state: NodeVisualState::Unvisited,
                });
            }
        }

        GraphCommand::RemoveNode { id, removed_edges, .. } => {
            graph.nodes.retain(|n| n.id != *id);
            graph.edges.retain(|e| e.from != *id && e.to != *id);
            // 삭제된 간선도 제거 (이미 retain으로 처리됨)
            let _ = removed_edges; // 역방향(undo)에서만 사용
        }

        GraphCommand::AddEdge { from, to, weight } => {
            graph.edges.push(Edge {
                from: *from,
                to: *to,
                weight: *weight,
            });
        }

        GraphCommand::RemoveEdge { from, to, weight, .. } => {
            // 첫 번째로 매칭되는 간선만 제거
            if let Some(pos) = graph.edges.iter().position(|e| {
                e.from == *from && e.to == *to && (e.weight - weight).abs() < f64::EPSILON
            }) {
                graph.edges.remove(pos);
            }
        }

        GraphCommand::MoveNode { id, to_x, to_y, .. } => {
            if let Some(node) = graph.nodes.iter_mut().find(|n| n.id == *id) {
                node.x = *to_x;
                node.y = *to_y;
            }
        }

        GraphCommand::RenameNode { id, new_label, .. } => {
            if let Some(node) = graph.nodes.iter_mut().find(|n| n.id == *id) {
                node.label = new_label.clone();
            }
        }

        GraphCommand::UpdateEdgeWeight { from, to, new_weight, .. } => {
            for edge in graph.edges.iter_mut() {
                if edge.from == *from && edge.to == *to {
                    edge.weight = *new_weight;
                    break;
                }
            }
        }

        GraphCommand::ToggleDirected { .. } => {
            graph.is_directed = !graph.is_directed;
        }

        GraphCommand::ReplaceGraph { new_graph, .. } => {
            *graph = new_graph.clone();
        }

        GraphCommand::Batch(cmds) => {
            for c in cmds {
                apply_command(graph, c);
            }
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// 역방향 적용 (Undo)
// ────────────────────────────────────────────────────────────────────────────

/// 커맨드를 역방향(undo)으로 그래프에 적용
fn apply_inverse(graph: &mut Graph, cmd: &GraphCommand) {
    match cmd {
        // AddNode의 역은 RemoveNode
        GraphCommand::AddNode { id, .. } => {
            graph.nodes.retain(|n| n.id != *id);
            graph.edges.retain(|e| e.from != *id && e.to != *id);
        }

        // RemoveNode의 역은 AddNode + 간선 복원
        GraphCommand::RemoveNode { id, x, y, label, removed_edges } => {
            graph.nodes.push(Node {
                id: *id,
                x: *x,
                y: *y,
                label: label.clone(),
                visual_state: NodeVisualState::Unvisited,
            });
            // 노드 ID 순서 정렬
            graph.nodes.sort_by_key(|n| n.id);
            // 삭제됐던 간선들 복원
            for edge in removed_edges {
                graph.edges.push(edge.clone());
            }
        }

        // AddEdge의 역은 RemoveEdge
        GraphCommand::AddEdge { from, to, weight } => {
            if let Some(pos) = graph.edges.iter().position(|e| {
                e.from == *from && e.to == *to && (e.weight - weight).abs() < f64::EPSILON
            }) {
                graph.edges.remove(pos);
            }
        }

        // RemoveEdge의 역은 AddEdge (원래 위치에 삽입)
        GraphCommand::RemoveEdge { from, to, weight, edge_index } => {
            let edge = Edge { from: *from, to: *to, weight: *weight };
            let insert_pos = (*edge_index).min(graph.edges.len());
            graph.edges.insert(insert_pos, edge);
        }

        // MoveNode의 역은 원래 위치로 복원
        GraphCommand::MoveNode { id, from_x, from_y, .. } => {
            if let Some(node) = graph.nodes.iter_mut().find(|n| n.id == *id) {
                node.x = *from_x;
                node.y = *from_y;
            }
        }

        // RenameNode의 역은 이전 라벨로 복원
        GraphCommand::RenameNode { id, old_label, .. } => {
            if let Some(node) = graph.nodes.iter_mut().find(|n| n.id == *id) {
                node.label = old_label.clone();
            }
        }

        // UpdateEdgeWeight의 역은 이전 가중치로 복원
        GraphCommand::UpdateEdgeWeight { from, to, old_weight, .. } => {
            for edge in graph.edges.iter_mut() {
                if edge.from == *from && edge.to == *to {
                    edge.weight = *old_weight;
                    break;
                }
            }
        }

        GraphCommand::ToggleDirected { .. } => {
            graph.is_directed = !graph.is_directed;
        }

        GraphCommand::ReplaceGraph { old_graph, .. } => {
            *graph = old_graph.clone();
        }

        GraphCommand::Batch(cmds) => {
            // 배치는 역순으로 undo
            for c in cmds.iter().rev() {
                apply_inverse(graph, c);
            }
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// 커맨드 빌더 헬퍼
// ────────────────────────────────────────────────────────────────────────────

/// 그래프에서 노드 삭제 커맨드를 생성합니다 (연결된 간선 정보 자동 수집)
pub fn build_remove_node_command(graph: &Graph, node_id: usize) -> Option<GraphCommand> {
    let node = graph.nodes.iter().find(|n| n.id == node_id)?;
    let removed_edges: Vec<Edge> = graph.edges.iter()
        .filter(|e| e.from == node_id || e.to == node_id)
        .cloned()
        .collect();

    Some(GraphCommand::RemoveNode {
        id: node.id,
        x: node.x,
        y: node.y,
        label: node.label.clone(),
        removed_edges,
    })
}

/// 간선 삭제 커맨드를 생성합니다 (간선 인덱스 자동 탐색)
pub fn build_remove_edge_command(graph: &Graph, from: usize, to: usize) -> Option<GraphCommand> {
    let (idx, edge) = graph.edges.iter().enumerate()
        .find(|(_, e)| e.from == from && e.to == to)?;

    Some(GraphCommand::RemoveEdge {
        from: edge.from,
        to: edge.to,
        weight: edge.weight,
        edge_index: idx,
    })
}

// ────────────────────────────────────────────────────────────────────────────
// 단위 테스트
// ────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Graph;

    #[test]
    fn test_add_undo_redo_node() {
        let mut graph = Graph::new();
        let mut history = GraphHistory::new(20);

        let cmd = GraphCommand::AddNode { id: 0, x: 50.0, y: 50.0, label: "A".into() };
        apply_command(&mut graph, &cmd);
        history.record(cmd);
        assert_eq!(graph.nodes.len(), 1);

        // Undo
        history.undo(&mut graph);
        assert_eq!(graph.nodes.len(), 0);

        // Redo
        history.redo(&mut graph);
        assert_eq!(graph.nodes.len(), 1);
        assert_eq!(graph.nodes[0].label, "A");
    }

    #[test]
    fn test_add_remove_edge() {
        let mut graph = Graph::new();
        graph.add_node(0.0, 0.0, "A".into());
        graph.add_node(100.0, 0.0, "B".into());

        let mut history = GraphHistory::new(20);

        let cmd = GraphCommand::AddEdge { from: 0, to: 1, weight: 3.0 };
        apply_command(&mut graph, &cmd);
        history.record(cmd);
        assert_eq!(graph.edges.len(), 1);

        // Undo: 간선 제거
        history.undo(&mut graph);
        assert_eq!(graph.edges.len(), 0);

        // Redo: 간선 복원
        history.redo(&mut graph);
        assert_eq!(graph.edges.len(), 1);
        assert!((graph.edges[0].weight - 3.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_move_node_undo() {
        let mut graph = Graph::new();
        graph.add_node(0.0, 0.0, "A".into());
        let mut history = GraphHistory::new(20);

        let cmd = GraphCommand::MoveNode { id: 0, from_x: 0.0, from_y: 0.0, to_x: 100.0, to_y: 200.0 };
        apply_command(&mut graph, &cmd);
        history.record(cmd);
        assert!((graph.nodes[0].x - 100.0).abs() < f32::EPSILON);

        history.undo(&mut graph);
        assert!((graph.nodes[0].x - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_batch_undo() {
        let mut graph = Graph::new();
        let mut history = GraphHistory::new(20);

        let batch = GraphCommand::Batch(vec![
            GraphCommand::AddNode { id: 0, x: 0.0, y: 0.0, label: "A".into() },
            GraphCommand::AddNode { id: 1, x: 100.0, y: 0.0, label: "B".into() },
            GraphCommand::AddEdge { from: 0, to: 1, weight: 1.0 },
        ]);
        apply_command(&mut graph, &batch);
        history.record(batch);
        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.edges.len(), 1);

        history.undo(&mut graph);
        assert_eq!(graph.nodes.len(), 0);
        assert_eq!(graph.edges.len(), 0);
    }

    #[test]
    fn test_max_history_limit() {
        let mut graph = Graph::new();
        let mut history = GraphHistory::new(3);

        for i in 0..5usize {
            let cmd = GraphCommand::AddNode { id: i, x: i as f32, y: 0.0, label: format!("{}", i) };
            apply_command(&mut graph, &cmd);
            history.record(cmd);
        }

        assert_eq!(history.undo_depth(), 3); // 최대 3개만 유지
    }

    #[test]
    fn test_redo_cleared_after_new_record() {
        let mut graph = Graph::new();
        let mut history = GraphHistory::new(20);

        let cmd1 = GraphCommand::AddNode { id: 0, x: 0.0, y: 0.0, label: "A".into() };
        apply_command(&mut graph, &cmd1);
        history.record(cmd1);

        history.undo(&mut graph);
        assert!(history.can_redo());

        // 새 커맨드 기록 → redo 스택 초기화
        let cmd2 = GraphCommand::AddNode { id: 1, x: 100.0, y: 0.0, label: "B".into() };
        apply_command(&mut graph, &cmd2);
        history.record(cmd2);

        assert!(!history.can_redo());
    }
}
