# COME2105-2026-TEAM01

# Algo-Caster 

> 그래프 알고리즘 시각화 및 성능 분석 학습 플랫폼

**소프트웨어공학 팀 프로젝트 | COME2105-2026-TEAM01**  
- 엄예지(20247141)
- 김고운(20247137) 
- 권경빈(20247144)

---

## 01.프로젝트 소개

### 개발 목적
기존 텍스트 중심 학습 방식은 그래프 탐색 알고리즘의 흐름을 시각적으로 이해하기 어렵다는 한계가 있습니다.본 프로젝트는 알고리즘 동작 과정을 실시간으로 시각화하여 탐색 순서·방문 상태·경로 변화를 직관적으로 학습할 수 있도록 하고, 성능 데이터 분석을 통해 알고리즘의 효율성을 비교할 수 있는 환경을 제공합니다. 


### 프로젝트 개요
Algo-Caster는 그래프 탐색 알고리즘 학습을 지원하는 **데스크톱 기반 교육용 애플리케이션**입니다. BFS, DFS, Dijkstra, Bellman-Ford, Floyd-Warshall, A*, Prim, Kruskal에 더해 위상 정렬, 양방향 BFS를 포함한 **총 10종의 알고리즘**을 지원하며,Undo/Redo, 실행 이력 저장, 타이머 기반 애니메이션 속도 고정 기능을 제공합니다.
외부 API나 LLM 없이 로컬 환경에서 **100% 독립 실행**됩니다.


### 프로젝트 차별성 
본 프로젝트는 교육용 애플리케이션에서 흔히 사용되는 Python, Java 등의 언어 대신 **Rust**를 채택하여 메모리 안전성과 높은 실행 성능을 직접 경험하고, 모든 알고리즘 로직을 외부 라이브러리 없이 순수하게 구현함으로써 자료구조와 알고리즘에 대한 깊은 이해와 차별성을 추구하였습니다.


---

## 주요 기능

### 그래프 편집
- 마우스 클릭으로 노드 추가 / 드래그로 이동
- 간선 연결 및 가중치 설정
- 방향 그래프 / 무방향 그래프 전환
- **Undo / Redo** 
- 그래프 JSON 저장 및 불러오기

### 알고리즘 시각화 (총 10종)
|      분류    | 알고리즘    |
|-------------|-----------|
| 탐색         | BFS, DFS, 양방향 BFS |
| 최단 경로     | Dijkstra, Bellman-Ford, Floyd-Warshall, A* |
| 최소 신장 트리 | Prim, Kruskal |
| 고급.        | 위상 정렬 (Topological Sort) 

**지원 알고리즘 (10종)**
- BFS · DFS · 양방향 BFS · Dijkstra · Bellman-Ford · Floyd-Warshall · A* · Prim · Kruskal · 위상 정렬

- 단계별 애니메이션 재생 / 일시정지 / 한 스텝
- 실행 속도 조절 슬라이더
- 수도코드 하이라이팅 (현재 실행 라인 강조)
- 큐 / 스택 / 우선순위 큐 상태 실시간 표시

### 성능 분석
- 실행 시간(ms) · 방문 노드 수 · 간선 탐색 횟수 측정
- 알고리즘별 고유 색상 바 차트 비교
- 노드 수 증가에 따른 스케일 예측 라인 차트
- 실행 이력 저장 및 히스토리 탭 조회

### 학습 모드
- **챌린지 모드**: 다음 방문 노드를 직접 클릭하여 알고리즘 순서 맞히기
- 다크 / 라이트 테마 전환

---

## 팀원별 역할 분담 및 브랜치 전략

|  팀원 |       역할        |
|------|------------------|
| 엄예지 |     UI & 인터랙션  |
| 김고운 |      알고리즘 엔진  |
| 권경빈 | 성능 분석 & 스토리지 |

## 브랜치 전략

각자 담당 브랜치에서 작업 후 Pull Request를 통해 main에 머지합니다.

