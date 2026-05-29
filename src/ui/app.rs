// ============================================================
// src/ui/app.rs
// 담당: 엄예지 – UI & 인터랙션 그래픽스 엔지니어
// 역할: 전체 레이아웃, 그래프 렌더링, 마우스 인터랙션,
//       코드 패널, 실행 상태 패널, 로그, 성능 차트
// ============================================================

use eframe::egui::{
    self, Align, Align2, Color32, FontId, Frame, Id, Layout, Margin,
    Pos2, Rect, RichText, Rounding, ScrollArea, Sense, Stroke, Vec2,
};

// ──────────────────────────────────────────────────────────────
// 타입 임포트 (각 팀원 모듈에서 가져옴)
// ──────────────────────────────────────────────────────────────
use crate::graph::GraphData;           // 권경빈/김고운 공용 그래프 구조체
use crate::algorithm::state::{
    AlgoSnapshot, AlgorithmType,
};                                     // 김고운 담당 스냅샷/알고타입
use crate::performance::stats::PerformanceStats; // 권경빈 담당 성능 통계

// ──────────────────────────────────────────────────────────────
// 색상 팔레트 (라이트 모드)
// ──────────────────────────────────────────────────────────────
const C_BG_MAIN: Color32        = Color32::from_rgb(245, 246, 250);
const C_BG_SIDEBAR: Color32     = Color32::from_rgb(237, 238, 244);
const C_BG_PANEL: Color32       = Color32::from_rgb(255, 255, 255);
const C_BG_CODE: Color32        = Color32::from_rgb(40,  42,  54);
const C_BG_LOG: Color32         = Color32::from_rgb(250, 250, 253);
const C_BG_CANVAS: Color32      = Color32::from_rgb(252, 252, 255);

const C_ACCENT_BLUE: Color32    = Color32::from_rgb(66,  133, 244);
const C_ACCENT_GREEN: Color32   = Color32::from_rgb(52,  168,  83);
const C_ACCENT_RED: Color32     = Color32::from_rgb(234,  67,  53);
const C_ACCENT_ORANGE: Color32  = Color32::from_rgb(251, 140,   0);
const C_ACCENT_GRAY: Color32    = Color32::from_rgb(158, 158, 165);

const C_NODE_DEFAULT: Color32   = Color32::WHITE;
const C_NODE_CURRENT: Color32   = Color32::from_rgb(255, 220,   0); // 노란색
const C_NODE_VISITED: Color32   = Color32::from_rgb( 72, 199, 116); // 초록색
const C_NODE_BORDER: Color32    = Color32::from_rgb( 80,  80,  90);
const C_EDGE_DEFAULT: Color32   = Color32::from_rgb(150, 150, 160);
const C_EDGE_ACTIVE: Color32    = Color32::from_rgb(255, 140,   0);

const C_CODE_TEXT: Color32      = Color32::from_rgb(200, 200, 210);
const C_CODE_NUM: Color32       = Color32::from_rgb(100, 100, 115);
const C_CODE_HL_CURR: Color32   = Color32::from_rgb(255, 220,   0);
const C_CODE_HL_NEXT: Color32   = Color32::from_rgb(180, 210, 255);
const C_CODE_HL_CURR_TEXT: Color32 = Color32::BLACK;
const C_CODE_HL_NEXT_TEXT: Color32 = Color32::from_rgb( 30,  30,  60);

const C_TEXT_MAIN: Color32      = Color32::from_rgb( 30,  30,  40);
const C_TEXT_SUB: Color32       = Color32::from_rgb(110, 110, 125);
const C_DIVIDER: Color32        = Color32::from_rgb(210, 212, 220);
const C_LOG_STEP: Color32       = Color32::from_rgb( 66, 133, 244);
const C_LOG_ACTIVE: Color32     = Color32::from_rgb( 52, 168,  83);

// ──────────────────────────────────────────────────────────────
// 레이아웃 상수
// ──────────────────────────────────────────────────────────────
const SIDEBAR_W: f32    = 200.0;
const CODE_W: f32       = 310.0;
const BOTTOM_H: f32     = 220.0;
const NODE_R: f32       = 24.0;
const BTN_H: f32        = 30.0;
const BTN_W: f32        = 176.0;
const SECTION_GAP: f32  = 10.0;

// ──────────────────────────────────────────────────────────────
// 편집 모드
// ──────────────────────────────────────────────────────────────
#[derive(Debug, Clone, PartialEq)]
enum EditMode {
    Select,
    AddNode,
    AddEdge,
    Delete,
    EditWeight,
}

// ──────────────────────────────────────────────────────────────
// 재생 상태
// ──────────────────────────────────────────────────────────────
#[derive(Debug, Clone, PartialEq)]
pub enum PlayState {
    Idle,
    Playing,
    Paused,
    Finished,
}

// ──────────────────────────────────────────────────────────────
// 알고리즘별 Java 코드 라인
// ──────────────────────────────────────────────────────────────
fn lines_bfs() -> Vec<&'static str> {
    vec![
        "void bfs(Graph graph, Node start) {",
        "    Queue<Node> queue = new LinkedList<>();",
        "    Set<Node> visited = new HashSet<>();",
        "    ",
        "    queue.add(start);",
        "    visited.add(start);",
        "    ",
        "    while (!queue.isEmpty()) {",
        "        Node current = queue.poll();",
        "        System.out.println(\"방문: \" + current.getId());",
        "        ",
        "        for (Edge edge : current.getEdges()) {",
        "            Node next = edge.getTo();",
        "            if (!visited.contains(next)) {",
        "                visited.add(next);",
        "                queue.add(next);",
        "            }",
        "        }",
        "    }",
        "}",
    ]
}

fn lines_dfs() -> Vec<&'static str> {
    vec![
        "void dfs(Graph graph, Node node,",
        "         Set<Node> visited) {",
        "    visited.add(node);",
        "    System.out.println(\"방문: \" + node.getId());",
        "    ",
        "    for (Edge edge : node.getEdges()) {",
        "        Node next = edge.getTo();",
        "        if (!visited.contains(next)) {",
        "            dfs(graph, next, visited);",
        "        }",
        "    }",
        "}",
    ]
}

