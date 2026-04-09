use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use crate::ADCCL;

#[no_mangle]
pub extern "C" fn verify_response(response_ptr: *const c_char, task_ptr: *const c_char) -> *mut c_char {
    if response_ptr.is_null() || task_ptr.is_null() {
        return CString::new("error: null pointer").unwrap().into_raw();
    }
    let response = unsafe { CStr::from_ptr(response_ptr).to_string_lossy() };
    let task = unsafe { CStr::from_ptr(task_ptr).to_string_lossy() };

    let adccl = ADCCL::new(0.1, None);
    let result = adccl.verify(&response, &task);
    
    let json_result = serde_json::to_string(&result).expect("Failed to serialize");
    CString::new(json_result).expect("Failed to create CString").into_raw()
}

#[no_mangle]
pub extern "C" fn free_string(s: *mut c_char) {
    unsafe {
        if s.is_null() { return; }
        let _ = CString::from_raw(s);
    }
}
