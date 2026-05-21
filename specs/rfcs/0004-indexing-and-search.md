# RFC 0004: Indexing and ANN Search

## Goals

Provide index subsystems that support:

- fast random lookup
- range and predicate indexes
- ANN vector search graphs
- parallel query-friendly structures

## Index segment types

- zone maps / range indexes
- bloom filters
- prefix indexes
- inverted lists
- HNSW and graph structures for ANN

## Design principles

- Index segments are read-only and independent.
- Queries use predicate pushdown to avoid unnecessary segment materialization.
- Graph metadata is stored in a compressed, page-aligned layout for fast traversal.
- Index structures should be accessible directly from mmap whenever possible.
