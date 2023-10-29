#![no_std]
#![allow(internal_features)]
#![feature(lang_items, panic_info_message)]
extern crate alloc;

pub mod day3;
pub mod day8;
pub use day3::*;
pub use day8::*;

use core::panic::PanicInfo;
use core::str::{from_utf8, Utf8Error};
use alloc::alloc::{Layout, GlobalAlloc};
use core::fmt::Write;


#[lang = "eh_personality"]
pub extern "C" fn rust_eh_personality() {
}

#[link(name = "c")]
extern "C" {
    fn malloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
    fn strlen(string: *const u8) -> usize;
}

extern "C" { 
    fn handle_panic(message: *const u8, message_len: usize, file: *const u8, file_len: usize, line: u32); 
    fn print_rust_str(string: *const u8, string_len: usize);
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

pub struct CWriter;

impl Write for CWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            print_rust_str(s.as_ptr(), s.len());
        }
        Ok(())
    }
}

pub static mut WRITER: CWriter = CWriter;

pub fn _print(args: core::fmt::Arguments) {
    unsafe {
        WRITER.write_fmt(args).unwrap();
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
