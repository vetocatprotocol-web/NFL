//! Low-level NFL segment and header layout definitions.

use std::convert::TryFrom;

use byteorder::{ByteOrder, LittleEndian};

use nfl_core::{align_up, is_aligned, Error, Result, SEGMENT_ALIGNMENT};

/// NFL format magic bytes anchored at the file start.
pub const NFL_MAGIC: [u8; 8] = *b"NFLFMT01";

/// Current on-disk format version.
pub const CURRENT_FORMAT_VERSION: u32 = 1;

/// Header size in bytes for the compact NFL file header.
pub const HEADER_SIZE: usize = std::mem::size_of::<Header>();

/// A fixed-length file header placed at the beginning of every NFL file.
///
/// The header itself is located at offset 0 and the segment directory starts
/// at a 4K-aligned offset after the header region.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Header {
    magic: [u8; 8],
    version: u32,
    header_size: u32,
    segment_directory_offset: u64,
    segment_directory_length: u64,
    segment_count: u32,
    reserved: u32,
    checksum: u64,
}

impl Header {
    /// Build a new header for the given directory layout.
    pub fn new(directory_offset: u64, directory_length: u64, segment_count: u32) -> Self {
        let mut header = Self {
            magic: NFL_MAGIC,
            version: CURRENT_FORMAT_VERSION,
            header_size: HEADER_SIZE as u32,
            segment_directory_offset: directory_offset,
            segment_directory_length: directory_length,
            segment_count,
            reserved: 0,
            checksum: 0,
        };
        header.checksum = header.compute_checksum();
        header
    }

    /// Parse a header in-place from a byte slice.
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < HEADER_SIZE {
            return Err(Error::InsufficientData);
        }

        let mut header = Self {
            magic: bytes[0..8].try_into().unwrap(),
            version: LittleEndian::read_u32(&bytes[8..12]),
            header_size: LittleEndian::read_u32(&bytes[12..16]),
            segment_directory_offset: LittleEndian::read_u64(&bytes[16..24]),
            segment_directory_length: LittleEndian::read_u64(&bytes[24..32]),
            segment_count: LittleEndian::read_u32(&bytes[32..36]),
            reserved: LittleEndian::read_u32(&bytes[36..40]),
            checksum: LittleEndian::read_u64(&bytes[40..48]),
        };

        if header.magic != NFL_MAGIC {
            return Err(Error::InvalidMagic);
        }

        if header.version != CURRENT_FORMAT_VERSION {
            return Err(Error::UnsupportedVersion(header.version));
        }

        if header.header_size as usize != HEADER_SIZE {
            return Err(Error::Corrupted("unexpected header size"));
        }

        if !is_aligned(header.segment_directory_offset, SEGMENT_ALIGNMENT as u64) {
            return Err(Error::MisalignedSegment {
                offset: header.segment_directory_offset,
                alignment: SEGMENT_ALIGNMENT as u64,
            });
        }

        let actual_checksum = header.compute_checksum();
        if actual_checksum != header.checksum {
            return Err(Error::ChecksumMismatch {
                expected: header.checksum,
                actual: actual_checksum,
            });
        }

