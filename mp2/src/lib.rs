use std::ops::Index;

pub mod dynamic_programming;

#[derive(Debug)]
pub struct Set {
    items: Vec<Item>,
}

pub struct SetConfig {
    pub min_weight: u16,
    pub max_weight: u16,

    pub min_value: u16,
    pub max_value: u16,

    pub total: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Item {
    pub weight: u16,
    pub value: u16,
}

use rand::rngs::StdRng;
use rand::Rng;

impl Set {
    pub fn new_random(config: SetConfig, mut rng: StdRng) -> Self {
        let mut set = Self {
            items: vec![
                Item {
                    weight: 0,
                    value: 0,
                };
                config.total
            ],
        };

        for i in 0..config.total {
            let weight = rng.gen_range(config.min_weight..config.max_weight);
            let value = rng.gen_range(config.min_value..config.max_value);

            set.items[i].weight = weight;
            set.items[i].value = value;
        }

        return set;
    }

    pub fn new(items: Vec<Item>) -> Self {
        return Self { items };
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}

impl Index<usize> for Set {
    type Output = Item;
    fn index(&self, index: usize) -> &Self::Output {
        return &self.items[index];
    }
}

#[derive(Debug)]
pub struct Knapsack {
    items: Vec<usize>,
}

impl Knapsack {
    pub fn new() -> Self {
        return Self { items: Vec::new() };
    }

    pub fn insert(&mut self, item: usize) {
        self.items.push(item);
    }

    pub fn items(&self, set: &Set) -> Vec<Item> {
        self.items.iter().map(|i| set[*i]).collect()
    }
}
