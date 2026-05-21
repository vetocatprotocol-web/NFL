# NFL Segment Layout

NFL segments are the core unit of immutability and zero-copy access.

## Segment properties

- Segments are independently mmap-able and checksum-validated.
- Each segment begins on a 4K boundary.
- Segment size may be large, but internal page boundaries should align to 4K for OS-friendly access.
- Segments may encode row, column, or vector blocks depending on kind.

## Data segment structure

Data segments are hybrid row-columnar containers.

- Page-aligned block header
- Optional compression/encoding metadata
- Column offsets table for fast seeker access
- Payload blocks for raw bytes or encoded values

## Index segment structure

Index segments store auxiliary structures like:

- zone maps
- bloom filters
- sorted lookup tables
- ANN graph structures

Index segments are designed to be read directly from mmap and should avoid relocation.

## Immutability model

- Writes append new segments, never mutate existing segments in-place.
- Readers use snapshot-safe directory entries and can operate lock-free.
- New segment versions are published by writing an updated segment directory and header.
