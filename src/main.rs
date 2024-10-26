use std::fmt;
use rand::Rng;
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};

#[derive(Copy, Clone)]
struct BitString {
    data: u64
}

impl BitString {
    fn new(data: u64) -> Self {
        Self { data  }
    }

    fn is_bit_set(&self, index: usize) -> bool {
        (self.data & (1u64 << index)) != 0
    }

    fn least_significant_bit(&self) -> usize {
        self.data.trailing_zeros() as usize
    }

    fn flip_bit(&mut self, index: usize) {
        self.data ^= 1 << index;
    }
}

impl fmt::Display for BitString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:08b}", self.data)
    }
}

struct Knapsack {
    weights: Vec<u16>,
    values: Vec<u16>,
    max_weight: u64,
    total_items: usize,
}

impl Knapsack {
    fn new(max_weight: u64, total_items: usize) -> Self {
        Self {
            weights: Vec::with_capacity(total_items as usize),
            values: Vec::with_capacity(total_items as usize),
            max_weight,
            total_items,
        }
    }

    fn set_value_and_weight(&mut self, value: u16, weight: u16) {
        self.weights.push(weight);
        self.values.push(value);
    }


    fn compute_value_and_weight(&self, subset: BitString) -> (u64, u64) {
        return (0..self.total_items).fold((0, 0), |acc, index| {
            if subset.is_bit_set(index) {
                (acc.0 + self.values[index] as u64, acc.1 + self.weights[index] as u64)
            } else {
                acc
            }
        })
    }

    fn solve(&self) -> (BitString, u64) {

        // we ignore the 0 bit string since it doesn't have any value
        let mut bit_str = BitString::new(0);

        let mut max_value = u64::MIN;
        let mut best_subset = BitString::new(0);

        let n = 1 << self.total_items;

        let pb = ProgressBar::new(n as u64);
            pb.set_style(
                ProgressStyle::with_template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
                )
                .unwrap(),
            );

        for i in (1..n).progress_with(pb) {
            let lsb = BitString::new(i).least_significant_bit();
            bit_str.flip_bit(lsb);

            let (value, weight) = self.compute_value_and_weight(bit_str);

            if weight > self.max_weight {
                continue
            }

            if max_value < value {
                max_value = value;
                best_subset = bit_str;
            }
        }

        return (best_subset, max_value)
    }
}



fn main() {
    let size = 35;
    let mut knapsack = Knapsack::new(1000, size);

    for _ in 0..size {
        let weight = rand::thread_rng().gen_range(50..=100);
        let value = rand::thread_rng().gen_range(100..=500);

        knapsack.set_value_and_weight(value, weight);
    }

    let (subset, value) = knapsack.solve();

    println!("{subset} {value}");
}
