#![allow(unused)]

pub mod fibonacci;
pub mod binary;

pub mod graph;

#[derive(Debug)]
pub struct Entry<K, T> {
    pub key: K,
    pub aux: T,
}

impl<K, T> Entry<K, T> {
    pub fn new(key: K, aux: T) -> Self {
        Self { key, aux }
    }
}