fn lines_dijkstra() -> Vec<&'static str> {
    vec![
        "void dijkstra(Graph graph, Node src) {",
        "    Map<Node,Integer> dist = new HashMap<>();",
        "    PriorityQueue<int[]> pq = new PriorityQueue<>(",
        "        Comparator.comparingInt(a -> a[0]));",
        "    dist.put(src, 0);",
        "    pq.add(new int[]{0, src.getId()});",
        "    ",
        "    while (!pq.isEmpty()) {",
        "        int[] cur = pq.poll();",
        "        int d = cur[0];",
        "        Node u = graph.getNode(cur[1]);",
        "        if (d > dist.getOrDefault(u, INF)) continue;",
        "        for (Edge e : u.getEdges()) {",
        "            int nd = dist.get(u) + e.getWeight();",
        "            if (nd < dist.getOrDefault(e.getTo(),INF)) {",
        "                dist.put(e.getTo(), nd);",
        "                pq.add(new int[]{nd, e.getTo().getId()});",
        "            }",
        "        }",
        "    }",
        "}",
    ]
}

// ──────────────────────────────────────────────────────────────
// 내부 노드/간선 (UI 전용 — GraphData 로드 전 편집용)
// ──────────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
struct UiNode {
    id: usize,
    x: f32,
    y: f32,
    label: String,
}

#[derive(Debug, Clone)]
struct UiEdge {
    from: usize,
    to: usize,
    weight: f32,
}

// ──────────────────────────────────────────────────────────────
// 메인 앱 구조체
// ──────────────────────────────────────────────────────────────
pub struct AlgoSketchApp {
    // 그래프 편집
    nodes: Vec<UiNode>,
    edges: Vec<UiEdge>,
    next_id: usize,

    // 편집 모드
    edit_mode: EditMode,
    dragging: Option<usize>,
    edge_from: Option<usize>,

    // 가중치 편집 팝업
    show_weight_popup: bool,
    pending_edge: Option<(usize, usize)>,
    weight_buf: String,

    // 가중치 직접 편집 (클릭한 간선)
    editing_edge_idx: Option<usize>,
    editing_weight_buf: String,

    // 알고리즘 제어
    selected_algo: AlgorithmType,
    start_node_buf: String,
    play_state: PlayState,
    speed: f32,
    last_step_time: f64,

    // 스냅샷 (김고운 알고리즘 엔진에서 주입)
    snapshots: Vec<AlgoSnapshot>,
    current_step: usize,

    // 성능 통계 (권경빈에서 주입)
    perf_bfs: Option<PerformanceStats>,
    perf_dfs: Option<PerformanceStats>,
    perf_dijkstra: Option<PerformanceStats>,

    // 로그
    log_lines: Vec<(String, bool)>, // (텍스트, 활성여부)

    // 캔버스 rect 저장 (좌표 변환용)
    canvas_rect: Rect,

    // 파일 작업 피드백
    file_msg: Option<(String, f64)>, // (메시지, 표시 시작 시각)
}

impl Default for AlgoSketchApp {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            next_id: 0,
            edit_mode: EditMode::Select,
            dragging: None,
            edge_from: None,
            show_weight_popup: false,
            pending_edge: None,
            weight_buf: "1".into(),
            editing_edge_idx: None,
            editing_weight_buf: String::new(),
            selected_algo: AlgorithmType::BFS,
            start_node_buf: "0".into(),
            play_state: PlayState::Idle,
            speed: 1.0,
            last_step_time: 0.0,
            snapshots: Vec::new(),
            current_step: 0,
            perf_bfs: None,
            perf_dfs: None,
            perf_dijkstra: None,
            log_lines: Vec::new(),
            canvas_rect: Rect::NOTHING,
            file_msg: None,
        }
    }
}

// ──────────────────────────────────────────────────────────────
// 팀원 인터페이스 (public)
// ──────────────────────────────────────────────────────────────
impl AlgoSketchApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    /// 김고운 알고리즘 엔진이 결과 주입 시 호출
    pub fn inject_snapshots(&mut self, snaps: Vec<AlgoSnapshot>) {
        self.snapshots    = snaps;
        self.current_step = 0;
        self.play_state   = PlayState::Playing;
        self.log_lines.clear();
        self.push_log("알고리즘 실행 시작".into(), false);
    }

    /// 권경빈 성능 분석 결과 주입
    pub fn inject_perf(
        &mut self,
        bfs: Option<PerformanceStats>,
        dfs: Option<PerformanceStats>,
        dijk: Option<PerformanceStats>,
    ) {
        self.perf_bfs      = bfs;
        self.perf_dfs      = dfs;
        self.perf_dijkstra = dijk;
    }

    /// UI → 알고리즘 엔진: 현재 그래프/설정 반환
    pub fn export_graph_data(&self) -> GraphData {
        // graph::GraphData 가 from_ui 생성자를 갖는다고 가정
        // 팀원과 인터페이스 맞춰 수정
        GraphData::from_raw(
            self.nodes.iter().map(|n| (n.id, n.x, n.y, n.label.clone())).collect(),
            self.edges.iter().map(|e| (e.from, e.to, e.weight)).collect(),
        )
    }

    pub fn selected_algo(&self) -> AlgorithmType {
        self.selected_algo.clone()
    }

    pub fn start_node_id(&self) -> Option<usize> {
        self.start_node_buf.trim().parse().ok()
    }
}

// ──────────────────────────────────────────────────────────────
// 내부 헬퍼
// ──────────────────────────────────────────────────────────────
impl AlgoSketchApp {
    fn push_log(&mut self, msg: String, active: bool) {
        let n = self.log_lines.len() + 1;
        self.log_lines.push((format!("[Step {}] {}", n, msg), active));
    }

    fn snap(&self) -> Option<&AlgoSnapshot> {
        self.snapshots.get(self.current_step)
    }

