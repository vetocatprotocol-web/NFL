//! Core NFL primitives and shared low-level utilities.
#![deny(missing_docs)]
#![deny(unsafe_code)]

use thiserror::Error;

/// A canonical result type for NFL library crates.
pub type Result<T> = std::result::Result<T, Error>;

/// NFL on-disk alignment for segment boundaries and mmap regions.
pub const SEGMENT_ALIGNMENT: usize = 4096;

/// Align `value` up to `alignment`.
#[inline]
pub const fn align_up(value: u64, alignment: u64) -> u64 {
    let mask = alignment - 1;
    (value + mask) & !mask
}

/// Returns true when `value` is a multiple of `alignment`.
#[inline]
pub const fn is_aligned(value: u64, alignment: u64) -> bool {
    alignment != 0 && (value & (alignment - 1)) == 0
}

/// NFL core error domain for format validation and storage primitives.
#[derive(Debug, Error)]
pub enum Error {
    /// The file header contains an unexpected magic value.
    #[error("invalid NFL magic value")]
    InvalidMagic,

    /// The file format version is not supported.
    #[error("unsupported NFL format version {0}")]
    UnsupportedVersion(u32),

    /// The buffer is too short to contain the requested structure.
    #[error("insufficient data for expected structure")]
    InsufficientData,

    /// A segment offset or directory entry does not satisfy the alignment policy.
    #[error("misaligned segment at offset {offset}, expected alignment {alignment}")]
    MisalignedSegment { offset: u64, alignment: u64 },

    /// Data integrity validation failed.
    #[error("checksum mismatch: expected {expected:#x}, actual {actual:#x}")]
    ChecksumMismatch { expected: u64, actual: u64 },

    /// A segment directory or header failed structural validation.
    #[error("invalid segment directory")]
    InvalidSegmentDirectory,

    /// Underlying IO failed.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// Generic corruption detected inside NFL metadata.
    #[error("corrupted NFL data: {0}")]
    Corrupted(&'static str),
}