```
src/
├── main.rs                          # 진입점, 한글 폰트 설정 (엄예지)
│
├── ui/
│   ├── mod.rs                       # UI 모듈 선언 (공용)
│   └── app.rs                       # 메인 App 구조체, 전체 UI 레이아웃 (엄예지)
│                                    # - 그래프 렌더링, 마우스 이벤트 처리
│                                    # - 알고리즘 실행 제어, 애니메이션
│                                    # - Undo/Redo 버튼 및 단축키
│                                    # - 하단 히스토리 탭 UI
│
├── algorithm/
│   ├── mod.rs                       # 알고리즘 모듈 선언 (공용)
│   ├── state.rs                     # StateSnapshot, AlgorithmKind 타입 정의 (공용)
│   ├── bfs.rs                       # BFS 구현 (김고운)
│   ├── dfs.rs                       # DFS 구현 (김고운)
│   ├── dijkstra.rs                  # Dijkstra 구현 (김고운)
│   ├── bellman_ford.rs              # Bellman-Ford 구현 (김고운)
│   ├── floyd_warshall.rs            # Floyd-Warshall 구현 (김고운)
│   ├── astar.rs                     # A* 구현 (김고운)
│   ├── prim.rs                      # Prim MST 구현 (김고운)
│   ├── kruskal.rs                   # Kruskal MST 구현 (김고운)
│   ├── topological_sort.rs          # 위상 정렬 구현 (김고운)
│   └── bidirectional_bfs.rs         # 양방향 BFS 구현 (김고운)
│
├── graph/
│   ├── mod.rs                       # Node, Edge, Graph 자료구조 정의 (공용)
│   └── history.rs                   # Undo/Redo Command Pattern (김고운)
│
├── performance/
│   ├── mod.rs                       # 성능 모듈 선언 (공용)
│   ├── tracker.rs                   # 실행 시간 측정 (공용)
│   ├── stats.rs                     # 알고리즘 비교 통계 집계 (공용)
│   ├── predictor.rs                 # 시간복잡도 기반 성능 예측 (권경빈)
│   └── benchmark.rs                 # 반복 실행 벤치마크 (권경빈)
│
├── storage/
│   ├── mod.rs                       # 스토리지 모듈 선언 (공용)
│   ├── serializer.rs                # 그래프 JSON 저장 (권경빈)
│   ├── deserializer.rs              # 그래프 JSON 불러오기 (권경빈)
│   ├── validator.rs                 # 저장 데이터 무결성 검증 (권경빈)
│   └── session_history.rs           # 실행 이력 JSONL 영속화 (권경빈)
│
└── visualization/
├── mod.rs                       # 시각화 모듈 선언 (공용)
└── dashboard.rs                 # 성능 분석 대시보드 차트 (권경빈)
```
---

## 실행 방법

### 요구 환경
- OS: Windows 10/11
- Rust Toolchain (stable) 설치 필요


### 빌드 및 실행
```bash
git clone https://github.com/HBNU-SWUNIV/COME2105-2026-TEAM01.git
cd COME2105-2026-TEAM01
cargo run
```

### 릴리즈 빌드
```bash
cargo build --release
```
실행 파일: `target/release/algo-caster.exe`

---

## 기술 스택

| 항목  | 내용  |
|------|------|
| 언어  | Rust 2021 Edition |
| UI 프레임워크 | egui 0.27 / eframe 0.27 |
| 데이터 시각화 | egui_plot 0.27 |
| 직렬화 | serde / serde_json |
| 파일 다이얼로그 | rfd 0.17 |
| 외부 AI/API | 사용 없음 (100% 로컬 동작) |

---

## 제약 조건

- 외부 AI API(ChatGPT 등) 일절 미사용
- 핵심 알고리즘 로직 직접 구현
- 오프라인 환경에서 100% 동작
- DB 서버 / 클라우드 연결 없이 독립 실행


---


## 02. 프로젝트 기능 / 비기능 요구사항
**링크 첨부**
> https://docs.google.com/document/d/1aeCucp2RI11GnjC4XQnUziRYXX3LSS7MZAOCEkgxRho/edit?tab=t.xqmgtbpojhp8 

--- 


## 03.프로젝트 테스트 케이스 (TC)
**링크 첨부**
> https://docs.google.com/spreadsheets/d/11FKeUdD_nu-CDD9Jcqdgdqp1TQTUlKztEiy8LbUBOHY/edit?usp=sharing


