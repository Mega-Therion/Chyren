pub mod adccl_logic;
pub mod ffi;

#[cfg(test)]
mod tests {
    use super::adccl_logic::ADCCL;

    #[test]
    fn test_adccl_verification() {
        let adccl = ADCCL::new(0.5, None);
        let result = adccl.verify("This is a test response", "Test task");
        assert!(result.passed);
        
        let result_stub = adccl.verify("TODO: Finish this", "Test task");
        assert!(!result_stub.passed);
        assert!(result_stub.flags.contains(&"STUB_MARKERS_DETECTED".to_string()));
    }
}