    fn node_at(&self, p: Pos2) -> Option<usize> {
        self.nodes.iter().find(|n| {
            Pos2::new(n.x, n.y).distance(p) <= NODE_R
        }).map(|n| n.id)
    }

    fn node_by_id(&self, id: usize) -> Option<&UiNode> {
        self.nodes.iter().find(|n| n.id == id)
    }

    fn node_color(&self, id: usize) -> Color32 {
        if let Some(s) = self.snap() {
            if s.current_node == Some(id) { return C_NODE_CURRENT; }
            if s.visited_nodes.contains(&id) { return C_NODE_VISITED; }
        }
        C_NODE_DEFAULT
    }

    fn code_lines(&self) -> Vec<&'static str> {
        match self.selected_algo {
            AlgorithmType::BFS      => lines_bfs(),
            AlgorithmType::DFS      => lines_dfs(),
            AlgorithmType::Dijkstra => lines_dijkstra(),
        }
    }

    fn algo_label(&self) -> &'static str {
        match self.selected_algo {
            AlgorithmType::BFS      => "BFS",
            AlgorithmType::DFS      => "DFS",
            AlgorithmType::Dijkstra => "Dijkstra",
        }
    }

    fn ds_label(&self) -> &'static str {
        match self.selected_algo {
            AlgorithmType::BFS      => "큐(Queue)",
            AlgorithmType::DFS      => "스택(Stack)",
            AlgorithmType::Dijkstra => "우선순위 큐",
        }
    }

    /// 자동 스텝 진행
    fn tick(&mut self, now: f64) {
        if self.play_state != PlayState::Playing { return; }
        if self.snapshots.is_empty() { return; }
        let interval = 1.0 / self.speed as f64;
        if now - self.last_step_time < interval { return; }
        self.last_step_time = now;

        if let Some(s) = self.snapshots.get(self.current_step) {
            let msg = s.log_message.clone();
            let is_active = true;
            self.log_lines.push((
                format!("[Step {}] {}", self.current_step + 1, msg),
                is_active,
            ));
        }

        if self.current_step + 1 < self.snapshots.len() {
            self.current_step += 1;
        } else {
            self.play_state = PlayState::Finished;
            self.log_lines.push(("✅ 알고리즘 실행 완료".into(), false));
        }
    }

    /// 엣지 중간점 (가중치 편집 클릭 감지용)
    fn edge_midpoint(&self, e: &UiEdge) -> Option<Pos2> {
        let f = self.node_by_id(e.from)?;
        let t = self.node_by_id(e.to)?;
        Some(Pos2::new((f.x + t.x) / 2.0, (f.y + t.y) / 2.0))
    }
}

