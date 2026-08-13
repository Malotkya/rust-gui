#![allow(dead_code, unsafe_op_in_unsafe_fn)]
#![cfg_attr(debug_assertions, deny(missing_debug_implementations))]

mod application_info;
pub use application_info::*;
pub mod data;
pub mod render;
