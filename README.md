# COME2105-2026-TEAM01

> **그래프 알고리즘 시각화 학습 플랫폼**



## 기술 스택
- **Language**: Rust (Core Logic & Engine)
- **UI Framework**: egui (GUI & Visualization Layer)

## 핵심 기능
- **DFS (깊이 우선 탐색) 시각화**
  - 스택(Stack)과 재귀 호출을 이용한 그래프 탐색 과정 시각화
- **BFS (너비 우선 탐색) 시각화**
  - 큐(Queue)를 이용한 최단 경로 및 인접 정점 방문 과정 시각화
- **동적 그래프 생성**
  - 시각화 테스트를 위한 정점(Vertex)과 간선(Edge)의 추가 및 편집 기능

## 프로젝트 폴더 구조
```text
├── src
│   ├── main.rs             # 프로그램 실행 시작점
│   ├── graph/              # 그래프 데이터 구조 및 탐색 엔진 (Rust)
│   └── ui/                 # egui 기반 GUI 창 및 애니메이션 레이아웃 
├── Cargo.toml              # Rust 패키지 및 Qt 라이브러리 의존성 관리
└── README.md               # 본 프로젝트 설명서
