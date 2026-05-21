# RFC 0003: Vector and Tensor Storage

## Objectives

NFL must support AI-native storage with:

- fixed-length embeddings
- quantized tensors
- BF16 and FP16 data paths
- ABI-compatible vector blocks
- ANN and similarity indexing

## Storage layout

- Vectors are stored in dense fixed-size blocks with alignment to 64 bytes when possible.
- Tensor shapes are stored alongside type metadata in schema segments.
- Quantized tensors include scale/zero-point metadata in a compact block header.

## Access model

- Readers can interpret vector blocks as raw slices or SIMD-ready registers.
- Shape and stride information is stored in the schema segment for zero-copy tensor view creation.
- Vector similarity kernels must support cosine, dot, and euclidean metrics.

## Compatibility

- Vector blocks are versioned independently from segment layout.
- New quantization schemes may be introduced as optional segment metadata.
