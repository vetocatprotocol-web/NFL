# NFL Segment Directory

## Descriptor format

Each `SegmentDescriptor` is 32 bytes with the following fields:

- `kind` (u16)
- `flags` (u16)
- `reserved` (u32)
- `offset` (u64)
- `length` (u64)
- `checksum` (u64)

### Segment kinds

- `Schema` — schema and field metadata
- `Data` — columnar or hybrid row/column payloads
- `Index` — secondary indexes and ANN structures
- `Mvcc` — snapshot/version metadata
- `Footer` — file summary and optional verification
- `Custom` — user-defined extensions

## Validation rules

- `offset` must be 4K aligned
- `length` may be zero only for optional placeholder segments
- `offset + length` must not exceed file size
- `checksum` must validate segment data integrity when enabled

## Layout semantics

- Directory entries are immutable once written.
- The directory may contain holes only through reserved placeholder entries.
- Consumers must not assume any particular segment ordering beyond the declared directory.

## Extensibility

- New kinds may be assigned in the custom namespace (`0x8000+`).
- Reserved flags may be used for future segment features such as compression or encryption.
