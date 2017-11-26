#![feature(test)]
#![feature(core_slice_ext)]
#![feature(universal_impl_trait)]
#![feature(conservative_impl_trait)]
#![feature(integer_atomics)]
#![feature(entry_and_modify)]

extern crate core;
extern crate test;
extern crate rand;
extern crate emd;

pub mod dictionary;
pub mod bow;
pub mod wordvector;
mod matrix;