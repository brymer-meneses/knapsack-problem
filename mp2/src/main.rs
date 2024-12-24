use mp2::{dynamic_programming, greedy, Knapsack};
use mp2::{Set, SetConfig};

fn main() {
    benchmark_algorithm(dynamic_programming::bottom_up);
    benchmark_algorithm(dynamic_programming::top_down_memoized);

    benchmark_algorithm(greedy::smallest_weight_first);
    benchmark_algorithm(greedy::largest_value_first);
    benchmark_algorithm(greedy::greatest_worth_first);
}

fn benchmark_algorithm<Algorithm>(algorithm: Algorithm)
where
    Algorithm: Fn(&Set, usize) -> Knapsack,
{
    use rand::{rngs::StdRng, SeedableRng};
    use std::time::Instant;

    println!("Testing {}", std::any::type_name::<Algorithm>());
    let capacity = 1000;
    let mut n = 100;

    while n <= 100_000 {
        let mut trials = vec![];

        for _ in 0..3 {
            let rng: StdRng = SeedableRng::seed_from_u64(n as u64);
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
            trials.push(elapsed);
        }

        println!("n: {} = {}", n, trials.iter().sum::<f64>() / 3.0);

        n *= 10;
    }
}