// ──────────────────────────────────────────────────────────────
// 렌더링 — 사이드바
// ──────────────────────────────────────────────────────────────
impl AlgoSketchApp {
    fn render_sidebar(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.set_min_width(SIDEBAR_W);

        // ── 프로젝트 섹션 ────────────────────────────────
        self.section_label(ui, "프로젝트");
        ui.add_space(4.0);

        if self.icon_btn(ui, "🗋  새로 만들기") {
            self.nodes.clear();
            self.edges.clear();
            self.next_id = 0;
            self.snapshots.clear();
            self.log_lines.clear();
            self.play_state = PlayState::Idle;
            self.current_step = 0;
        }
        if self.icon_btn(ui, "📂  열기") {
            // 권경빈 storage::load() 연동 예정
            self.file_msg = Some(("그래프 불러오기 (storage 연동 예정)".into(),
                ctx.input(|i| i.time)));
        }
        if self.icon_btn(ui, "💾  저장") {
            // 권경빈 storage::save() 연동 예정
            self.file_msg = Some(("그래프 저장 (storage 연동 예정)".into(),
                ctx.input(|i| i.time)));
        }

        ui.add_space(SECTION_GAP);
        self.divider(ui);

        // ── 편집 모드 섹션 ────────────────────────────────
        self.section_label(ui, "편집 도구");
        ui.add_space(4.0);

        // 아이콘 툴바 (가로 배치)
        ui.horizontal_wrapped(|ui| {
            let modes = [
                ("🖱",  "선택",     EditMode::Select),
                ("➕",  "노드 추가", EditMode::AddNode),
                ("🔗",  "간선 추가", EditMode::AddEdge),
                ("🗑",  "삭제",     EditMode::Delete),
                ("✏",  "가중치 편집", EditMode::EditWeight),
            ];
            for (icon, tip, mode) in modes {
                let active = self.edit_mode == mode;
                let btn = egui::Button::new(
                    RichText::new(icon).size(18.0)
                ).fill(if active { C_ACCENT_BLUE } else { C_BG_PANEL })
                 .rounding(Rounding::same(6.0))
                 .min_size(Vec2::new(34.0, 32.0));
                let resp = ui.add(btn).on_hover_text(tip);
                if resp.clicked() {
                    self.edit_mode = mode;
                    self.edge_from = None;
                }
            }
        });

        // 현재 모드 힌트
        ui.add_space(4.0);
        let hint = match self.edit_mode {
            EditMode::Select     => "노드를 드래그하여 이동",
            EditMode::AddNode    => "빈 캔버스 클릭 → 노드 생성",
            EditMode::AddEdge    => "노드 클릭 → 노드 클릭 → 연결",
            EditMode::Delete     => "노드/간선 클릭 → 삭제",
            EditMode::EditWeight => "간선 중앙 클릭 → 가중치 수정",
        };
        ui.label(RichText::new(hint).size(11.0).color(C_TEXT_SUB));

        ui.add_space(SECTION_GAP);
        self.divider(ui);

        // ── 알고리즘 섹션 ─────────────────────────────────
        self.section_label(ui, "알고리즘");
        ui.add_space(4.0);

        for (label, algo) in [
            ("BFS",      AlgorithmType::BFS),
            ("DFS",      AlgorithmType::DFS),
            ("Dijkstra", AlgorithmType::Dijkstra),
        ] {
            let active = self.selected_algo == algo;
            let btn = egui::Button::new(RichText::new(label).size(13.0))
                .fill(if active { C_ACCENT_BLUE } else { C_BG_PANEL })
                .rounding(Rounding::same(6.0))
                .min_size(Vec2::new(BTN_W, BTN_H));
            if ui.add(btn).clicked() {
                self.selected_algo = algo;
            }
            ui.add_space(3.0);
        }

        ui.add_space(6.0);
        ui.horizontal(|ui| {
            ui.label(RichText::new("시작 노드:").size(12.0).color(C_TEXT_SUB));
            ui.add(egui::TextEdit::singleline(&mut self.start_node_buf)
                .desired_width(50.0));
        });

        ui.add_space(SECTION_GAP);
        self.divider(ui);

        // ── 설정 (속도 슬라이더) ─────────────────────────
        self.section_label(ui, "설정");
        ui.add_space(4.0);
        ui.label(RichText::new("애니메이션 속도").size(12.0).color(C_TEXT_SUB));
        ui.horizontal(|ui| {
            ui.label(RichText::new("느림").size(11.0).color(C_TEXT_SUB));
            ui.add(egui::Slider::new(&mut self.speed, 0.2..=5.0)
                .show_value(false)
                .step_by(0.1));
            ui.label(RichText::new("빠름").size(11.0).color(C_TEXT_SUB));
        });

        ui.add_space(SECTION_GAP);
        self.divider(ui);

        // ── 실행 제어 버튼 ───────────────────────────────
        ui.add_space(4.0);

        // 실행
        let run_btn = egui::Button::new(
            RichText::new("▶  실행").size(13.0).color(Color32::WHITE)
        ).fill(C_ACCENT_GREEN)
         .rounding(Rounding::same(6.0))
         .min_size(Vec2::new(BTN_W, BTN_H));
        if ui.add(run_btn).clicked() {
            // 김고운 알고리즘 엔진 호출 예정:
            // let req = self.export_graph_data();
            // let snaps = algorithm::run(req, self.selected_algo(), ...);
            // self.inject_snapshots(snaps);
            //
            // 단독 테스트용 (더미):
            self.play_state   = PlayState::Playing;
            self.current_step = 0;
            self.log_lines.clear();
            self.push_log(
                format!("{} 실행 요청 (시작: {})",
                    self.algo_label(),
                    self.start_node_buf),
                false,
            );
            ctx.request_repaint();
        }
        ui.add_space(3.0);

        // 일시정지 / 계속
        let pause_label = if self.play_state == PlayState::Paused {
            "▶  계속"
        } else {
            "⏸  일시정지"
        };
        let pause_btn = egui::Button::new(
            RichText::new(pause_label).size(13.0)
        ).fill(C_BG_PANEL)
         .rounding(Rounding::same(6.0))
         .min_size(Vec2::new(BTN_W, BTN_H));
        if ui.add(pause_btn).clicked() {
            match self.play_state {
                PlayState::Playing => self.play_state = PlayState::Paused,
                PlayState::Paused  => self.play_state = PlayState::Playing,
                _ => {}
            }
        }
        ui.add_space(3.0);

        // 정지
        let stop_btn = egui::Button::new(
            RichText::new("⏹  정지").size(13.0).color(Color32::WHITE)
        ).fill(C_ACCENT_RED)
         .rounding(Rounding::same(6.0))
         .min_size(Vec2::new(BTN_W, BTN_H));
        if ui.add(stop_btn).clicked() {
            self.play_state   = PlayState::Idle;
            self.current_step = 0;
        }
        ui.add_space(3.0);

        // 초기화
        let reset_btn = egui::Button::new(
            RichText::new("🔄  초기화").size(13.0)
        ).fill(C_BG_PANEL)
         .rounding(Rounding::same(6.0))
         .min_size(Vec2::new(BTN_W, BTN_H));
        if ui.add(reset_btn).clicked() {
            self.snapshots.clear();
            self.current_step = 0;
            self.play_state   = PlayState::Idle;
            self.log_lines.clear();
        }

        // 스텝 수동 이동
        if !self.snapshots.is_empty() {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                if ui.button("◀").clicked() && self.current_step > 0 {
                    self.current_step -= 1;
                    self.play_state = PlayState::Paused;
                }
                ui.label(RichText::new(
                    format!("{}/{}", self.current_step + 1, self.snapshots.len())
                ).size(12.0).color(C_TEXT_SUB));
                if ui.button("▶").clicked()
                    && self.current_step + 1 < self.snapshots.len()
                {
                    self.current_step += 1;
                    self.play_state = PlayState::Paused;
                }
            });
        }

        // 상태 표시 줄
        ui.add_space(8.0);
        let status_color = match self.play_state {
            PlayState::Playing  => C_ACCENT_GREEN,
            PlayState::Paused   => C_ACCENT_ORANGE,
            PlayState::Finished => C_ACCENT_BLUE,
            PlayState::Idle     => C_ACCENT_GRAY,
        };
        let status_text = match self.play_state {
            PlayState::Playing  => format!("● 실행 중 ({})", self.algo_label()),
            PlayState::Paused   => "⏸ 일시정지".into(),
            PlayState::Finished => "✅ 완료".into(),
            PlayState::Idle     => "○ 대기 중".into(),
        };
        ui.label(RichText::new(status_text).size(11.0).color(status_color));

        // 파일 작업 피드백 메시지
        if let Some((ref msg, _t)) = self.file_msg.clone() {
            ui.add_space(4.0);
            ui.label(RichText::new(msg).size(11.0).color(C_ACCENT_BLUE));
        }
    }

    // 사이드바 섹션 레이블
    fn section_label(&self, ui: &mut egui::Ui, text: &str) {
        ui.add_space(6.0);
        ui.label(RichText::new(text).size(12.0).strong().color(C_TEXT_MAIN));
    }

    // 구분선
    fn divider(&self, ui: &mut egui::Ui) {
        let rect = ui.available_rect_before_wrap();
        let y = rect.min.y;
        ui.painter().line_segment(
            [Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)],
            Stroke::new(1.0, C_DIVIDER),
        );
        ui.add_space(6.0);
    }

    // 아이콘+텍스트 버튼
    fn icon_btn(&self, ui: &mut egui::Ui, label: &str) -> bool {
        ui.add(
            egui::Button::new(RichText::new(label).size(12.5))
                .fill(C_BG_PANEL)
                .rounding(Rounding::same(5.0))
                .min_size(Vec2::new(BTN_W, BTN_H)),
        ).clicked()
    }
}

