//! Memory-mapped NFL file support.

use std::fs::File;
use std::io;
use std::path::Path;

use memmap2::{Mmap, MmapOptions};

use nfl_core::Result;
use nfl_format::layout::{Header, SegmentDescriptor, SegmentDirectory, SegmentKind, HEADER_SIZE};
use nfl_schema::Schema;

/// A zero-copy view into a segment inside an NFL file.
#[derive(Debug)]
pub struct SegmentView<'a> {
    descriptor: SegmentDescriptor,
    data: &'a [u8],
}

impl<'a> SegmentView<'a> {
    /// The descriptor for this segment.
    pub fn descriptor(&self) -> &SegmentDescriptor {
        &self.descriptor
    }

    /// The mapped payload bytes for this segment.
    pub fn data(&self) -> &'a [u8] {
        self.data
    }

    /// The kind of segment.
    pub fn kind(&self) -> SegmentKind {
        self.descriptor.kind()
    }
}

/// A read-only NFL file mapped into memory.
#[derive(Debug)]
pub struct NflFile {
    mmap: Mmap,
    header: Header,
    directory: SegmentDirectory,
}

impl NflFile {
    /// Open an NFL file from disk and memory-map it read-only.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let file = File::open(path.as_ref())?;
        let metadata = file.metadata()?;
        let file_size = metadata.len();

        let mmap = unsafe {
            // Safety: the file is not mutated while the mapping exists.
            MmapOptions::new().map(&file)?
        };

        if mmap.len() < HEADER_SIZE {
            return Err(nfl_core::Error::InsufficientData);
        }

        let header = Header::parse(&mmap[..HEADER_SIZE])?;
        let directory_offset = header.segment_directory_offset() as usize;
        let directory_length = header.segment_directory_length() as usize;

        if directory_offset.checked_add(directory_length).map_or(true, |end| end > mmap.len()) {
            return Err(nfl_core::Error::Corrupted("segment directory extends past file bounds"));
        }

        let directory_bytes = &mmap[directory_offset..directory_offset + directory_length];
        let directory = SegmentDirectory::parse(directory_bytes, file_size)?;

