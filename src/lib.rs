#![feature(test)]
#![feature(core_slice_ext)]
#![feature(universal_impl_trait)]
#![feature(conservative_impl_trait)]
#![feature(integer_atomics)]
#![feature(entry_and_modify)]
#![feature(vec_resize_default)]

extern crate core;
extern crate test;

pub mod dictionary;
pub mod bow;
pub mod wordvector;
pub mod utils;

mod matrix;
mod testing;

pub struct WordVector<'a> {
    model: &'a WordVectorModel,
    distance: &'a WordVectorDistance,
}

pub trait WordVectorModel {
    fn word_index(&self, word: &str) -> Option<i64>;
    fn word_to_vector(&self, word: &str) -> Option<Vec<f32>>;
    fn sentence_to_vector(&self, text: &str) -> Option<Vec<f32>>;
}

pub trait WordVectorDistance {
    fn calc(&self, doc_bow1: &[f32], doc_bow2: &[f32], distance_matrix: &[f32]) -> f32;
}
