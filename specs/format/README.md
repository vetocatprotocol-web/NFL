# NFL Format Specifications

This directory defines NFL's binary format and on-disk contract for production-grade storage.

The goals are:

- stable immutable segment architecture
- 4K-aligned mmap-first access
- forward and backward compatibility
- explicit checksum and integrity policies
- extensible segment kinds for AI-native workloads

## Documents

- `file-layout.md` — file header, directory placement, and footer semantics
- `segment-directory.md` — segment descriptor format and validation rules
- `segment-layout.md` — segment payload structure and hybrid storage model
- `mmap-access.md` — mmap semantics, zero-copy contracts, and safety rules
- `compatibility.md` — versioning policy and extensibility model
