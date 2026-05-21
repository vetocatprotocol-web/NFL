//! NFL file format and segment metadata definitions.
#![deny(missing_docs)]
#![deny(unsafe_code)]

pub mod layout;

pub use layout::{Header, SegmentDescriptor, SegmentDirectory, SegmentKind};
