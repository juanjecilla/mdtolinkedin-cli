use criterion::{criterion_group, criterion_main, Criterion};
use mdtolinkedin::converter::{convert, ConvertOptions};

fn build_large_input() -> String {
    let base = "# Header\n\n**bold** and *italic* with a [link](https://example.com).\n\n- item one\n- item two\n\n> quote line\n\n```rust\nfn main() {}\n```\n\n";
    let mut out = String::with_capacity(base.len() * 200);
    for _ in 0..200 {
        out.push_str(base);
    }
    out
}

fn bench_convert_large(c: &mut Criterion) {
    let input = build_large_input();
    let options = ConvertOptions::default();

    c.bench_function("convert_large", |b| {
        b.iter(|| {
            let _ = convert(&input, &options);
        })
    });
}

criterion_group!(benches, bench_convert_large);
criterion_main!(benches);
