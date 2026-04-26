use crate::app::{AppState, MessageRole, Tab};
use crate::event::EventSender;
use crate::proc::ProcessManager;

#[derive(Debug, Clone)]
pub struct CommandSpec {
    pub name: &'static str,
    pub description: &'static str,
    pub category: &'static str,
}

pub const COMMANDS: &[CommandSpec] = &[
    CommandSpec { name: "/help",       description: "Toggle help overlay",                  category: "core" },
    CommandSpec { name: "/clear",      description: "Clear chat history",                   category: "core" },
    CommandSpec { name: "/status",     description: "Refresh system status",                category: "core" },
    CommandSpec { name: "/quit",       description: "Exit the TUI",                         category: "core" },
    CommandSpec { name: "/chat",       description: "Switch to Chat tab",                   category: "nav" },
    CommandSpec { name: "/mesh",       description: "Switch to Mesh tab",                   category: "nav" },
    CommandSpec { name: "/telemetry",  description: "Switch to Telemetry tab",              category: "nav" },
    CommandSpec { name: "/dream",      description: "Switch to Dream tab",                  category: "nav" },
    CommandSpec { name: "/system",     description: "Switch to System tab",                 category: "nav" },
    CommandSpec { name: "/run dream",  description: "Run dream maintenance cycle",          category: "system" },
    CommandSpec { name: "/run dream-micro", description: "Run micro dream cycle",           category: "system" },
    CommandSpec { name: "/run live",   description: "Boot Medulla API + Next.js dev",       category: "system" },
    CommandSpec { name: "/run server", description: "Start Medulla API server only",        category: "system" },
    CommandSpec { name: "/run sovereign", description: "Launch sovereign Docker workspace", category: "system" },
    CommandSpec { name: "/run recon",  description: "Run social recon pipeline",            category: "system" },
    CommandSpec { name: "/run reset",  description: "Reset Medulla ledger (destructive)",   category: "system" },
    CommandSpec { name: "/kill",       description: "Kill the selected System process",     category: "system" },
    CommandSpec { name: "/solve",      description: "Solve a millennium problem (arg)",     category: "task" },
    CommandSpec { name: "/discipline", description: "Absorb a discipline (arg)",            category: "task" },
    CommandSpec { name: "/provider",   description: "Pin a provider for next request",      category: "task" },
    CommandSpec { name: "/verify",     description: "Run ADCCL verify on last response",    category: "task" },
];

#[derive(Debug, Clone)]
pub enum RouteOutcome {
    Handled,
    SendToChat(String),
    Quit,
}

pub struct Router<'a> {
    pub state: &'a mut AppState,
    pub tx: EventSender,
    pub pm: ProcessManager,
    pub repo_dir: String,
    pub chyren_bin: String,
}

impl<'a> Router<'a> {
    pub fn dispatch(&mut self, raw: &str) -> RouteOutcome {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return RouteOutcome::Handled;
        }

        if !trimmed.starts_with('/') {
            return RouteOutcome::SendToChat(trimmed.to_string());
        }

        let mut parts = trimmed.splitn(2, char::is_whitespace);
        let cmd = parts.next().unwrap_or("");
        let arg = parts.next().unwrap_or("").trim();

