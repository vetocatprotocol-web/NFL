# RFC 0005: Security and Integrity

## Goals

- Protect data integrity across immutable files
- Support optional encryption and authenticated metadata
- Allow offline validation and corruption detection

## Integrity

- Header and segment checksums are mandatory for production files.
- Checksums are computed over header fields and segment payloads.
- Readers must validate segment descriptors and checksums before use.

## Encryption

- Encryption is optional and applied at the segment level.
- Metadata must remain readable to discover file layout.
- Encrypted segments are identified by a reserved flag and custom segment kind.

## Trust model

- Files are trusted only after checksum validation.
- Unknown or unsupported flags must be rejected or safely ignored if explicitly designed for extensibility.