// ──────────────────────────────────────────────────────────────
// 렌더링 — 중앙 그래프 캔버스
// ──────────────────────────────────────────────────────────────
impl AlgoSketchApp {
    fn render_canvas(&mut self, ui: &mut egui::Ui) {
        let avail = ui.available_rect_before_wrap();
        let (resp, painter) =
            ui.allocate_painter(avail.size(), Sense::click_and_drag());
        self.canvas_rect = resp.rect;

        // 배경
        painter.rect_filled(resp.rect, Rounding::ZERO, C_BG_CANVAS);
        // 격자 (연한)
        self.draw_grid(&painter, resp.rect);

        let mouse = resp.hover_pos();

        // ── 드래그 (노드 이동) ────────────────────────────
        if resp.drag_started() {
            if let Some(p) = mouse {
                if self.edit_mode == EditMode::Select {
                    self.dragging = self.node_at(p);
                }
            }
        }
        if resp.dragged() {
            if let (Some(id), Some(p)) = (self.dragging, mouse) {
                if let Some(n) = self.nodes.iter_mut().find(|n| n.id == id) {
                    n.x = p.x;
                    n.y = p.y;
                }
            }
        }
        if resp.drag_stopped() {
            self.dragging = None;
        }

        // ── 클릭 ─────────────────────────────────────────
        if resp.clicked() {
            if let Some(p) = mouse {
                self.handle_canvas_click(p);
            }
        }

        // 활성 간선
        let active_edge = self.snap().and_then(|s| s.active_edge);

        // ── 간선 그리기 ───────────────────────────────────
        for (idx, edge) in self.edges.iter().enumerate() {
            let f = self.node_by_id(edge.from);
            let t = self.node_by_id(edge.to);
            if let (Some(f), Some(t)) = (f, t) {
                let p1 = Pos2::new(f.x, f.y);
                let p2 = Pos2::new(t.x, t.y);
                let is_active = active_edge
                    .map(|(a, b)| (a == edge.from && b == edge.to)
                                || (a == edge.to   && b == edge.from))
                    .unwrap_or(false);

                let (col, w) = if is_active {
                    (C_EDGE_ACTIVE, 3.0)
                } else {
                    (C_EDGE_DEFAULT, 1.8)
                };
                painter.line_segment([p1, p2], Stroke::new(w, col));

                // 가중치 텍스트 (배경 원)
                let mid = Pos2::new((p1.x + p2.x) / 2.0, (p1.y + p2.y) / 2.0);
                painter.circle_filled(mid, 11.0, C_BG_PANEL);
                painter.circle_stroke(mid, 11.0, Stroke::new(1.0, C_DIVIDER));

                // 가중치 편집 중인 간선이면 강조
                if self.editing_edge_idx == Some(idx) {
                    painter.circle_stroke(mid, 13.0, Stroke::new(2.0, C_ACCENT_BLUE));
                }

                painter.text(
                    mid,
                    Align2::CENTER_CENTER,
                    format!("{:.0}", edge.weight),
                    FontId::proportional(11.0),
                    C_TEXT_MAIN,
                );
            }
        }

        // 간선 연결 중: 임시 선
        if self.edit_mode == EditMode::AddEdge {
            if let (Some(fid), Some(mp)) = (self.edge_from, mouse) {
                if let Some(fn_) = self.node_by_id(fid) {
                    painter.line_segment(
                        [Pos2::new(fn_.x, fn_.y), mp],
                        Stroke::new(1.5, C_ACCENT_BLUE),
                    );
                }
            }
        }

        // ── 노드 그리기 ───────────────────────────────────
        let snap_clone = self.snap().cloned();
        for node in &self.nodes {
            let center = Pos2::new(node.x, node.y);
            let fill = if let Some(ref s) = snap_clone {
                if s.current_node == Some(node.id) { C_NODE_CURRENT }
                else if s.visited_nodes.contains(&node.id) { C_NODE_VISITED }
                else { C_NODE_DEFAULT }
            } else {
                C_NODE_DEFAULT
            };

            // 그림자 효과
            painter.circle_filled(
                center + Vec2::new(2.0, 3.0),
                NODE_R,
                Color32::from_rgba_premultiplied(0, 0, 0, 20),
            );
            painter.circle_filled(center, NODE_R, fill);

            let border_w = if self.edge_from == Some(node.id) { 3.0 } else { 1.8 };
            let border_c = if self.edge_from == Some(node.id) {
                C_ACCENT_BLUE
            } else {
                C_NODE_BORDER
            };
            painter.circle_stroke(center, NODE_R, Stroke::new(border_w, border_c));

            painter.text(
                center,
                Align2::CENTER_CENTER,
                &node.label,
                FontId::proportional(15.0),
                C_TEXT_MAIN,
            );
        }

        // ── 빈 캔버스 힌트 ───────────────────────────────
        if self.nodes.is_empty() {
            painter.text(
                resp.rect.center(),
                Align2::CENTER_CENTER,
                "노드 추가 모드에서 캔버스를 클릭하여 그래프를 생성하세요",
                FontId::proportional(14.0),
                Color32::from_rgb(190, 190, 200),
            );
        }

        // ── 우상단 모드 배지 ─────────────────────────────
        let badge = match self.edit_mode {
            EditMode::Select     => "🖱 선택",
            EditMode::AddNode    => "➕ 노드 추가",
            EditMode::AddEdge    => {
                if self.edge_from.is_some() { "🔗 두 번째 노드 선택" }
                else { "🔗 첫 번째 노드 선택" }
            }
            EditMode::Delete     => "🗑 삭제",
            EditMode::EditWeight => "✏ 가중치 편집",
        };
        let badge_pos = resp.rect.min + Vec2::new(10.0, 10.0);
        painter.rect_filled(
            Rect::from_min_size(badge_pos - Vec2::new(4.0, 2.0),
                Vec2::new(160.0, 22.0)),
            Rounding::same(4.0),
            Color32::from_rgba_premultiplied(255, 255, 255, 200),
        );
        painter.text(
            badge_pos,
            Align2::LEFT_TOP,
            badge,
            FontId::proportional(12.0),
            C_TEXT_SUB,
        );
    }

