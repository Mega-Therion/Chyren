use crate::adccl_logic::ADCCL;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Verify an LLM response against a task, returning a JSON string.
///
/// # Safety
/// `response_ptr` and `task_ptr` must be valid, NUL-terminated C strings for the duration
/// of this call. The returned pointer must be released by calling [`free_string`].
#[no_mangle]
pub unsafe extern "C" fn verify_response(
    response_ptr: *const c_char,
    task_ptr: *const c_char,
) -> *mut c_char {
    if response_ptr.is_null() || task_ptr.is_null() {
        return CString::new("error: null pointer").unwrap().into_raw();
    }
    let response = CStr::from_ptr(response_ptr).to_string_lossy();
    let task = CStr::from_ptr(task_ptr).to_string_lossy();

    let adccl = ADCCL::new(0.1, None);
    let result = adccl.verify(&response, &task);

    let json_result = serde_json::to_string(&result).expect("Failed to serialize");
    CString::new(json_result)
        .expect("Failed to create CString")
        .into_raw()
}

/// Free a C string allocated by [`verify_response`].
///
/// # Safety
/// `s` must be a pointer previously returned by [`verify_response`]. It must not be used
/// after this function returns, and it must not be freed more than once.
#[no_mangle]
pub unsafe extern "C" fn free_string(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    let _ = CString::from_raw(s);
}
