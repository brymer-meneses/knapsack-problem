use mp2::dynamic_programming;
use mp2::{Set, SetConfig};

use rand::{rngs::StdRng, SeedableRng};

fn main() {
    let capacity = 5;
    let rng: StdRng = SeedableRng::seed_from_u64(1);

    let set = Set::new_random(
        SetConfig {
            min_weight: 1,
            max_weight: 3,
            min_value: 10,
            max_value: 30,
            total: 5,
        },
        rng,
    );

    let knapsack = dynamic_programming::bottom_up(&set, capacity);

    println!("{:?}", knapsack);
}
