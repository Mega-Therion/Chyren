pub struct AegisResult {
    pub passed: bool,
    pub note: String,
}

pub fn check_message(text: &str) -> AegisResult {
    // Simple AEGIS gate: filter out blocked patterns
    if text.contains("admin") {
        return AegisResult { passed: false, note: "Unauthorized command".to_string() };
    }
    AegisResult { passed: true, note: "Verified".to_string() }
}
