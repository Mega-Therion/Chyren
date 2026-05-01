//! Bridge to `scripts/formal_verification.py`.
//!
//! The Z3-backed skill verifier lives in Python (Z3's first-class binding is
//! Python; a pure-Rust port would mean shipping a SAT solver). This module
//! invokes the Python script as a subprocess, parses its JSON report, and
//! returns a typed [`SkillVerification`].
//!
//! The Conductor calls this at skill-admission time inside the Dynamic
//! Skill-Upgrade Sandbox (DSUS): a new `.skill` spec is rejected unless the
//! verifier returns `verified: true`.

use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::Command;
use thiserror::Error;

/// Path to the Python verifier, relative to the repo root or absolute.
/// Override at runtime with `CHYREN_FORMAL_VERIFIER_PATH`.
pub const DEFAULT_VERIFIER_PATH: &str = "scripts/formal_verification.py";

/// Parsed report emitted by `formal_verification.py`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SkillVerification {
    /// Skill name from the spec.
    pub skill: String,
    /// True when both `pre_satisfiable` and `post_holds` are true.
    pub verified: bool,
    /// Whether the precondition set admits at least one input.
    pub pre_satisfiable: bool,
    /// Whether the post-condition holds for every admitted input.
    pub post_holds: bool,
    /// Z3 model violating the post-condition, when one was produced.
    #[serde(default)]
    pub counterexample: Option<serde_json::Value>,
    /// Free-form notes from the verifier.
    #[serde(default)]
    pub notes: Vec<String>,
}

/// Errors when invoking the verifier.
#[derive(Debug, Error)]
pub enum VerifierError {
    /// Subprocess could not be spawned.
    #[error("verifier spawn failed: {0}")]
    Spawn(String),
    /// Verifier exited with a non-zero status that is *not* `1` (a 1 status
    /// means "not verified" — that's a normal `Ok(SkillVerification)` with
    /// `verified=false`, not an error).
    #[error("verifier returned status {0}: {1}")]
    BadStatus(i32, String),
    /// Verifier stdout could not be parsed as JSON.
    #[error("verifier output parse error: {0}")]
    Parse(String),
}

/// Run `formal_verification.py <skill_path>` and return the parsed report.
///
/// `python` defaults to `python3`. Set `CHYREN_PYTHON_BIN` to override (e.g.
/// when running inside a venv).
pub fn verify_skill_file(skill_path: &Path) -> Result<SkillVerification, VerifierError> {
    let py = std::env::var("CHYREN_PYTHON_BIN").unwrap_or_else(|_| "python3".to_string());
    let verifier = std::env::var("CHYREN_FORMAL_VERIFIER_PATH")
        .unwrap_or_else(|_| DEFAULT_VERIFIER_PATH.to_string());

    let output = Command::new(&py)
        .arg(&verifier)
        .arg(skill_path)
        .output()
        .map_err(|e| VerifierError::Spawn(e.to_string()))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let code = output.status.code().unwrap_or(-1);

    // Status 0 = verified, 1 = unverified (still a parseable JSON report),
    // 2 (or anything else) = real error (missing file, bad spec, etc).
    if code != 0 && code != 1 {
        return Err(VerifierError::BadStatus(code, stderr.to_string()));
    }

    serde_json::from_str::<SkillVerification>(&stdout).map_err(|e| {
        VerifierError::Parse(format!(
            "{e}; stdout={}; stderr={}",
            stdout.trim(),
            stderr.trim()
        ))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Smoke test: only runs when CHYREN_RUN_PYTHON_TESTS=1 is set, since CI
    /// may lack z3-solver. Locally: `CHYREN_RUN_PYTHON_TESTS=1 cargo test
    /// -p chyren-aegis skill_verifier`.
    #[test]
    fn verify_minimal_skill() {
        if std::env::var("CHYREN_RUN_PYTHON_TESTS").unwrap_or_default() != "1" {
            return;
        }
        let dir = std::env::temp_dir().join("chyren-skill-test");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("trivial.skill.json");
        std::fs::write(
            &path,
            r#"{
                "name": "trivial",
                "vars": [{"name": "x", "type": "Int"}],
                "pre": ["x > 0"],
                "post": ["x > -1"],
                "result": {"name": "result", "type": "Int"}
            }"#,
        )
        .unwrap();
        let result = verify_skill_file(&path).expect("verifier ran");
        assert_eq!(result.skill, "trivial");
        assert!(result.verified);
    }
}
