use std::{hint::black_box, sync::Arc, thread};
use criterion::{ criterion_group, criterion_main, Criterion};

fn arc_thread(n: usize, clones_per_thread: usize) {
    let data = Arc::new(0);
    let mut handles = vec![];

    for _ in 0..n {
        let data = data.clone();
        handles.push(thread::spawn(move || {
            let mut clones = vec![];
            for _ in 0..clones_per_thread {
                let clone = black_box(data.clone());
                clones.push(clone);
            }
            // 显式丢弃，模拟使用
            drop(clones);
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
}

fn arc_benchmark(c: &mut Criterion) {
    c.bench_function("arc_benchmark", |b|  b.iter(|| arc_thread(8, 125000)));
}
criterion_group!(benches, arc_benchmark);
criterion_main!(benches);