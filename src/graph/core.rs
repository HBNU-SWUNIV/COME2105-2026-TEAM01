use serde::{Serialize, Deserialize};

// =================================================================
// [공통 데이터] 모든 팀원이 공유할 그래프의 기본 단위
// =================================================================
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Node {
    pub id: usize,
    pub x: f32,       // 화면상 X 좌표 (팀원 A가 드래그로 제어할 예정)
    pub y: f32,       // 화면상 Y 좌표 (팀원 A가 드래그로 제어할 예정)
    pub label: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Edge {
    pub from: usize,  // 시작 노드 ID
    pub to: usize,    // 끝 노드 ID
    pub weight: i32,  // 선의 가중치 숫자
}

// =================================================================
// [팀원 B 전용 데이터 인터페이스] 알고리즘 상태 트래커
// =================================================================
#[derive(Default, Clone, Debug)]
pub struct AlgorithmTracker {
    pub current_line: usize,       // 현재 왼쪽 창에 하이라이팅할 코드 줄 번호
    pub visited_nodes: Vec<usize>, // 방문이 완료된 노드 ID 리스트
    pub open_set: Vec<usize>,      // 실시간 하단 Queue / Stack 창에 시각화할 데이터 배열
    pub is_finished: bool,         // 알고리즘 탐색 완료 여부
}