use crate::event::{ChatResponse, SystemEvent};
use crate::input::InputBuffer;
use chrono::Local;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    Normal,
    Insert,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Chat,
    Mesh,
    Telemetry,
    Dream,
}

impl Tab {
    pub fn next(self) -> Self {
        match self {
            Tab::Chat => Tab::Mesh,
            Tab::Mesh => Tab::Telemetry,
            Tab::Telemetry => Tab::Dream,
            Tab::Dream => Tab::Chat,
        }
    }

    pub fn index(self) -> usize {
        match self {
            Tab::Chat => 0,
            Tab::Mesh => 1,
            Tab::Telemetry => 2,
            Tab::Dream => 3,
        }
    }

    pub fn from_index(i: usize) -> Option<Self> {
        match i {
            0 => Some(Tab::Chat),
            1 => Some(Tab::Mesh),
            2 => Some(Tab::Telemetry),
            3 => Some(Tab::Dream),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Message {
    pub role: MessageRole,
    pub content: String,
    pub timestamp: f64,
    pub adccl_score: Option<f64>,
    pub provider: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageRole {
    User,
    Chyren,
}

#[derive(Debug, Clone)]
pub struct ChatState {
    pub messages: Vec<Message>,
    pub streaming: bool,
    pub streaming_buffer: String,
    pub adccl_score: f64,
    pub provider: String,
    pub tier: u32,
    pub latency_ms: f64,
}

impl ChatState {
    pub fn new() -> Self {
        Self {
            messages: vec![Message {
                role: MessageRole::Chyren,
                content: "◈ CHYREN — Sovereign Intelligence Orchestrator initialized. Ready to reason and execute.".to_string(),
                timestamp: current_timestamp(),
                adccl_score: None,
                provider: None,
            }],
            streaming: false,
            streaming_buffer: String::new(),
            adccl_score: 0.7,
            provider: "openrouter".to_string(),
            tier: 0,
            latency_ms: 0.0,
        }
    }

    pub fn add_message(&mut self, role: MessageRole, content: String) {
        self.messages.push(Message {
            role,
            content,
            timestamp: current_timestamp(),
            adccl_score: if role == MessageRole::Chyren { Some(self.adccl_score) } else { None },
            provider: if role == MessageRole::Chyren { Some(self.provider.clone()) } else { None },
        });
    }

    pub fn start_streaming(&mut self) {
        self.streaming = true;
        self.streaming_buffer.clear();
    }

    pub fn add_stream_chunk(&mut self, chunk: String) {
        self.streaming_buffer.push_str(&chunk);
    }

    pub fn finish_streaming(&mut self) {
        self.streaming = false;
        if !self.streaming_buffer.is_empty() {
            self.messages.push(Message {
                role: MessageRole::Chyren,
                content: self.streaming_buffer.clone(),
                timestamp: current_timestamp(),
                adccl_score: Some(self.adccl_score),
                provider: Some(self.provider.clone()),
            });
        }
        self.streaming_buffer.clear();
    }

    pub fn clear(&mut self) {
        self.messages.clear();
        self.add_message(MessageRole::Chyren, "◈ History cleared.".to_string());
    }
}

impl Default for ChatState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct AgentEntry {
    pub id: String,
    pub status: AgentStatus,
    pub last_active: f64,
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentStatus {
    Idle,
    Busy,
    Offline,
}

#[derive(Debug, Clone)]
pub struct MeshState {
    pub agents: Vec<AgentEntry>,
    pub last_updated: f64,
}

impl MeshState {
    pub fn new() -> Self {
        Self {
            agents: vec![
                AgentEntry {
                    id: "MathSpoke·1".to_string(),
                    status: AgentStatus::Idle,
                    last_active: current_timestamp(),
                    capabilities: vec!["arithmetic".to_string(), "topology".to_string()],
                },
                AgentEntry {
                    id: "MathSpoke·2".to_string(),
                    status: AgentStatus::Idle,
                    last_active: current_timestamp(),
                    capabilities: vec!["number_theory".to_string()],
                },
                AgentEntry {
                    id: "IngestorAgent".to_string(),
                    status: AgentStatus::Idle,
                    last_active: current_timestamp(),
                    capabilities: vec!["ingest".to_string(), "vector".to_string()],
                },
            ],
            last_updated: current_timestamp(),
        }
    }
}

impl Default for MeshState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct TelemetryEvent {
    pub component: String,
    pub event_type: String,
    pub level: String,
    pub timestamp: f64,
}

#[derive(Debug, Clone)]
pub struct TelemetryState {
    pub events: Vec<TelemetryEvent>,
    pub filter: String,
    pub max_events: usize,
}

impl TelemetryState {
    pub fn new() -> Self {
        Self {
            events: vec![TelemetryEvent {
                component: "Chyren".to_string(),
                event_type: "BOOT_COMPLETE".to_string(),
                level: "INFO".to_string(),
                timestamp: current_timestamp(),
            }],
            filter: String::new(),
            max_events: 500,
        }
    }

    pub fn add_event(&mut self, event: SystemEvent) {
        if self.events.len() >= self.max_events {
            self.events.remove(0);
        }
        self.events.push(TelemetryEvent {
            component: event.component,
            event_type: event.event_type,
            level: event.level,
            timestamp: event.timestamp,
        });
    }

    pub fn filtered_events(&self) -> Vec<&TelemetryEvent> {
        if self.filter.is_empty() {
            self.events.iter().collect()
        } else {
            self.events
                .iter()
                .filter(|e| e.component.contains(&self.filter))
                .collect()
        }
    }
}

impl Default for TelemetryState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct DreamState {
    pub episodes: Vec<DreamEpisode>,
    pub chi: f64,
    pub omega: f64,
}

#[derive(Debug, Clone)]
pub struct DreamEpisode {
    pub id: String,
    pub task_summary: String,
    pub score: f64,
    pub timestamp: f64,
    pub flags: Vec<String>,
}

impl DreamState {
    pub fn new() -> Self {
        Self {
            episodes: vec![],
            chi: 0.73,
            omega: 0.82,
        }
    }
}

impl Default for DreamState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct SystemStatus {
    pub connected: bool,
    pub provider: String,
    pub latency_ms: f64,
    pub active_runs: usize,
    pub total_runs: usize,
    pub dream_episodes: usize,
}

impl Default for SystemStatus {
    fn default() -> Self {
        Self {
            connected: true,
            provider: "openrouter".to_string(),
            latency_ms: 0.0,
            active_runs: 0,
            total_runs: 0,
            dream_episodes: 0,
        }
    }
}

pub struct AppState {
    pub mode: AppMode,
    pub active_tab: Tab,
    pub chat: ChatState,
    pub mesh: MeshState,
    pub telemetry: TelemetryState,
    pub dream: DreamState,
    pub input: InputBuffer,
    pub status: SystemStatus,
    pub show_help: bool,
    pub show_command_palette: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            mode: AppMode::Normal,
            active_tab: Tab::Chat,
            chat: ChatState::new(),
            mesh: MeshState::new(),
            telemetry: TelemetryState::new(),
            dream: DreamState::new(),
            input: InputBuffer::new(),
            status: SystemStatus::default(),
            show_help: false,
            show_command_palette: false,
        }
    }

    pub fn set_mode(&mut self, mode: AppMode) {
        self.mode = mode;
    }

    pub fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            AppMode::Normal => AppMode::Insert,
            AppMode::Insert => AppMode::Normal,
        };
    }

    pub fn switch_tab(&mut self, tab: Tab) {
        self.active_tab = tab;
    }

    pub fn next_tab(&mut self) {
        self.active_tab = self.active_tab.next();
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

fn current_timestamp() -> f64 {
    Local::now().timestamp() as f64
}
