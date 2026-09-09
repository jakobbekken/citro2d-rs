#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all)]
#![allow(unexpected_cfgs)]
#![allow(unnecessary_transmutes)]

mod bindings {
    use libc::*;

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::*;

pub mod result;
pub use result::*;

pub unsafe fn errno() -> s32 {
    unsafe { *__errno() }
}
