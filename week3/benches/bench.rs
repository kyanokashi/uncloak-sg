#![allow(unused_imports)]
use std::{thread, time};

use criterion::{black_box as bb, criterion_group, criterion_main, Criterion};
use crypto::digest::Digest;
use crypto::{sha3, sha2};

const TEXT: &[u8] = b"some text";

fn bench_blake(c: &mut Criterion) {
    c.bench_function("blake3", |g| {
        g.iter(|| {
            let mut hasher = blake3::Hasher::new();
            hasher.update(bb(TEXT));
            hasher.finalize();
        })
    });
}

fn bench_sha_3(c: &mut Criterion) {
    c.bench_function("sha3-256", |g| {
        g.iter(|| {
            let mut hasher = sha3::Sha3::sha3_256();
            hasher.input(TEXT);
            hasher.result_str();
        })
    });
}

fn bench_sha_2(c: &mut Criterion) {
    c.bench_function("sha2-256", |g| {
        g.iter(|| {
            let mut hasher = sha2::Sha256::new();
            hasher.input(TEXT);
            hasher.result_str();
        })
    });
}

criterion_group!(benches, bench_blake, bench_sha_3, bench_sha_2);
criterion_main!(benches);