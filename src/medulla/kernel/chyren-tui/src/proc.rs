use crate::event::{Event, EventSender};
use std::collections::HashMap;
use std::process::Stdio;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::Mutex as AsyncMutex;

#[derive(Debug, Clone)]
pub struct ProcEntry {
    pub id: String,
    pub label: String,
    pub running: bool,
    pub exit_code: Option<i32>,
    pub log: Vec<ProcLine>,
    pub started_at: f64,
}

#[derive(Debug, Clone)]
pub struct ProcLine {
    pub line: String,
    pub is_err: bool,
}

#[derive(Default)]
pub struct ProcessRegistry {
    pub entries: HashMap<String, ProcEntry>,
    pub order: Vec<String>,
    pub max_log_lines: usize,
    pub selected: usize,
}

impl ProcessRegistry {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            order: Vec::new(),
            max_log_lines: 500,
            selected: 0,
        }
    }

    pub fn upsert_started(&mut self, id: &str, label: &str) {
        if !self.entries.contains_key(id) {
            self.order.push(id.to_string());
        }
        self.entries.insert(
            id.to_string(),
            ProcEntry {
                id: id.to_string(),
                label: label.to_string(),
                running: true,
                exit_code: None,
                log: Vec::new(),
                started_at: chrono::Local::now().timestamp() as f64,
            },
        );
    }

    pub fn append_line(&mut self, id: &str, line: String, is_err: bool) {
        if let Some(entry) = self.entries.get_mut(id) {
            if entry.log.len() >= self.max_log_lines {
                entry.log.remove(0);
            }
            entry.log.push(ProcLine { line, is_err });
        }
    }

    pub fn mark_exited(&mut self, id: &str, code: Option<i32>) {
        if let Some(entry) = self.entries.get_mut(id) {
            entry.running = false;
            entry.exit_code = code;
        }
    }

    pub fn selected_entry(&self) -> Option<&ProcEntry> {
        let id = self.order.get(self.selected)?;
        self.entries.get(id)
    }

    pub fn select_next(&mut self) {
        if !self.order.is_empty() {
            self.selected = (self.selected + 1) % self.order.len();
        }
    }

    pub fn select_prev(&mut self) {
        if !self.order.is_empty() {
            self.selected = if self.selected == 0 {
                self.order.len() - 1
            } else {
                self.selected - 1
            };
        }
    }

    pub fn active_count(&self) -> usize {
        self.entries.values().filter(|e| e.running).count()
    }
}

#[derive(Clone, Default)]
pub struct ProcessManager {
    children: Arc<AsyncMutex<HashMap<String, Child>>>,
    counter: Arc<Mutex<u64>>,
}

impl ProcessManager {
    pub fn new() -> Self {
        Self {
            children: Arc::new(AsyncMutex::new(HashMap::new())),
            counter: Arc::new(Mutex::new(0)),
        }
    }

    fn next_id(&self, prefix: &str) -> String {
        let mut c = self.counter.lock().unwrap();
        *c += 1;
        format!("{}-{}", prefix, *c)
    }

    pub fn spawn_streaming(
        &self,
        label: &str,
        program: &str,
        args: &[&str],
        cwd: Option<&str>,
        tx: EventSender,
    ) -> String {
        let id = self.next_id(label);
        let id_clone = id.clone();
        let label = label.to_string();
        let program = program.to_string();
        let args: Vec<String> = args.iter().map(|s| s.to_string()).collect();
        let cwd = cwd.map(|s| s.to_string());
        let children = self.children.clone();

        let _ = tx.send(Event::ProcStarted {
            id: id.clone(),
            label: label.clone(),
        });

        tokio::spawn(async move {
            let mut cmd = Command::new(&program);
            cmd.args(&args);
            cmd.stdout(Stdio::piped());
            cmd.stderr(Stdio::piped());
            cmd.stdin(Stdio::null());
            if let Some(d) = &cwd {
                cmd.current_dir(d);
            }

            let mut child = match cmd.spawn() {
                Ok(c) => c,
                Err(e) => {
                    let _ = tx.send(Event::ProcLine {
                        id: id_clone.clone(),
                        line: format!("[spawn error] {}", e),
                        is_err: true,
                    });
                    let _ = tx.send(Event::ProcExited {
                        id: id_clone,
                        code: Some(-1),
                    });
                    return;
                }
            };

            let stdout = child.stdout.take();
            let stderr = child.stderr.take();

            children.lock().await.insert(id_clone.clone(), child);

            let tx_out = tx.clone();
            let id_out = id_clone.clone();
            let stdout_task = tokio::spawn(async move {
                if let Some(out) = stdout {
                    let mut reader = BufReader::new(out).lines();
                    while let Ok(Some(line)) = reader.next_line().await {
                        let _ = tx_out.send(Event::ProcLine {
                            id: id_out.clone(),
                            line,
                            is_err: false,
                        });
                    }
                }
            });

            let tx_err = tx.clone();
            let id_err = id_clone.clone();
            let stderr_task = tokio::spawn(async move {
                if let Some(err) = stderr {
                    let mut reader = BufReader::new(err).lines();
                    while let Ok(Some(line)) = reader.next_line().await {
                        let _ = tx_err.send(Event::ProcLine {
                            id: id_err.clone(),
                            line,
                            is_err: true,
                        });
                    }
                }
            });

            let _ = stdout_task.await;
            let _ = stderr_task.await;

            let exit_code = {
                let mut guard = children.lock().await;
                if let Some(mut child) = guard.remove(&id_clone) {
                    match child.wait().await {
                        Ok(status) => status.code(),
                        Err(_) => None,
                    }
                } else {
                    None
                }
            };

            let _ = tx.send(Event::ProcExited {
                id: id_clone,
                code: exit_code,
            });
        });

        id
    }

    pub async fn kill(&self, id: &str) {
        let mut guard = self.children.lock().await;
        if let Some(child) = guard.get_mut(id) {
            let _ = child.start_kill();
        }
    }

    pub async fn kill_all(&self) {
        let mut guard = self.children.lock().await;
        for (_, child) in guard.iter_mut() {
            let _ = child.start_kill();
        }
    }
}