        match cmd {
            "/help" => {
                self.state.show_help = !self.state.show_help;
            }
            "/clear" => self.state.chat.clear(),
            "/status" => {
                self.system_msg("Refreshing status...");
            }
            "/quit" | "/exit" => return RouteOutcome::Quit,
            "/chat" => self.state.switch_tab(Tab::Chat),
            "/mesh" => self.state.switch_tab(Tab::Mesh),
            "/telemetry" => self.state.switch_tab(Tab::Telemetry),
            "/dream" => self.state.switch_tab(Tab::Dream),
            "/system" => self.state.switch_tab(Tab::System),
            "/run" => self.handle_run(arg),
            "/kill" => self.handle_kill(),
            "/solve" => {
                if arg.is_empty() {
                    self.system_msg("Usage: /solve <problem>");
                } else {
                    return RouteOutcome::SendToChat(format!("Solve: {}", arg));
                }
            }
            "/discipline" => {
                if arg.is_empty() {
                    self.system_msg("Usage: /discipline <name>");
                } else {
                    return RouteOutcome::SendToChat(format!("Absorb discipline: {}", arg));
                }
            }
            "/provider" => {
                if arg.is_empty() {
                    self.system_msg(&format!("Current provider: {}", self.state.chat.provider));
                } else {
                    self.state.chat.provider = arg.to_string();
                    self.state.status.provider = arg.to_string();
                    self.system_msg(&format!("Provider pinned: {}", arg));
                }
            }
            "/verify" => {
                let last_asst = self.state.chat.messages.iter().rev().find(|m| m.role == MessageRole::Chyren);
                let last_user = self.state.chat.messages.iter().rev().find(|m| m.role == MessageRole::User);

                if let (Some(asst), Some(user)) = (last_asst, last_user) {
                    let adccl = chyren_adccl::ADCCL::new(self.state.chat.adccl_score, None);
                    let result = adccl.verify(&asst.content, &user.content);
                    
                    let mut flags_str = result.flags.join(", ");
                    if flags_str.is_empty() { flags_str = "None".to_string(); }

                    let msg = format!(
                        "ADCCL Verification for last response:\n\n\
                        ◈ Status: {}\n\
                        ◈ Score:  {:.2}\n\
                        ◈ Flags:  {}\n\n\
                        Constitutional alignment verified at threshold {:.2}.",
                        result.status.to_uppercase(),
                        result.score,
                        flags_str,
                        self.state.chat.adccl_score
                    );
                    self.system_msg(&msg);
                } else {
                    self.system_msg("No conversation pair found to verify.");
                }
            }
            _ => {
                self.system_msg(&format!("Unknown command: {}", cmd));
            }
        }

        RouteOutcome::Handled
    }

    fn handle_run(&mut self, arg: &str) {
        let target = arg.split_whitespace().next().unwrap_or("");
        match target {
            "dream" => self.spawn_chyren(&["dream"], "dream"),
            "dream-micro" => self.spawn_chyren(&["dream", "--micro"], "dream-micro"),
            "live" => self.spawn_chyren(&["live"], "live"),
            "server" => self.spawn_medulla_bin("server"),
            "sovereign" => self.spawn_chyren(&["sovereign"], "sovereign"),
            "recon" => self.spawn_chyren(&["recon"], "recon"),
            "reset" => self.spawn_chyren(&["reset", "--yes"], "reset"),
            "" => self.system_msg("Usage: /run <dream|dream-micro|live|server|sovereign|recon|reset>"),
            other => self.system_msg(&format!("Unknown run target: {}", other)),
        }
    }

    fn spawn_chyren(&mut self, args: &[&str], label: &str) {
        let _id = self.pm.spawn_streaming(
            label,
            &self.chyren_bin,
            args,
            Some(&self.repo_dir),
            self.tx.clone(),
        );
        self.state.switch_tab(Tab::System);
        self.system_msg(&format!("Spawned: {} {}", self.chyren_bin, args.join(" ")));
    }

    fn spawn_medulla_bin(&mut self, sub: &str) {
        let _id = self.pm.spawn_streaming(
            sub,
            &self.chyren_bin,
            &[sub],
            Some(&self.repo_dir),
            self.tx.clone(),
        );
        self.state.switch_tab(Tab::System);
        self.system_msg(&format!("Spawned: {} {}", self.chyren_bin, sub));
    }

    fn handle_kill(&mut self) {
        if let Some(entry) = self.state.proc.selected_entry() {
            let id = entry.id.clone();
            let id_for_msg = id.clone();
            let pm = self.pm.clone();
            tokio::spawn(async move { pm.kill(&id).await });
            self.system_msg(&format!("Sent kill signal to {}", id_for_msg));
        } else {
            self.system_msg("No process selected.");
        }
    }

    fn system_msg(&mut self, msg: &str) {
        self.state
            .chat
            .add_message(MessageRole::Chyren, format!("◈ {}", msg));
    }
}
