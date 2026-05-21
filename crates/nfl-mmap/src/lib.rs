//! NFL mmap-first access and zero-copy segment reader.
#![deny(missing_docs)]

pub mod mmap;

pub use mmap::{NflFile, SegmentView};
