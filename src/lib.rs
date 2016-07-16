extern crate libc;
extern crate regex;

mod note;

use note::Note;
use std::ffi::CStr;


/// Given a string describing a note in scientific pitch (e.g. "C#5", "Dbb4",
/// "E0"), returns an integer representing the note as an unbounded MIDI note
/// number.
#[no_mangle]
pub unsafe extern fn musictheory_note(note: *mut libc::c_char) -> isize {
    let s = CStr::from_ptr(note).to_str().unwrap_or("");
    match Note::from_str(s) {
        Err(msg) => panic!(msg),
        Ok(note) => note.number()
    }
}
