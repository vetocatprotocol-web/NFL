# NFL

NFL is a systems-first universal storage substrate for AI-native workloads.

## Workspace layout

- `crates/nfl-core` — shared primitives, alignment helpers, error domain
- `crates/nfl-format` — immutable segment layout, file header, segment directory
- `crates/nfl-schema` — schema and type metadata
- `crates/nfl-types` — storage-native scalar and vector types
- `crates/nfl-io` — async IO and direct IO integration
- `crates/nfl-mmap` — mmap-first memory mapping helpers
- `crates/nfl-simd` — architecture-specific SIMD kernels
- `crates/nfl-index` — index metadata and lookup structures
- `crates/nfl-vector` — vector embedding storage and search
- `crates/nfl-tensor` — tensor layout and quantized tensors
- `crates/nfl-arrow` — Arrow interoperability and buffer translation
- `crates/nfl-query` — query execution and predicate pushdown
- `crates/nfl-security` — encryption and integrity layer
- `crates/nfl-testing` — full test harness and corruption validation
- `apps/nfl-cli` — command-line tooling
- `apps/nfl-bench` — benchmark harness

## Current progress

- Initialized Cargo workspace with core and format crates
- Implemented NFL file header and aligned segment directory metadata
- Added zero-copy parsing primitives and header validation tests
- Added a Criterion benchmark harness for header path
- Established `specs/` documentation for format, RFCs, and benchmarks

## Next steps

1. Expand `crates/nfl-format` to support segment dictionary encoding, compressed block layout, and independent checksum validation.
2. Add `crates/nfl-mmap` for page-aligned memory mapping, file-backed segment views, and zero-copy access APIs.
3. Implement `crates/nfl-schema` metadata and Arrow schema compatibility through `crates/nfl-arrow`.

## Specifications

- `specs/format` — binary layout and on-disk contract
- `specs/rfcs` — architectural proposals for core subsystems
- `specs/benchmarks` — performance targets and validation methodology
