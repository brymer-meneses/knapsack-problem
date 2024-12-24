use crate::{Knapsack, Set};

pub fn smallest_weight_first(set: &Set, capacity: usize) -> Knapsack {
    // Sort items by weight in ascending order.
    let sorted = set.cloned_sort(|i1, i2| i1.weight.cmp(&i2.weight));

    let mut knapsack = Knapsack::new();
    for item in sorted.items {
        if knapsack.weight() + item.weight as usize <= capacity {
            knapsack.insert(item)
        }
    }
    return knapsack;
}

pub fn largest_value_first(set: &Set, capacity: usize) -> Knapsack {
    // Sort items by value in descending order.
    let sorted = set.cloned_sort(|i1, i2| i1.value.cmp(&i2.value).reverse());

    let mut knapsack = Knapsack::new();
    for item in sorted.items {
        if knapsack.weight() + item.weight as usize <= capacity {
            knapsack.insert(item)
        }
    }
    return knapsack;
}

pub fn greatest_worth_first(set: &Set, capacity: usize) -> Knapsack {
    // sort based on the value/weight ratio
    let sorted = set.cloned_sort(|i1, i2| {
        let r1 = i1.value as f64 / i1.weight as f64;
        let r2 = i2.value as f64 / i2.weight as f64;
        r1.total_cmp(&r2).reverse()
    });

    let mut knapsack = Knapsack::new();
    for item in sorted.items {
        if knapsack.weight() + item.weight as usize <= capacity {
            knapsack.insert(item)
        }
    }
    return knapsack;
}