        Ok(header)
    }

    /// Serialize the header into a fixed-size output buffer.
    pub fn write_to(&self, out: &mut [u8]) -> Result<()> {
        if out.len() < HEADER_SIZE {
            return Err(Error::InsufficientData);
        }

        out[0..8].copy_from_slice(&self.magic);
        LittleEndian::write_u32(&mut out[8..12], self.version);
        LittleEndian::write_u32(&mut out[12..16], self.header_size);
        LittleEndian::write_u64(&mut out[16..24], self.segment_directory_offset);
        LittleEndian::write_u64(&mut out[24..32], self.segment_directory_length);
        LittleEndian::write_u32(&mut out[32..36], self.segment_count);
        LittleEndian::write_u32(&mut out[36..40], self.reserved);
        LittleEndian::write_u64(&mut out[40..48], self.checksum);
        Ok(())
    }

    /// The offset where the segment directory begins.
    pub fn segment_directory_offset(&self) -> u64 {
        self.segment_directory_offset
    }

    /// The segment directory length in bytes.
    pub fn segment_directory_length(&self) -> u64 {
        self.segment_directory_length
    }

    /// The number of segment descriptors in the directory.
    pub fn segment_count(&self) -> u32 {
        self.segment_count
    }

    /// Compute the header checksum with the checksum field zeroed.
    pub fn checksum(&self) -> u64 {
        let mut temporary = *self;
        temporary.checksum = 0;
        temporary.compute_checksum()
    }

    fn compute_checksum(&self) -> u64 {
        let mut buffer = [0u8; HEADER_SIZE];
        let mut temporary = *self;
        temporary.checksum = 0;
        temporary.write_to(&mut buffer).expect("header buffer is sufficient");
        let mut accumulator = 0u64;
        for chunk in buffer[..40].chunks_exact(8) {
            accumulator = accumulator.wrapping_add(LittleEndian::read_u64(chunk));
        }

        let remainder = &buffer[40..40 + (HEADER_SIZE - 40)];
        if !remainder.is_empty() {
            let mut last = [0u8; 8];
            last[..remainder.len()].copy_from_slice(remainder);
            accumulator = accumulator.wrapping_add(LittleEndian::read_u64(&last));
        }

        accumulator.rotate_left(13).wrapping_add(0x9e3779b97f4a7c15)
    }
}

/// Segment kinds used in NFL's immutable file layout.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum SegmentKind {
    Schema = 1,
    Data = 2,
    Index = 3,
    Mvcc = 4,
    Footer = 5,
    Custom = 0x8000,
}

impl From<u16> for SegmentKind {
    fn from(value: u16) -> Self {
        match value {
            1 => SegmentKind::Schema,
            2 => SegmentKind::Data,
            3 => SegmentKind::Index,
            4 => SegmentKind::Mvcc,
            5 => SegmentKind::Footer,
            _ => SegmentKind::Custom,
        }
    }
}

/// Metadata for a single segment in the NFL file.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SegmentDescriptor {
    kind: u16,
    flags: u16,
    reserved: u32,
    offset: u64,
    length: u64,
    checksum: u64,
}

impl SegmentDescriptor {
    /// Number of bytes used by a descriptor on disk.
    pub const SIZE: usize = std::mem::size_of::<SegmentDescriptor>();

    /// Build a new descriptor with an explicit kind tag and checksum.
    pub fn new(kind: SegmentKind, flags: u16, offset: u64, length: u64, checksum: u64) -> Self {
        Self {
            kind: kind as u16,
            flags,
            reserved: 0,
            offset,
            length,
            checksum,
        }
    }

    /// Parse a descriptor from a byte slice.
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < Self::SIZE {
            return Err(Error::InsufficientData);
        }

        Ok(Self {
            kind: LittleEndian::read_u16(&bytes[0..2]),
            flags: LittleEndian::read_u16(&bytes[2..4]),
            reserved: LittleEndian::read_u32(&bytes[4..8]),
            offset: LittleEndian::read_u64(&bytes[8..16]),
            length: LittleEndian::read_u64(&bytes[16..24]),
            checksum: LittleEndian::read_u64(&bytes[24..32]),
        })
    }

    /// Serialize the descriptor to a byte buffer.
    pub fn write_to(&self, out: &mut [u8]) -> Result<()> {
        if out.len() < Self::SIZE {
            return Err(Error::InsufficientData);
        }

        LittleEndian::write_u16(&mut out[0..2], self.kind);
        LittleEndian::write_u16(&mut out[2..4], self.flags);
        LittleEndian::write_u32(&mut out[4..8], self.reserved);
        LittleEndian::write_u64(&mut out[8..16], self.offset);
        LittleEndian::write_u64(&mut out[16..24], self.length);
        LittleEndian::write_u64(&mut out[24..32], self.checksum);
        Ok(())
    }

    /// The segment offset in the file.
    pub fn offset(&self) -> u64 {
        self.offset
    }

    /// The segment length in bytes.
    pub fn length(&self) -> u64 {
        self.length
    }

    /// The descriptor checksum, if present.
    pub fn checksum(&self) -> u64 {
        self.checksum
    }

    /// Validate that the segment descriptor obeys NFL alignment and bounds checks.
    pub fn validate(&self, file_size: u64) -> Result<()> {
        if !is_aligned(self.offset, SEGMENT_ALIGNMENT as u64) {
            return Err(Error::MisalignedSegment {
                offset: self.offset,
                alignment: SEGMENT_ALIGNMENT as u64,
            });
        }

        if self.offset.checked_add(self.length).map_or(true, |end| end > file_size) {
            return Err(Error::Corrupted("segment extends past file bounds"));
        }

        Ok(())
    }

    /// Interpret the stored kind tag.
    pub fn kind(&self) -> SegmentKind {
        SegmentKind::from(self.kind)
    }
}

