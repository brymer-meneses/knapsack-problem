use std::{fmt, time::Instant};
use rand::Rng;
use indicatif::{ProgressBar, ProgressStyle};

use tabled::{Table, Tabled};

#[derive(Copy, Clone)]
struct BitString {
    data: u64
}

impl BitString {
    fn new(data: u64) -> Self {
        Self { data  }
    }

    #[inline(always)]
    fn is_bit_set(&self, index: usize) -> bool {
        (self.data & (1u64 << index)) != 0
    }

    fn least_significant_bit(&self) -> usize {
        self.data.trailing_zeros() as usize
    }

    #[inline(always)]
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
            weights: vec![0; total_items],
            values: vec![0; total_items],
            max_weight,
            total_items,
        }
    }


    fn initialize_values(&mut self) {
        for i in 0..self.total_items {
            let weight = rand::thread_rng().gen_range(50..=100);
            let value = rand::thread_rng().gen_range(100..=500);
            self.weights[i] = weight;
            self.values[i] = value;
        }
    }

    fn print_weights_and_values(&self) {
        #[derive(Tabled)]
        struct Item {
            index: usize,
            weight: u16,
            value: u16,
        }

        let items: Vec<Item> = (0..self.total_items)
                .map(|i| Item {
                    index: i,
                    weight: self.weights[i],
                    value: self.values[i],
                })
                .collect();
        let table = Table::new(&items).to_string();
        println!("{}", table);
    }

    fn print_best_subset(&self, subset: BitString) {
        #[derive(Tabled)]
        struct Item {
            index: usize,
            weight: u16,
            value: u16,
        }


        let mut items: Vec<Item> = Vec::with_capacity(self.total_items);
        for i in 0..self.total_items {
            if subset.is_bit_set(i) {
                items.push(Item {
                    index: i,
                    weight: self.weights[i],
                    value: self.values[i]
                })
            }
        }

        let table = Table::new(&items).to_string();
        println!("{}", table);
    }

    fn solve(&self) -> (BitString, u64) {

        // we ignore the 0 bit string since it doesn't have any value
        let mut bit_str = BitString::new(0);

        let mut max_value = 0;
        let mut best_subset = BitString::new(0);

        let mut current_weight = 0;
        let mut current_value = 0;

        let n = 1 << self.total_items;

        let pb = ProgressBar::new(n as u64);
            pb.set_style(
                ProgressStyle::with_template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
                )
                .unwrap(),
            );


        for i in 1..n {
            let lsb = BitString::new(i).least_significant_bit();
            bit_str.flip_bit(lsb);

            if bit_str.is_bit_set(lsb) {
                current_weight += self.weights[lsb] as u64;
                current_value += self.values[lsb] as u64;
            } else {
                current_weight -= self.weights[lsb] as u64;
                current_value -= self.values[lsb] as u64;
            }

            if current_weight > self.max_weight {
                continue
            }

            if max_value < current_value {
                max_value = current_value;
                best_subset = bit_str;
            }

            if i % 1_000_000 == 0 {
                pb.set_position(i);
            }
        }

        return (best_subset, max_value)
    }
}

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of total tiems
    #[arg(short, long)]
    size: usize,

    /// Number of times to run the experiment
    #[arg(short, long, default_value_t = 3)]
    trials: usize,
}

fn main() {
    let args = Args::parse();

    let mut knapsack = Knapsack::new(1000, args.size);
    let mut trial_results: Vec<f64> = vec![];

    for i in 0..args.trials {
        println!("-------------------------------- TRIAL {i} --------------------------------");

        knapsack.initialize_values();
        knapsack.print_weights_and_values();

        let now = Instant::now();
        let (subset, value) = knapsack.solve();
        let elapsed = now.elapsed().as_secs_f64();

        trial_results.push(elapsed);

        println!("Done! Took {elapsed} seconds");
        println!("Best subset with value: {value} is");
        knapsack.print_best_subset(subset);


        println!("--------------------------------------------------------------------------");
        println!();
        println!();
    }

    println!("Took on average {}", trial_results.iter().sum::<f64>() / args.trials as f64);
}
