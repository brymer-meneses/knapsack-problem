fn main() {
    use std::thread;
    // Launch a new thread for benchmarking the algorithms. If we don't the
    // top down memoized dynamic programming algorithm will cause a segfault.
    let builder = thread::Builder::new().stack_size(1024 * 1024 * 100);

    let thread = builder.spawn(analysis_main).unwrap();

    thread.join().unwrap();
}

use indicatif::ProgressStyle;
use mp2::{dynamic_programming, utils, Knapsack, Set, SetGenerationConfig};

fn analysis_main() {
    use mp2::{dynamic_programming, greedy};

    analyze_greedy_algorithm(greedy::smallest_weight_first);
    analyze_greedy_algorithm(greedy::largest_value_first);
    analyze_greedy_algorithm(greedy::greatest_worth_first);

    analyze_bottom_up();
    analyze_top_down();
}

fn analyze_greedy_algorithm<Algorithm>(algorithm: Algorithm)
where
    Algorithm: Fn(&Set, usize) -> Knapsack,
{
    use indicatif::ProgressBar;
    use rand::{rngs::StdRng, SeedableRng};

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
        .write_record([
            "n", "trial 1", "trial 2", "trial 3", "average", "value 1", "value 2", "value 3",
        ])
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
        let mut values = [0.0; 3];

        if n > 100_000 {
            n = 100_000;
        }

        for i in 0..3 {
            let rng: StdRng = SeedableRng::seed_from_u64(n as u64 + i);
            let set = Set::new_random(config, n, rng);

            let (elapsed, knapsack) = utils::time(|| algorithm(&set, capacity));

            trials[i as usize] = elapsed;
            values[i as usize] = knapsack.value() as f64;
        }

        writer
            .write_record(
                [
                    n as f64,
                    trials[0],
                    trials[1],
                    trials[2],
                    (trials.iter().sum::<f64>() / 3.0),
                    values[0],
                    values[1],
                    values[2],
                ]
                .map(|e| e.to_string()),
            )
            .unwrap();

        bar.inc(1000);
    }
}

fn analyze_bottom_up() {
    use indicatif::ProgressBar;
    use rand::{rngs::StdRng, SeedableRng};

    let csv_path = "bottom_up.csv";
    if std::fs::exists(&csv_path).unwrap() {
        std::fs::remove_file(&csv_path).expect("Cannot remove file");
    }

    let mut writer = csv::Writer::from_path(csv_path).unwrap();
    writer
        .write_record([
            "n",
            "table 1",
            "table 2",
            "table 3",
            "average",
            "backtrack 1",
            "backtrack 2",
            "backtrack 3",
            "average",
        ])
        .unwrap();

    let bar = {
        let b = ProgressBar::new(100_000);
        b.set_message("bottom_up");
        b.set_style(
            ProgressStyle::with_template(
                "{msg} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
            )
            .unwrap(),
        );
        b
    };

    let capacity = 1000;
    let config = SetGenerationConfig {
        min_weight: 100,
        max_weight: 1500,
        min_value: 100,
        max_value: 500,
    };

    for mut n in (100..=101_000).step_by(1000) {
        let mut tables = [0.0; 3];
        let mut backtracks = [0.0; 3];

        if n > 100_000 {
            n = 100_000;
        }

        for i in 0..3 {
            let rng: StdRng = SeedableRng::seed_from_u64(n as u64 + i);
            let set = Set::new_random(config, n, rng);

            let (elapsed_table, v) =
                utils::time(|| dynamic_programming::create_bottom_up_table(&set, capacity));

            let (elapsed_backtrack, _) =
                utils::time(|| dynamic_programming::backtrack(&set, capacity, v));

            tables[i as usize] = elapsed_table;
            backtracks[i as usize] = elapsed_backtrack;
        }

        writer
            .write_record(
                [
                    n as f64,
                    tables[0],
                    tables[1],
                    tables[2],
                    (tables.iter().sum::<f64>() / 3.0),
                    backtracks[0],
                    backtracks[1],
                    backtracks[2],
                    (backtracks.iter().sum::<f64>() / 3.0),
                ]
                .map(|e| e.to_string()),
            )
            .unwrap();

        bar.inc(1000);
    }
}

fn analyze_top_down() {
    use indicatif::ProgressBar;
    use rand::{rngs::StdRng, SeedableRng};

    let csv_path = "top_down_memoized.csv";
    if std::fs::exists(&csv_path).unwrap() {
        std::fs::remove_file(&csv_path).expect("Cannot remove file");
    }

    let mut writer = csv::Writer::from_path(csv_path).unwrap();
    writer
        .write_record([
            "n",
            "table 1",
            "table 2",
            "table 3",
            "average",
            "backtrack 1",
            "backtrack 2",
            "backtrack 3",
            "average",
            "cache miss 1",
            "cache miss 2",
            "cache miss 3",
            "average",
            "cache hit 1",
            "cache hit 2",
            "cache hit 3",
            "average",
        ])
        .unwrap();

    let bar = {
        let b = ProgressBar::new(100_000);
        b.set_message("top_down_memoized");
        b.set_style(
            ProgressStyle::with_template(
                "{msg} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
            )
            .unwrap(),
        );
        b
    };

    let capacity = 1000;
    let config = SetGenerationConfig {
        min_weight: 100,
        max_weight: 1500,
        min_value: 100,
        max_value: 500,
    };

    for mut n in (100..=101_000).step_by(1000) {
        let mut tables = [0.0; 3];
        let mut backtracks = [0.0; 3];
        let mut cache_misses = [0.0; 3];
        let mut cache_hits = [0.0; 3];

        if n > 100_000 {
            n = 100_000;
        }

        for i in 0..3usize {
            let rng: StdRng = SeedableRng::seed_from_u64(n as u64 + i as u64);
            let set = Set::new_random(config, n, rng);

            let (elapsed_table, (v, metrics)) =
                utils::time(|| dynamic_programming::create_top_down_memoized_table(&set, capacity));

            let (elapsed_backtrack, _) =
                utils::time(|| dynamic_programming::backtrack(&set, capacity, v));

            tables[i] = elapsed_table;
            backtracks[i] = elapsed_backtrack;
            cache_hits[i] = metrics.cache_hit as f64;
            cache_misses[i] = metrics.cache_hit as f64;
        }

        writer
            .write_record(
                [
                    n as f64,
                    tables[0],
                    tables[1],
                    tables[2],
                    (tables.iter().sum::<f64>() / 3.0),
                    backtracks[0],
                    backtracks[1],
                    backtracks[2],
                    (backtracks.iter().sum::<f64>() / 3.0),
                    cache_misses[0],
                    cache_misses[1],
                    cache_misses[2],
                    (cache_misses.iter().sum::<f64>() / 3.0),
                    cache_hits[0],
                    cache_hits[1],
                    cache_hits[2],
                    (cache_hits.iter().sum::<f64>() / 3.0),
                ]
                .map(|e| e.to_string()),
            )
            .unwrap();

        bar.inc(1000);
    }
}