/// A compact, typed segment directory for immutable NFL files.
pub struct SegmentDirectory {
    pub entries: Vec<SegmentDescriptor>,
}

impl SegmentDirectory {
    /// Parse a directory from raw bytes.
    pub fn parse(mut bytes: &[u8], file_size: u64) -> Result<Self> {
        let mut entries = Vec::new();
        while !bytes.is_empty() {
            if bytes.len() < SegmentDescriptor::SIZE {
                return Err(Error::InvalidSegmentDirectory);
            }

            let descriptor = SegmentDescriptor::parse(&bytes[..SegmentDescriptor::SIZE])?;
            descriptor.validate(file_size)?;
            entries.push(descriptor);
            bytes = &bytes[SegmentDescriptor::SIZE..];
        }

        Ok(Self { entries })
    }

    /// Serialize the directory into the provided buffer.
    pub fn write_to(&self, out: &mut [u8]) -> Result<()> {
        let required = self.entries.len() * SegmentDescriptor::SIZE;
        if out.len() < required {
            return Err(Error::InsufficientData);
        }

        for (i, entry) in self.entries.iter().enumerate() {
            let start = i * SegmentDescriptor::SIZE;
            entry.write_to(&mut out[start..start + SegmentDescriptor::SIZE])?;
        }
        Ok(())
    }

    /// Layout the directory using 4K segment alignment.
    pub fn layout(entries: &[SegmentDescriptor]) -> (u64, u64) {
        let directory_size = (entries.len() * SegmentDescriptor::SIZE) as u64;
        let directory_offset = align_up(HEADER_SIZE as u64, SEGMENT_ALIGNMENT as u64);
        (directory_offset, directory_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_round_trip() {
        let entries = [SegmentDescriptor::new(SegmentKind::Schema, 0, 4096, 1024, 0x1234_5678_9abc_def0)];
        let (directory_offset, directory_length) = SegmentDirectory::layout(&entries);
        let header = Header::new(directory_offset, directory_length, entries.len() as u32);

        let mut buffer = [0u8; HEADER_SIZE];
        header.write_to(&mut buffer).expect("write header");
        let parsed = Header::parse(&buffer).expect("parse header");

        assert_eq!(parsed, header);
        assert_eq!(parsed.segment_directory_offset, directory_offset);
    }

    #[test]
    fn segment_directory_parse_and_validate() {
        let descriptor = SegmentDescriptor::new(SegmentKind::Data, 0, 8192, 2048, 0xdead_beef_cafe_babe);
        let mut buffer = vec![0u8; SegmentDescriptor::SIZE];
        descriptor.write_to(&mut buffer).expect("write descriptor");

        let directory = SegmentDirectory::parse(&buffer, 32_768).expect("parse directory");
        assert_eq!(directory.entries.len(), 1);
        assert_eq!(directory.entries[0], descriptor);
    }

    #[test]
    fn reject_invalid_magic() {
        let mut buffer = [0u8; HEADER_SIZE];
        buffer[0..8].copy_from_slice(b"BADMAGIC");
        assert!(matches!(Header::parse(&buffer), Err(Error::InvalidMagic)));
    }
}
