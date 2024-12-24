use indicatif::ProgressStyle;
use mp2::{Knapsack, Set, SetConfig};

fn main() {
    use mp2::{dynamic_programming, greedy};
    use std::thread;
    // allocate a stack size of 100 MiB
    let builder = thread::Builder::new().stack_size(1024 * 1024 * 100);

    let thread = builder
        .spawn(|| {
            benchmark_algorithm(dynamic_programming::bottom_up);
            benchmark_algorithm(dynamic_programming::top_down_memoized);

            benchmark_algorithm(greedy::smallest_weight_first);
            benchmark_algorithm(greedy::largest_value_first);
            benchmark_algorithm(greedy::greatest_worth_first);
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

    let capacity = 1000;
    let mut n = 100;

    let algorithm_name = std::any::type_name::<Algorithm>()
        .split("::")
        .last()
        .unwrap();

    let csv_path = format!("{}.csv", algorithm_name);

    if std::fs::exists(&csv_path).unwrap() {
        std::fs::remove_file(&csv_path).expect("Cannot remove file");
    }

    let mut writer = csv::Writer::from_path(csv_path).unwrap();
    writer.write_record(["n", "seconds"]).unwrap();

    let bar = ProgressBar::new(100_000);
    bar.set_message(algorithm_name);
    bar.set_style(
        ProgressStyle::with_template(
            "{msg} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
        )
        .unwrap(),
    );

    while n <= 100_000 {
        let mut trials = [0.0; 3];

        for i in 0..3 {
            let rng: StdRng = SeedableRng::seed_from_u64(n as u64 + i);
            let config = SetConfig {
                min_weight: 1,
                max_weight: 3,
                min_value: 10,
                max_value: 30,
                total: n,
            };
            let set = Set::new_random(config, rng);

            let now = Instant::now();
            let _knapsack = algorithm(&set, capacity);

            let elapsed = now.elapsed().as_secs_f64();
            trials[i as usize] = elapsed;
        }

        writer
            .write_record([
                format!("{}", n),
                format!("{}", trials.iter().sum::<f64>() / 3.0),
            ])
            .unwrap();

        n += 50;
        bar.inc(50)
    }
}
