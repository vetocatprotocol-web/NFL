# NFL Benchmark: Vector Search

## Targets

Measure ANN and similarity search performance for AI-native vector workloads.

- cosine similarity, dot product, euclidean distance
- HNSW graph traversal throughput
- high-dimensional embedding recall/latency tradeoffs

## Methodology

- benchmark vector block scan and k-NN search inside index segments
- include quantized and unquantized vector execution paths
- measure both preparation and query execution cost