    fn draw_grid(&self, painter: &egui::Painter, rect: Rect) {
        let step = 40.0;
        let col  = Color32::from_rgba_premultiplied(0, 0, 0, 8);
        let mut x = rect.min.x;
        while x < rect.max.x {
            painter.line_segment(
                [Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)],
                Stroke::new(1.0, col),
            );
            x += step;
        }
        let mut y = rect.min.y;
        while y < rect.max.y {
            painter.line_segment(
                [Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)],
                Stroke::new(1.0, col),
            );
            y += step;
        }
    }

    fn handle_canvas_click(&mut self, p: Pos2) {
        match self.edit_mode {
            EditMode::AddNode => {
                let id    = self.next_id;
                let label = Self::node_label(id);
                self.nodes.push(UiNode { id, x: p.x, y: p.y, label });
                self.next_id += 1;
            }
            EditMode::AddEdge => {
                match (self.edge_from, self.node_at(p)) {
                    (None, Some(id)) => {
                        self.edge_from = Some(id);
                    }
                    (Some(fid), Some(tid)) if fid != tid => {
                        self.pending_edge       = Some((fid, tid));
                        self.show_weight_popup  = true;
                        self.weight_buf         = "1".into();
                        self.edge_from          = None;
                    }
                    _ => { self.edge_from = None; }
                }
            }
            EditMode::Delete => {
                if let Some(id) = self.node_at(p) {
                    self.nodes.retain(|n| n.id != id);
                    self.edges.retain(|e| e.from != id && e.to != id);
                } else {
                    // 간선 삭제: 중간점 근처 클릭
                    let edges_clone = self.edges.clone();
                    for (i, e) in edges_clone.iter().enumerate() {
                        if let Some(mid) = self.edge_midpoint(e) {
                            if mid.distance(p) < 14.0 {
                                self.edges.remove(i);
                                break;
                            }
                        }
                    }
                }
            }
            EditMode::EditWeight => {
                let edges_clone = self.edges.clone();
                for (i, e) in edges_clone.iter().enumerate() {
                    if let Some(mid) = self.edge_midpoint(e) {
                        if mid.distance(p) < 14.0 {
                            self.editing_edge_idx   = Some(i);
                            self.editing_weight_buf =
                                format!("{:.0}", e.weight);
                            break;
                        }
                    }
                }
            }
            EditMode::Select => {}
        }
    }

    /// 노드 라벨: 0→A, 1→B, …
    fn node_label(id: usize) -> String {
        let c = (b'A' + (id % 26) as u8) as char;
        if id < 26 { c.to_string() }
        else { format!("{}{}", c, id / 26) }
    }
}

// ──────────────────────────────────────────────────────────────
// 렌더링 — 우측 코드 패널
// ──────────────────────────────────────────────────────────────
impl AlgoSketchApp {
    fn render_code_panel(&self, ui: &mut egui::Ui) {
        Frame::none()
            .fill(C_BG_CODE)
            .inner_margin(Margin::symmetric(10.0, 10.0))
            .show(ui, |ui| {
                ui.set_min_width(CODE_W);

                // 헤더
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(format!("코드 ({}.java)", self.algo_label()))
                            .size(12.5)
                            .color(Color32::from_rgb(150, 155, 170))
                            .strong(),
                    );
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        // 범례
                        ui.label(
                            RichText::new("■ 다음 실행 라인")
                                .size(10.0)
                                .color(C_CODE_HL_NEXT),
                        );
                        ui.add_space(4.0);
                        ui.label(
                            RichText::new("■ 현재 실행 라인")
                                .size(10.0)
                                .color(C_CODE_HL_CURR),
                        );
                    });
                });
                ui.add_space(6.0);

                let curr_line = self.snap().map(|s| s.current_line).unwrap_or(usize::MAX);
                let lines     = self.code_lines();

                ScrollArea::vertical()
                    .id_source("code_scroll")
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        for (i, line) in lines.iter().enumerate() {
                            let is_curr = i == curr_line;
                            let is_next = i == curr_line + 1;

                            let (bg, text_color) = if is_curr {
                                (C_CODE_HL_CURR, C_CODE_HL_CURR_TEXT)
                            } else if is_next {
                                (C_CODE_HL_NEXT, C_CODE_HL_NEXT_TEXT)
                            } else {
                                (Color32::TRANSPARENT, C_CODE_TEXT)
                            };

                            Frame::none()
                                .fill(bg)
                                .inner_margin(Margin { left: 4.0, right: 4.0,
                                    top: 1.5, bottom: 1.5 })
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        // 줄 번호
                                        ui.add_sized(
                                            [22.0, 16.0],
                                            egui::Label::new(
                                                RichText::new(format!("{:>2}", i + 1))
                                                    .monospace()
                                                    .size(12.0)
                                                    .color(C_CODE_NUM),
                                            ),
                                        );
                                        // 코드
                                        ui.label(
                                            RichText::new(*line)
                                                .monospace()
                                                .size(12.0)
                                                .color(text_color),
                                        );
                                    });
                                });
                        }
                    });
            });
    }
}

