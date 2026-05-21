# RFC 0001: NFL Format Architecture

## Overview

NFL is built around immutable file segments and a lightweight segment directory. The design prioritizes:

- 4K-aligned independent segments
- zero-copy reads via mmap
- immutable versioned segment publishing
- forward-compatible extension points
- low-overhead checksum validation

## Segment model

A file is a container of segments. Each segment is a self-contained unit:

- schema metadata
- data payloads
- index structures
- transactional metadata
- footer summaries

Segments may be accessed directly from mmap and are validated by descriptor metadata.

## Directory and header

The file header and segment directory are the only mutable parts during file creation.

- header at offset 0
- directory aligned to 4K
- immutable segment entries after commit
- directory size and entry count encoded in header

## Extension strategy

- New segment kinds are added without changing the base descriptor format.
- Custom segment kinds use the `0x8000+` range.
- Flags and reserved fields allow future capabilities to land without compatibility breaks.

## Compatibility

- Readers ignore unknown segment kinds and reserved flags.
- Writers may include optional segments for newer feature sets.
- Version negotiation is explicit, and unknown versions fail fast if unsupported.
