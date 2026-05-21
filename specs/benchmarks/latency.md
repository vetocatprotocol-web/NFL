# NFL Benchmark: Latency

## Targets

NFL aims for sub-10 microsecond indexed lookup latency on warm data.

- measure single-row or single-vector lookup paths
- evaluate segment directory and descriptor validation overhead
- include cold and warm mmap path comparisons

## Methodology

- benchmark random access across index and data segments
- isolate descriptor validation, page fault cost, and payload decode cost
- capture tail latencies (P95/P99)
