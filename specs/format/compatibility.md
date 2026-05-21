# NFL Compatibility Model

NFL is designed for long-term binary stability and forward-compatible evolution.

## Versioning policy

- Major version changes may introduce incompatible segment semantics.
- Minor version changes must preserve existing on-disk structures.
- Version negotiation occurs through the header's `version` field.

## Forward compatibility

- Unknown segment kinds are treated as opaque by readers.
- Flags and reserved fields are allowed to be ignored if unsupported.
- Custom segment kinds enable vendor-specific extensions without breaking core readers.

## Backward compatibility

- Readers must accept older versions that use known segment kinds.
- Deprecated fields may remain present but unused.
- A compatibility shim layer may normalize older metadata to current runtime structures.

## Stability guarantees

- The header, segment descriptor, and directory formats are stable and fixed-size.
- Segment payload schemas are versioned at the schema segment level.
- Checksums provide corruption detection even when extension metadata is present.