// ──────────────────────────────────────────────────────────────
// 렌더링 — 하단 왼쪽: 실행 로그
// ──────────────────────────────────────────────────────────────
impl AlgoSketchApp {
    fn render_log(&self, ui: &mut egui::Ui) {
        Frame::none()
            .fill(C_BG_LOG)
            .inner_margin(Margin::symmetric(8.0, 6.0))
            .show(ui, |ui| {
                ui.label(
                    RichText::new("실행 로그")
                        .size(12.0)
                        .strong()
                        .color(C_TEXT_MAIN),
                );
                ui.add_space(4.0);

                ScrollArea::vertical()
                    .id_source("log_scroll")
                    .stick_to_bottom(true)
                    .max_height(BOTTOM_H - 50.0)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        for (text, active) in &self.log_lines {
                            let color = if *active { C_LOG_ACTIVE } else { C_LOG_STEP };
                            ui.label(
                                RichText::new(text)
                                    .monospace()
                                    .size(11.5)
                                    .color(color),
                            );
                        }
                        if self.log_lines.is_empty() {
                            ui.label(
                                RichText::new("알고리즘 실행 후 로그가 출력됩니다.")
                                    .size(11.5)
                                    .color(C_TEXT_SUB),
                            );
                        }
                    });
            });
    }
}

// ──────────────────────────────────────────────────────────────
// 렌더링 — 하단 가운데: 실행 정보 패널
// ──────────────────────────────────────────────────────────────
impl AlgoSketchApp {
    fn render_info_panel(&self, ui: &mut egui::Ui) {
        Frame::none()
            .fill(C_BG_PANEL)
            .inner_margin(Margin::symmetric(10.0, 8.0))
            .show(ui, |ui| {
                ui.label(
                    RichText::new("실행 정보")
                        .size(12.0)
                        .strong()
                        .color(C_TEXT_MAIN),
                );
                ui.add_space(6.0);

                if let Some(s) = self.snap() {
                    self.info_row(ui, "알고리즘",
                        &RichText::new(self.algo_label())
                            .color(C_ACCENT_BLUE).size(12.5));
                    self.info_row(ui, "시작 노드",
                        &RichText::new(&self.start_node_buf).size(12.5));
                    self.info_row(ui, "현재 방문 노드",
                        &RichText::new(
                            s.current_node.map(|id| Self::node_label(id))
                                          .unwrap_or_else(|| "-".into())
                        ).color(C_ACCENT_GREEN).size(12.5));
                    self.info_row(ui, "방문 순서",
                        &RichText::new(
                            s.visit_order.iter()
                                .map(|id| Self::node_label(*id))
                                .collect::<Vec<_>>()
                                .join(", ")
                        ).size(12.0));
                    self.info_row(ui, self.ds_label(),
                        &RichText::new(
                            format!("[{}]",
                                s.queue_or_stack.iter()
                                    .map(|id| Self::node_label(*id))
                                    .collect::<Vec<_>>()
                                    .join(", "))
                        ).size(12.0));

                    // 방문 체크 배열
                    ui.add_space(4.0);
                    ui.label(RichText::new("방문 체크").size(11.5).color(C_TEXT_SUB));
                    ui.horizontal_wrapped(|ui| {
                        for node in &self.nodes {
                            let visited = s.visited_nodes.contains(&node.id);
                            let color = if visited { C_ACCENT_GREEN } else { C_ACCENT_RED };
                            let mark  = if visited { "✔" } else { "–" };
                            ui.label(
                                RichText::new(format!("{} {}", node.label, mark))
                                    .size(11.5)
                                    .color(color),
                            );
                        }
                    });
                } else {
                    ui.label(
                        RichText::new("(실행 전)")
                            .size(12.0)
                            .color(C_TEXT_SUB),
                    );
                }
            });
    }

    fn info_row(&self, ui: &mut egui::Ui, label: &str, value: &RichText) {
        ui.horizontal(|ui| {
            ui.label(
                RichText::new(format!("{}: ", label))
                    .size(11.5)
                    .color(C_TEXT_SUB),
            );
            ui.label(value.clone());
        });
        ui.add_space(2.0);
    }
}

// ──────────────────────────────────────────────────────────────
// 렌더링 — 하단 오른쪽: 성능 비교 Bar Chart
// ──────────────────────────────────────────────────────────────
impl AlgoSketchApp {
    fn render_perf_chart(&self, ui: &mut egui::Ui) {
        Frame::none()
            .fill(C_BG_PANEL)
            .inner_margin(Margin::symmetric(10.0, 8.0))
            .show(ui, |ui| {
                ui.label(
                    RichText::new("성능 비교")
                        .size(12.0)
                        .strong()
                        .color(C_TEXT_MAIN),
                );
                ui.add_space(4.0);

                let has_data = self.perf_bfs.is_some()
                    || self.perf_dfs.is_some()
                    || self.perf_dijkstra.is_some();

                if !has_data {
                    ui.label(
                        RichText::new("알고리즘 실행 후 성능 데이터가 표시됩니다.")
                            .size(11.5)
                            .color(C_TEXT_SUB),
                    );
                    return;
                }

                ui.columns(2, |cols| {
                    // 왼쪽: 실행 시간 비교
                    self.draw_bar_chart(
                        &mut cols[0],
                        "실행 시간 비교 (ms)",
                        &[
                            ("BFS",      self.perf_bfs.as_ref().map(|p| p.exec_time_ms)),
                            ("DFS",      self.perf_dfs.as_ref().map(|p| p.exec_time_ms)),
                            ("Dijkstra", self.perf_dijkstra.as_ref().map(|p| p.exec_time_ms)),
                        ],
                    );
                    // 오른쪽: 방문 노드 수 비교
                    self.draw_bar_chart(
                        &mut cols[1],
                        "방문 노드 수 비교",
                        &[
                            ("BFS",      self.perf_bfs.as_ref().map(|p| p.visited_count as f64)),
                            ("DFS",      self.perf_dfs.as_ref().map(|p| p.visited_count as f64)),
                            ("Dijkstra", self.perf_dijkstra.as_ref().map(|p| p.visited_count as f64)),
                        ],
                    );
                });
            });
    }

