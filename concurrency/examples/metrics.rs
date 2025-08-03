use std::{thread, time::Duration};
use rand::Rng;
use anyhow::{Ok, Result};
use concurrency::Metrics;

const N: usize = 2;
const M: usize = 4;

fn main() -> Result<()> {
    let metrics = Metrics::new();
    // metrics.inc("a");
    // metrics.inc("b");
    // metrics.dec("a");
    println!("{:?}", metrics);
    for idx in 0..N {
        task_worker(idx, metrics.clone())?;
    }

    for _ in 0..M {
        request_worker(metrics.clone())?;
    }

    loop {
        thread::sleep(Duration::from_secs(1));
        println!("{:?}", metrics);
    }
}

fn task_worker(idx: usize, metrics: Metrics) -> Result<()> {
    thread::spawn(move || loop {
        let mut rng  = rand::rng();
        thread::sleep(Duration::from_millis(rng.random_range(100..5000)));
        metrics.inc(format!("thread.worker.{}", idx)).unwrap();
    });
    Ok(())
}

fn request_worker(metrics: Metrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::rng();
            thread::sleep(Duration::from_millis(rng.random_range(100..800)));
            let page = rng.random_range(1..5);

            metrics.inc(format!("request.page.{}", page))?;
        }
        #[allow(unreachable_code)]
        Ok(())
    });
    Ok(())
}
