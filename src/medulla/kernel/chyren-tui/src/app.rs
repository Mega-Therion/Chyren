use crate::event::{StatusSnapshot, SystemEvent};
use crate::input::InputBuffer;
use crate::proc::ProcessRegistry;
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
    System,
}

impl Tab {
    pub fn next(self) -> Self {
        match self {
            Tab::Chat => Tab::Mesh,
            Tab::Mesh => Tab::Telemetry,
            Tab::Telemetry => Tab::Dream,
            Tab::Dream => Tab::System,
            Tab::System => Tab::Chat,
        }
    }

    pub fn index(self) -> usize {
        match self {
            Tab::Chat => 0,
            Tab::Mesh => 1,
            Tab::Telemetry => 2,
            Tab::Dream => 3,
            Tab::System => 4,
        }
    }

    pub fn from_index(i: usize) -> Option<Self> {
        match i {
            0 => Some(Tab::Chat),
            1 => Some(Tab::Mesh),
            2 => Some(Tab::Telemetry),
            3 => Some(Tab::Dream),
            4 => Some(Tab::System),
            _ => None,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Tab::Chat => "Chat",
            Tab::Mesh => "Mesh",
            Tab::Telemetry => "Telemetry",
            Tab::Dream => "Dream",
            Tab::System => "System",
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    pub scroll: u16,
}

impl ChatState {
    pub fn new() -> Self {
        Self {
            messages: vec![Message {
                role: MessageRole::Chyren,
                content: "◈ CHYREN — Sovereign Intelligence Orchestrator initialized.\nType naturally to chat, or use slash commands (try /help).".to_string(),
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
            scroll: 0,
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

impl AgentStatus {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "idle" | "ready" => Self::Idle,
            "busy" | "running" | "active" => Self::Busy,
            _ => Self::Offline,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MeshState {
    pub agents: Vec<AgentEntry>,
    pub last_updated: f64,
}

impl MeshState {
    pub fn new() -> Self {
        Self {
            agents: default_agent_seed(),
            last_updated: current_timestamp(),
        }
    }

    pub fn replace_from_api(&mut self, agents: Vec<crate::event::MeshAgent>) {
        let now = current_timestamp();
        self.agents = agents
            .into_iter()
            .map(|a| AgentEntry {
                id: a.id,
                status: AgentStatus::from_str(&a.status),
                last_active: now - a.last_active_secs,
                capabilities: a.capabilities,
            })
            .collect();
        self.last_updated = now;
    }
}

fn default_agent_seed() -> Vec<AgentEntry> {
    vec![
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
    ]
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
    pub filter_mode: bool,
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
            filter_mode: false,
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
            let needle = self.filter.to_lowercase();
            self.events
                .iter()
                .filter(|e| {
                    e.component.to_lowercase().contains(&needle)
                        || e.event_type.to_lowercase().contains(&needle)
                        || e.level.to_lowercase().contains(&needle)
                })
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
    pub chyren: f64,
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
            chyren: 0.82,
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
            connected: false,
            provider: "openrouter".to_string(),
            latency_ms: 0.0,
            active_runs: 0,
            total_runs: 0,
            dream_episodes: 0,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PaletteState {
    pub query: String,
    pub selected: usize,
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
    pub proc: ProcessRegistry,
    pub palette: PaletteState,
    pub show_help: bool,
    pub show_command_palette: bool,
    pub should_quit: bool,
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
            proc: ProcessRegistry::new(),
            palette: PaletteState::default(),
            show_help: false,
            show_command_palette: false,
            should_quit: false,
        }
    }

    pub fn set_mode(&mut self, mode: AppMode) {
        self.mode = mode;
    }

    pub fn switch_tab(&mut self, tab: Tab) {
        self.active_tab = tab;
    }

    pub fn next_tab(&mut self) {
        self.active_tab = self.active_tab.next();
    }

    pub fn apply_status(&mut self, snap: StatusSnapshot) {
        self.status.connected = snap.api_reachable;
        if !snap.provider.is_empty() {
            self.status.provider = snap.provider.clone();
            self.chat.provider = snap.provider;
        }
        if snap.adccl_score > 0.0 {
            self.chat.adccl_score = snap.adccl_score;
        }
        self.status.latency_ms = snap.latency_ms;
        self.status.active_runs = snap.active_runs;
        self.status.total_runs = snap.total_runs;
        self.status.dream_episodes = snap.dream_episodes;
        if snap.chi > 0.0 {
            self.dream.chi = snap.chi;
        }
        if snap.chyren > 0.0 {
            self.dream.chyren = snap.chyren;
        }
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
