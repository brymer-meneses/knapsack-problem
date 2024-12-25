use indicatif::ProgressStyle;
use mp2::{Knapsack, Set, SetGenerationConfig};

fn main() {
    use mp2::{dynamic_programming, greedy};
    use std::thread;

    // Launch a new thread for benchmarking the algorithms. If we don't the
    // top down memoized dynamic programming algorithm will cause a segfault.
    let builder = thread::Builder::new().stack_size(1024 * 1024 * 100);

    let thread = builder
        .spawn(|| {
            benchmark_algorithm(greedy::smallest_weight_first);
            benchmark_algorithm(greedy::largest_value_first);
            benchmark_algorithm(greedy::greatest_worth_first);
            benchmark_algorithm(dynamic_programming::top_down_memoized);
            benchmark_algorithm(dynamic_programming::bottom_up);
        })
        .unwrap();

    thread.join().unwrap();
}

fn benchmark_algorithm<Algorithm>(algorithm: Algorithm)
where
    Algorithm: Fn(&Set, usize) -> Knapsack,
{
    use indicatif::ProgressBar;
    use rand::{rngs::StdRng, SeedableRng};
    use std::time::Instant;

    let algorithm_name = std::any::type_name::<Algorithm>()
        .split("::")
        .last()
        .unwrap();

    let csv_path = format!("{}.csv", algorithm_name);

    if std::fs::exists(&csv_path).unwrap() {
        std::fs::remove_file(&csv_path).expect("Cannot remove file");
    }

    let mut writer = csv::Writer::from_path(csv_path).unwrap();
    writer
        .write_record(["n", "trial 1", "trial 2", "trial 3", "average"])
        .unwrap();

    let bar = ProgressBar::new(100_000);
    bar.set_message(algorithm_name);
    bar.set_style(
        ProgressStyle::with_template(
            "{msg} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
        )
        .unwrap(),
    );

    let capacity = 1000;
    let config = SetGenerationConfig {
        min_weight: 100,
        max_weight: 1500,
        min_value: 100,
        max_value: 500,
    };
    for mut n in (100..=101_000).step_by(1000) {
        let mut trials = [0.0; 3];

        if n > 100_000 {
            n = 100_000;
        }

        for i in 0..3 {
            let rng: StdRng = SeedableRng::seed_from_u64(n as u64 + i);
            let set = Set::new_random(config, n, rng);

            let now = Instant::now();
            let _knapsack = algorithm(&set, capacity);

            let elapsed = now.elapsed().as_secs_f64();
            trials[i as usize] = elapsed;
        }

        writer
            .write_record(
                [
                    n as f64,
                    trials[0],
                    trials[1],
                    trials[2],
                    (trials.iter().sum::<f64>() / 3.0),
                ]
                .map(|e| e.to_string()),
            )
            .unwrap();

        bar.inc(1000);
    }
}
