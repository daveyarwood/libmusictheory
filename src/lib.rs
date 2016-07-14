extern crate libc;
extern crate regex;

use std::ffi::CStr;

mod note;

#[no_mangle]
pub unsafe extern fn music_note(note: *mut libc::c_char) -> isize {
    match CStr::from_ptr(note).to_str() {
        Ok(s)  => note::from_str(s),
        Err(_) => 42 // TODO: throw an exception or something
    }
}