        Ok(Self {
            mmap,
            header,
            directory,
        })
    }

    /// Access the parsed file header.
    pub fn header(&self) -> &Header {
        &self.header
    }

    /// Access the parsed segment directory.
    pub fn directory(&self) -> &SegmentDirectory {
        &self.directory
    }

    /// Return the first segment matching the requested kind.
    pub fn segment_by_kind(&self, kind: SegmentKind) -> Result<Option<SegmentView<'_>>> {
        for descriptor in &self.directory.entries {
            if descriptor.kind() == kind {
                return self.segment_by_descriptor(descriptor).map(Some);
            }
        }
        Ok(None)
    }

    /// Read the schema segment if present and parse it.
    pub fn schema(&self) -> Result<Option<Schema>> {
        if let Some(segment) = self.segment_by_kind(SegmentKind::Schema)? {
            Ok(Some(Schema::parse(segment.data())?))
        } else {
            Ok(None)
        }
    }

    /// Return a segment view by descriptor index.
    pub fn segment_by_index(&self, index: usize) -> Result<SegmentView<'_>> {
        let descriptor = self
            .directory
            .entries
            .get(index)
            .ok_or_else(|| nfl_core::Error::Corrupted("segment index out of range"))?;
        self.segment_by_descriptor(descriptor)
    }

    fn segment_by_descriptor(&self, descriptor: &SegmentDescriptor) -> Result<SegmentView<'_>> {
        let offset = descriptor.offset() as usize;
        let length = descriptor.length() as usize;
        let end = offset.checked_add(length).ok_or(nfl_core::Error::Corrupted("segment length overflow"))?;

        if end > self.mmap.len() {
            return Err(nfl_core::Error::Corrupted("segment extends past mapped file"));
        }

        // Descriptor validation guarantees the segment is mapped safely.
        let data = &self.mmap[offset..end];

        Ok(SegmentView {
            descriptor: *descriptor,
            data,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{remove_file, File};
    use std::io::Write;
    use std::path::PathBuf;

    use super::*;
    use nfl_format::layout::{SegmentDescriptor, SegmentDirectory, SegmentKind};

    fn make_temp_file(bytes: &[u8]) -> PathBuf {
        let mut path = std::env::temp_dir();
        let filename = format!("nfl_test_{}.bin", std::process::id());
        path.push(filename);
        let mut file = File::create(&path).expect("create temp file");
        file.write_all(bytes).expect("write temp file");
        path
    }

    #[test]
    fn open_and_read_header_directory() {
        let entries = [SegmentDescriptor::new(SegmentKind::Schema, 0, 4096, 64, 0)];
        let (directory_offset, directory_length) = SegmentDirectory::layout(&entries);
        let header = Header::new(directory_offset, directory_length, entries.len() as u32);

        let mut buffer = vec![0u8; directory_offset as usize + directory_length as usize];
        header.write_to(&mut buffer[..HEADER_SIZE]).expect("write header");
        entries[0].write_to(&mut buffer[directory_offset as usize..]).expect("write descriptor");

        let path = make_temp_file(&buffer);
        let nft = NflFile::open(&path).expect("open nfl file");

        assert_eq!(nft.header(), &header);
        assert_eq!(nft.directory().entries.len(), 1);
        assert_eq!(nft.directory().entries[0], entries[0]);

        remove_file(path).expect("remove temp file");
    }

    #[test]
    fn segment_by_kind_returns_none_when_missing() {
        let entries = [SegmentDescriptor::new(SegmentKind::Data, 0, 4096, 64, 0)];
        let (directory_offset, directory_length) = SegmentDirectory::layout(&entries);
        let header = Header::new(directory_offset, directory_length, entries.len() as u32);

        let mut buffer = vec![0u8; directory_offset as usize + directory_length as usize];
        header.write_to(&mut buffer[..HEADER_SIZE]).expect("write header");
        entries[0].write_to(&mut buffer[directory_offset as usize..]).expect("write descriptor");

        let path = make_temp_file(&buffer);
        let nfl = NflFile::open(&path).expect("open nfl file");
        let segment = nfl.segment_by_kind(SegmentKind::Schema).expect("query kind");
        assert!(segment.is_none());

        remove_file(path).expect("remove temp file");
    }

    #[test]
    fn read_schema_segment_from_mapped_file() {
        let schema = nfl_schema::Schema::new(vec![
            nfl_schema::Field::new("id", nfl_schema::FieldType::Int64, false),
            nfl_schema::Field::new("payload", nfl_schema::FieldType::Utf8, true),
        ]);
        let schema_bytes = schema.serialize();
        let segment_offset = 8192;
        let descriptor = SegmentDescriptor::new(SegmentKind::Schema, 0, segment_offset, schema_bytes.len() as u64, 0);
        let entries = [descriptor];
        let (directory_offset, directory_length) = SegmentDirectory::layout(&entries);
        let header = Header::new(directory_offset, directory_length, entries.len() as u32);

        let mut buffer = vec![0u8; segment_offset as usize + schema_bytes.len()];
        header.write_to(&mut buffer[..HEADER_SIZE]).expect("write header");
        entries[0].write_to(&mut buffer[directory_offset as usize..]).expect("write descriptor");
        buffer[descriptor.offset as usize..descriptor.offset as usize + schema_bytes.len()].copy_from_slice(&schema_bytes);

        let path = make_temp_file(&buffer);
        let nfl = NflFile::open(&path).expect("open nfl file");
        let parsed = nfl.schema().expect("schema parse").expect("schema exists");
        assert_eq!(parsed, schema);

        remove_file(path).expect("remove temp file");
    }
}
