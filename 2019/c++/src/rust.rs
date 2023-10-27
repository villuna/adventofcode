#![no_std]
#![feature(lang_items, panic_info_message)]
extern crate alloc;

pub mod day3;
pub use day3::*;

use core::panic::PanicInfo;
use core::str::{from_utf8, Utf8Error};
use alloc::alloc::{Layout, GlobalAlloc};


#[lang = "eh_personality"]
pub extern "C" fn rust_eh_personality() {
}

#[link(name = "c")]
extern "C" {
    fn malloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
    fn exit(code: i32) -> !;
    fn strlen(string: *const u8) -> usize;
}

extern "C" { 
    fn handle_panic(message: *const u8, message_len: usize, file: *const u8, file_len: usize, line: u32); 
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let (msg, msg_len) = match info.message().and_then(|m| m.as_str()) {
        Some(string) => (string.as_ptr() as *const u8, string.len()),
        None => (0 as *const u8, 0),
    };

    let (file, file_len, line) = match info.location() {
        Some(loc) => (loc.file().as_ptr() as *const u8, loc.file().len(), loc.line()),
        None => (0 as *const u8, 0, 0),
    };

    unsafe { handle_panic(msg, msg_len, file, file_len, line); }

    unreachable!();
}

pub unsafe fn to_rust_str<'a>(string: *const u8) -> Result<&'a str, Utf8Error> {
    let len = strlen(string);
    let slice = core::ptr::slice_from_raw_parts(string, len);
    let slice = &*slice;

    from_utf8(slice)
}

pub struct CAlloc;

unsafe impl GlobalAlloc for CAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        malloc(layout.size())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free(ptr);
    }
}

#[global_allocator]
pub static CALLOC: CAlloc = CAlloc;
