use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub unsafe extern fn music_note(note: *mut c_char) -> usize {
    match CStr::from_ptr(note).to_str() {
        Ok(s)  => s.to_string().len(),
        Err(_) => 42 // TODO: throw an exception or something
    }
}