    fn draw_bar_chart(
        &self,
        ui: &mut egui::Ui,
        title: &str,
        data: &[(&str, Option<f64>)],
    ) {
        ui.label(RichText::new(title).size(11.0).color(C_TEXT_SUB));
        ui.add_space(2.0);

        let avail = ui.available_rect_before_wrap();
        let chart_h = 80.0;
        let (_, painter) = ui.allocate_painter(
            Vec2::new(avail.width(), chart_h),
            Sense::hover(),
        );

        let bar_colors = [C_ACCENT_BLUE, C_ACCENT_GREEN, C_ACCENT_ORANGE];
        let valid: Vec<(&str, f64)> = data.iter()
            .filter_map(|(n, v)| v.map(|val| (*n, val)))
            .collect();

        if valid.is_empty() { return; }

        let max_val = valid.iter().map(|(_, v)| *v).fold(0.0_f64, f64::max).max(1.0);
        let n       = valid.len();
        let gap     = 10.0;
        let bar_w   = (avail.width() / n as f32) - gap;

        for (i, (name, val)) in valid.iter().enumerate() {
            let ratio = (*val / max_val) as f32;
            let bar_h = ratio * (chart_h - 24.0);
            let x     = avail.min.x + i as f32 * (bar_w + gap) + gap / 2.0;
            let y_bot = avail.min.y + chart_h - 16.0;
            let rect  = Rect::from_min_max(
                Pos2::new(x, y_bot - bar_h),
                Pos2::new(x + bar_w, y_bot),
            );
            painter.rect_filled(rect, Rounding::same(3.0), bar_colors[i % 3]);
            painter.text(
                Pos2::new(x + bar_w / 2.0, y_bot + 8.0),
                Align2::CENTER_CENTER,
                name,
                FontId::proportional(10.5),
                C_TEXT_SUB,
            );
            painter.text(
                Pos2::new(x + bar_w / 2.0, y_bot - bar_h - 4.0),
                Align2::CENTER_BOTTOM,
                format!("{:.2}", val),
                FontId::proportional(10.0),
                C_TEXT_MAIN,
            );
        }
    }
}

// ──────────────────────────────────────────────────────────────
// 팝업
// ──────────────────────────────────────────────────────────────
impl AlgoSketchApp {
    fn render_weight_popup(&mut self, ctx: &egui::Context) {
        if !self.show_weight_popup { return; }

        egui::Window::new("간선 가중치 입력")
            .collapsible(false)
            .resizable(false)
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.label("두 노드를 연결할 가중치를 입력하세요:");
                ui.add_space(4.0);
                let resp = ui.add(
                    egui::TextEdit::singleline(&mut self.weight_buf)
                        .desired_width(80.0),
                );
                if resp.lost_focus()
                    && ctx.input(|i| i.key_pressed(egui::Key::Enter))
                {
                    self.confirm_edge();
                }
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    if ui.button("확인").clicked() { self.confirm_edge(); }
                    if ui.button("취소").clicked() {
                        self.show_weight_popup = false;
                        self.pending_edge      = None;
                    }
                });
            });
    }

    fn confirm_edge(&mut self) {
        let w = self.weight_buf.parse::<f32>().unwrap_or(1.0);
        if let Some((from, to)) = self.pending_edge {
            self.edges.push(UiEdge { from, to, weight: w });
        }
        self.show_weight_popup = false;
        self.pending_edge      = None;
    }

    fn render_weight_edit_popup(&mut self, ctx: &egui::Context) {
        if self.editing_edge_idx.is_none() { return; }

        egui::Window::new("가중치 수정")
            .collapsible(false)
            .resizable(false)
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.label("새 가중치:");
                ui.text_edit_singleline(&mut self.editing_weight_buf);
                ui.horizontal(|ui| {
                    if ui.button("적용").clicked() {
                        if let Some(idx) = self.editing_edge_idx {
                            let w = self.editing_weight_buf
                                .parse::<f32>()
                                .unwrap_or(1.0);
                            if let Some(e) = self.edges.get_mut(idx) {
                                e.weight = w;
                            }
                        }
                        self.editing_edge_idx = None;
                    }
                    if ui.button("취소").clicked() {
                        self.editing_edge_idx = None;
                    }
                });
            });
    }
}

// ──────────────────────────────────────────────────────────────
// eframe::App 구현
// ──────────────────────────────────────────────────────────────
impl eframe::App for AlgoSketchApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 자동 스텝 진행
        let now = ctx.input(|i| i.time);
        self.tick(now);
        if self.play_state == PlayState::Playing {
            ctx.request_repaint();
        }

        // 파일 메시지 3초 후 자동 삭제
        if let Some((_, t)) = self.file_msg.clone() {
            if now - t > 3.0 { self.file_msg = None; }
        }

        // ── 팝업 ─────────────────────────────────────────
        self.render_weight_popup(ctx);
        self.render_weight_edit_popup(ctx);

        // ── 좌측 사이드바 ─────────────────────────────────
        egui::SidePanel::left("sidebar")
            .exact_width(SIDEBAR_W)
            .resizable(false)
            .frame(Frame::none()
                .fill(C_BG_SIDEBAR)
                .inner_margin(Margin::symmetric(12.0, 10.0)))
            .show(ctx, |ui| {
                ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        self.render_sidebar(ui, ctx);
                    });
            });

        // ── 우측 코드 패널 ────────────────────────────────
        egui::SidePanel::right("code_panel")
            .exact_width(CODE_W)
            .resizable(false)
            .frame(Frame::none().fill(C_BG_CODE))
            .show(ctx, |ui| {
                self.render_code_panel(ui);
            });

        // ── 하단 패널 ─────────────────────────────────────
        egui::TopBottomPanel::bottom("bottom_panel")
            .exact_height(BOTTOM_H)
            .frame(Frame::none()
                .fill(C_BG_MAIN)
                .inner_margin(Margin::symmetric(0.0, 0.0)))
            .show(ctx, |ui| {
                ui.columns(3, |cols| {
                    // 실행 로그
                    self.render_log(&mut cols[0]);
                    // 실행 정보
                    self.render_info_panel(&mut cols[1]);
                    // 성능 차트
                    self.render_perf_chart(&mut cols[2]);
                });
            });

        // ── 중앙 캔버스 ───────────────────────────────────
        egui::CentralPanel::default()
            .frame(Frame::none().fill(C_BG_CANVAS))
            .show(ctx, |ui| {
                self.render_canvas(ui);
            });
    }
}