#![feature(test)]
#![feature(core_slice_ext)]
#![feature(universal_impl_trait)]
#![feature(conservative_impl_trait)]
#![feature(integer_atomics)]
#![feature(entry_and_modify)]
#![feature(vec_resize_default)]

extern crate core;
extern crate test;
extern crate rand;

pub mod dictionary;
pub mod bow;
pub mod wordvector;
pub mod utils;

mod matrix;
mod testing;
