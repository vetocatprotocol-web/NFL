# NFL Benchmark: IO Patterns

## Targets

NFL must minimize syscall overhead and align to efficient IO patterns.

- batch reads rather than tiny page requests
- scatter/gather semantics for segment collection
- direct IO compatibility for write path

## Methodology

- benchmark buffered vs direct writes for segment commit
- measure read amplification for sparse segment access
- validate prefetch and async submission behavior
