//! chyren-mega: MEGA cloud storage integration for Chyren via MEGAcmd subprocess bridge.
//!
//! All operations delegate to the locally-installed `mega-*` binaries (MEGAcmd daemon).
//! The daemon must be running and authenticated (`mega-login`) before use.

use serde::{Deserialize, Serialize};
use std::process::Command;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MegaError {
    #[error("MEGAcmd not found — install MEGAcmd and ensure it is in PATH")]
    NotInstalled,
    #[error("Not authenticated — run `mega-login` first")]
    NotAuthenticated,
    #[error("MEGA command failed: {0}")]
    CommandFailed(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type MegaResult<T> = Result<T, MegaError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MegaEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferStatus {
    pub tag: String,
    pub source: String,
    pub destination: String,
    pub progress_pct: f32,
    pub state: String,
}

/// Run a mega-* command and return stdout, mapping auth/missing errors.
fn run(bin: &str, args: &[&str]) -> MegaResult<String> {
    let output = Command::new(bin).args(args).output().map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            MegaError::NotInstalled
        } else {
            MegaError::Io(e)
        }
    })?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if !output.status.success() {
        let msg = if stderr.contains("Not logged in") || stdout.contains("Not logged in") {
            return Err(MegaError::NotAuthenticated);
        } else {
            format!("{} {}", stderr.trim(), stdout.trim())
        };
        return Err(MegaError::CommandFailed(msg));
    }

    Ok(stdout)
}

/// Returns the authenticated account email, or an error if not logged in.
pub fn whoami() -> MegaResult<String> {
    let out = run("mega-whoami", &[])?;
    let email = out
        .lines()
        .find(|l| l.contains("Account e-mail:"))
        .map(|l| l.replace("Account e-mail:", "").trim().to_string())
        .unwrap_or_else(|| out.trim().to_string());
    Ok(email)
}

/// List entries at a MEGA cloud path.
pub fn ls(remote_path: &str) -> MegaResult<Vec<MegaEntry>> {
    let out = run("mega-ls", &["-l", remote_path])?;
    let entries = out
        .lines()
        .filter(|l| !l.trim().is_empty() && !l.starts_with("INFO"))
        .map(|l| {
            let is_dir = l.trim_start().starts_with('d');
            let name = l.split_whitespace().last().unwrap_or("").to_string();
            let path = format!("{}/{}", remote_path.trim_end_matches('/'), name);
            MegaEntry { name, path, is_dir }
        })
        .collect();
    Ok(entries)
}

/// Create a remote directory (and parents with -p).
pub fn mkdir(remote_path: &str) -> MegaResult<()> {
    run("mega-mkdir", &["-p", remote_path])?;
    Ok(())
}

/// Upload a local file or directory to a MEGA cloud path.
/// Uses -q (quiet) flag; large uploads run via the MEGAcmd daemon queue.
pub fn upload(local_path: &str, remote_path: &str) -> MegaResult<()> {
    run("mega-put", &["-q", local_path, remote_path])?;
    Ok(())
}

/// Download a file or directory from MEGA to a local path.
pub fn download(remote_path: &str, local_path: &str) -> MegaResult<()> {
    run("mega-get", &[remote_path, local_path])?;
    Ok(())
}

/// Delete a remote path. Set recursive=true for directories.
pub fn remove(remote_path: &str, recursive: bool) -> MegaResult<()> {
    if recursive {
        run("mega-rm", &["-r", "-f", remote_path])?;
    } else {
        run("mega-rm", &["-f", remote_path])?;
    }
    Ok(())
}

/// Move/rename a remote path.
pub fn mv(src: &str, dst: &str) -> MegaResult<()> {
    run("mega-mv", &[src, dst])?;
    Ok(())
}

/// Copy a remote path.
pub fn cp(src: &str, dst: &str) -> MegaResult<()> {
    run("mega-cp", &[src, dst])?;
    Ok(())
}

/// Set up a persistent two-way sync between a local folder and a remote path.
pub fn sync(local_path: &str, remote_path: &str) -> MegaResult<()> {
    run("mega-sync", &[local_path, remote_path])?;
    Ok(())
}

/// List all active syncs.
pub fn list_syncs() -> MegaResult<String> {
    run("mega-sync", &[])
}

/// Show active transfer queue.
pub fn transfers() -> MegaResult<String> {
    run("mega-transfers", &[])
}

/// MEGA cloud disk usage.
pub fn disk_usage(remote_path: &str) -> MegaResult<String> {
    run("mega-du", &[remote_path])
}

/// Export a public share link for a remote path.
pub fn export(remote_path: &str) -> MegaResult<String> {
    run("mega-export", &["-a", remote_path])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whoami_returns_email_or_auth_error() {
        match whoami() {
            Ok(email) => assert!(email.contains('@'), "expected email, got: {email}"),
            Err(MegaError::NotAuthenticated) => {} // acceptable in CI
            Err(MegaError::NotInstalled) => {}     // acceptable in CI
            Err(e) => panic!("unexpected error: {e}"),
        }
    }

    #[test]
    fn ls_root_or_auth_error() {
        match ls("/") {
            Ok(_) => {}
            Err(MegaError::NotAuthenticated) => {}
            Err(MegaError::NotInstalled) => {}
            Err(e) => panic!("unexpected error: {e}"),
        }
    }
}
