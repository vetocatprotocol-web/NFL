# NFL mmap Access Model

NFL is optimized for mmap-first zero-copy reads.

## Mapping contract

- File header and segment directory must be mapped before accessing segments.
- Segments are mapped at their file-aligned offsets.
- Readers must respect the 4K-aligned segment boundary contract.

## Zero-copy semantics

- Data structures stored in segments must be POD-compatible and stable across execution environments.
- Pointer-free representation is required for on-disk structures.
- Variable-length vectors and strings are referenced through offsets relative to the segment base.

## Safety guarantees

- A segment descriptor validation pass must verify bounds before mapping.
- Offsets within a segment must be validated before dereferencing.
- Unaligned loads are permitted only within validated payload buffers and via explicit SIMD-safe access paths.

## Performance considerations

- Prefer `madvise(MADV_SEQUENTIAL)` for scan-heavy segments.
- Use page-aligned prefetch hints to reduce first-touch faults.
- Avoid small random page loads in the hot read path.
