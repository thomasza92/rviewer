use criterion::{black_box, criterion_group, criterion_main, Criterion};
use egui::Context;
use rviewer::loader::{extract_exif_metadata, load_displayimage};

fn benchmark_extract_exif_metadata(c: &mut Criterion) {
    let test_image_path = "assets/test-images/testimg1.jpg";

    c.bench_function("extract_exif_metadata", |b| {
        b.iter(|| {
            extract_exif_metadata(black_box(test_image_path));
        });
    });
}

fn benchmark_load_displayimage(c: &mut Criterion) {
    let test_image_path = "assets/test-images/testimg1.jpg";
    let ctx = Context::default();

    c.bench_function("load_displayimage", |b| {
        b.iter(|| {
            load_displayimage(black_box(&ctx), black_box(test_image_path));
        });
    });
}

criterion_group!(
    benches,
    benchmark_extract_exif_metadata,
    benchmark_load_displayimage
);
criterion_main!(benches);
