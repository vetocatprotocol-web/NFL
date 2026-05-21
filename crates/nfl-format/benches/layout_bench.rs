use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nfl_format::layout::{Header, SegmentDescriptor, SegmentDirectory, SegmentKind, HEADER_SIZE};

pub fn header_checksum_benchmark(c: &mut Criterion) {
    let entries = [SegmentDescriptor::new(SegmentKind::Data, 0, 8192, 2048, 0xdead_beef_cafe_babe)];
    let (directory_offset, directory_length) = SegmentDirectory::layout(&entries);
    let header = Header::new(directory_offset, directory_length, entries.len() as u32);

    c.bench_function("header checksum", |b| {
        b.iter(|| black_box(header.checksum()));
    });
}

pub fn header_parse_benchmark(c: &mut Criterion) {
    let entries = [SegmentDescriptor::new(SegmentKind::Data, 0, 8192, 2048, 0xdead_beef_cafe_babe)];
    let (directory_offset, directory_length) = SegmentDirectory::layout(&entries);
    let header = Header::new(directory_offset, directory_length, entries.len() as u32);
    let mut buffer = [0u8; HEADER_SIZE];
    header.write_to(&mut buffer).unwrap();

    c.bench_function("header parse", |b| {
        b.iter(|| Header::parse(black_box(&buffer)).unwrap());
    });
}

criterion_group!(benches, header_checksum_benchmark, header_parse_benchmark);
criterion_main!(benches);
