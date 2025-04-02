// SPDX-License-Identifier: LGPL-3.0-or-later
//
// (c) SYSTEC electronic AG, D-08468 Heinsdorfergrund, Am Windrad 2
//     www.systec-electronic.com

use std::collections::HashMap;

use criterion::{criterion_group, criterion_main, Criterion};

use sysworxx_io::io::util::PairMap;

#[derive(Debug, PartialEq, Eq)]
struct Small(u32);

#[derive(Debug, PartialEq, Eq)]
struct Fat(u64, u64, u64, u64);

pub fn bench_hashmap_small(c: &mut Criterion) {
    let mut map = HashMap::new();

    for i in 0..8 {
        map.insert(i, Small(i as u32));
    }

    c.bench_function("hash map small", |b| {
        for i in 0..8 {
            b.iter(|| map.get(&i).unwrap());
        }
    });
}

// seems like the size of the struct does not have any influence on the benchmark
// pub fn bench_hashmap_fat(c: &mut Criterion) {
//     let mut map = HashMap::new();
//
//     for i in 0..8 {
//         map.insert(i, Fat(i as u64, i as u64, i as u64, i as u64));
//     }
//
//     c.bench_function("hash map fat", |b| {
//         for i in 0..8 {
//             b.iter(|| map.get(&i).unwrap());
//         }
//     });
// }

pub fn bench_pairmap_small(c: &mut Criterion) {
    let mut map: PairMap<u16, Small> = PairMap::new();

    for i in 0..8 {
        map.push(i, Small(i as u32));
    }

    c.bench_function("pait map small", |b| {
        for i in 0..8 {
            b.iter(|| {
                let _ = map.get(i).unwrap();
            });
        }
    });
}

pub fn bench_pairmap_fat(c: &mut Criterion) {
    let mut map: PairMap<u16, Fat> = PairMap::new();

    for i in 0..8 {
        map.push(i, Fat(i as u64, i as u64, i as u64, i as u64));
    }

    c.bench_function("pait map fat", |b| {
        for i in 0..8 {
            b.iter(|| {
                let _ = map.get(i).unwrap();
            });
        }
    });
}

criterion_group!(
    benches,
    bench_hashmap_small,
    // bench_hashmap_fat,
    bench_pairmap_small,
    bench_pairmap_fat,
);

criterion_main!(benches);
