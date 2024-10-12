use std::fmt;
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};

use rand::Rng;


#[derive(Clone, Copy, Debug)]
struct KnapsackItem {
    weight: u8,
    value: u16,
}

struct BitString {
    data: u64,
    length: u8,
}

impl fmt::Display for KnapsackItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})", self.value, self.weight)
    }
}

impl fmt::Display for BitString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:050b}", self.data)
    }
}

impl BitString {
    fn new(data: u64, length: u8) -> BitString {
        return BitString {data, length}
    }

    fn generate_subset<T: Copy>(&self, items: &[T]) -> Vec<T> {
        let mut result = vec![];
        for bit in 0..self.length {
            if self.is_bit_set(bit) {
                result.push(items[bit as usize])
            }
        }
        return result;
    }

    fn is_bit_set(&self, index: u8) -> bool {
        (self.data & (1u64 << index)) != 0
    }
}

fn solve_knapsack(items: &[KnapsackItem], max_weight: u64) -> (Vec<KnapsackItem>, u64) {
    let length = items.len();
    let n = (2 as usize).pow(length as u32);

    let mut current_value = std::u64::MIN;
    let mut current_subset = vec![];
    let pb = ProgressBar::new(n as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
        )
        .unwrap(),
    );

    for i in (0..n).progress_with(pb) {
        let bit_string = BitString::new(i as u64, length as u8);
        let subset = bit_string.generate_subset(items);
        let (weight, value) = subset.iter().fold((0u64, 0u64), |(acc_weight, acc_value), item| {
            (acc_weight + item.weight as u64,  acc_value + item.value as u64)
        });
        if weight > max_weight{
            continue;
        }

        if value > current_value {
            current_value = value;
            current_subset = subset.clone();
        }
    }

    return (current_subset, current_value)
}


fn main() {
    let items: Vec<KnapsackItem> = (0..30).map(|_| {
        KnapsackItem {
            weight: rand::thread_rng().gen_range(50..=100), 
            value: rand::thread_rng().gen_range(100..=500)
        }
    }).collect();

    let (subset, value)= solve_knapsack(&items, 1000);

    println!("Found {:?}", subset);
    println!("Value {}", value);
}

#[test]
fn test_generate_subset() {
    let bit_string = BitString::new(0b111, 3);
    let subset = bit_string.generate_subset(&[1, 2, 3]);
    assert_eq!(subset, [1, 2, 3])
}

