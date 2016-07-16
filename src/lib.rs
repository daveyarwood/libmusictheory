extern crate libc;
extern crate regex;

mod note;

use note::Note;
use std::ffi::CStr;

#[no_mangle]
pub unsafe extern fn musictheory_note(note: *mut libc::c_char) -> isize {
    let s = CStr::from_ptr(note).to_str().unwrap_or("");
    match Note::from_str(s) {
        Err(_)   => 42, // TODO: throw an exception or something
        Ok(note) => note.number()
    }
}
