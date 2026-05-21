# NFL Benchmark: Scalability

## Targets

NFL should scale near-linearly across multiple cores for read-heavy workloads.

- multi-threaded sequential scan parallelism
- concurrent vector search query execution
- index lookup throughput under concurrent readers

## Methodology

- benchmark shard-based segment scans with thread pools
- measure lock-free read path performance as thread count increases
- avoid global contention in hot loops
