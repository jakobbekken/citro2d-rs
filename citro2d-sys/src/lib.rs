#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(unused_imports)]
#![allow(clippy::all)]

pub use citro3d_sys::*;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
