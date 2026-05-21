# RFC 0002: NFL I/O and mmap Model

## Goal

Provide a production-grade I/O model that supports:

- mmap-first read paths
- buffered and direct IO for writes
- async submission for large workloads
- memory-aligned page management

## Read path

- Map header and directory first.
- Validate segment descriptors.
- Map segments lazily as needed.
- Use OS hints for sequential scans and prefetching.

## Write path

- Writes are append-only.
- Segment contents are written to buffered or direct I/O targets.
- A final commit writes the segment directory and header.
- Partial writes must be detectable by a separately stored footer or atomic rename.

## mmap semantics

- Segment base must be page-aligned.
- Payload structures must use offset-based references. No raw pointers on disk.
- Variable-length data is encoded with explicit size and offset tables.
