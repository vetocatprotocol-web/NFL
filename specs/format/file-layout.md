# NFL File Layout

NFL files are designed as a sequence of immutable, independently addressable segments optimized for mmap-first access.

## Binary structure

```
+-----------------+
| FILE HEADER     |  48 bytes
+-----------------+
| padding / gap   | up to 4K alignment
+-----------------+
| SEGMENT DIR     | N * 32 bytes
+-----------------+
| padding / gap   | 4K aligned
+-----------------+
| SCHEMA SEGMENT  |
+-----------------+
| DATA SEGMENT    |
+-----------------+
| INDEX SEGMENT   |
+-----------------+
| MVCC SEGMENT    |
+-----------------+
| FOOTER SEGMENT  |
+-----------------+
```

## Header

The file header is fixed-size and always stored at offset 0. It contains:

- magic bytes `NFLFMT01`
- format version
- segment directory offset and length
- segment count
- reserved space for future flags
- checksum over header fields with the checksum zeroed

### Alignment

- The header is always located at offset 0.
- The segment directory begins at the first 4K-aligned offset after the header.
- Every segment must begin on a 4K boundary.

## Segment directory

The segment directory holds one `SegmentDescriptor` per segment.

- Descriptor size: 32 bytes
- Directory is a contiguous array of descriptors
- Directory length is stored in the header and may be sparse if reserved entries are added later

## Footer

An explicit footer segment is optional but recommended for metadata integrity and fast file discovery.

- Footers must be declared via a directory entry of kind `Footer`
- Footer content may include summary checksums, schema fingerprints, and compound segment offsets
