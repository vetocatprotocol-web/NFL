# NFL Benchmark: Throughput

## Targets

NFL targets sustained sequential scan throughput approaching modern NVMe bandwidth.

- goal: > 10 GB/s scan throughput on optimized hardware
- measurement: end-to-end read+decode throughput for columnar and vector data
- dataset shapes: wide columns, dense embeddings, compressed payloads

## Methodology

- Use large contiguous segments for sequential scans.
- Avoid serialization overhead in hot read paths.
- Measure both raw byte throughput and decoded record throughput.

## Key metrics

- bytes-per-second from mapped pages
- CPU cycles per byte
- vector read bandwidth for embeddings
