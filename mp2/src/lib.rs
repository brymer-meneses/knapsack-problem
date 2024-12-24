use std::ops::Index;

pub mod dynamic_programming;
pub mod greedy;

#[derive(Debug)]
pub struct Set {
    pub items: Vec<Item>,
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
            let weight = rng.gen_range(config.min_weight..=config.max_weight);
            let value = rng.gen_range(config.min_value..=config.max_value);

            set.items[i].weight = weight;
            set.items[i].value = value;
        }

        return set;
    }

    pub fn new(items: Vec<Item>) -> Self {
        return Self { items };
    }

    pub fn cloned_sort<F>(&self, f: F) -> Self
    where
        F: FnMut(&Item, &Item) -> std::cmp::Ordering,
    {
        let mut items = self.items.clone();
        items.sort_by(f);

        Self::new(items)
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
    items: Vec<Item>,

    weight: usize,
    value: usize,
}

impl Knapsack {
    pub fn new() -> Self {
        return Self {
            items: Vec::new(),
            weight: 0,
            value: 0,
        };
    }

    pub fn insert(&mut self, item: Item) {
        self.items.push(item);

        self.weight += item.weight as usize;
        self.value += item.value as usize;
    }

    pub fn weight(&self) -> usize {
        return self.weight;
    }

    pub fn value(&self) -> usize {
        return self.value;
    }

    pub fn items(&self) -> &[Item] {
        return self.items.as_ref();
    }
}
