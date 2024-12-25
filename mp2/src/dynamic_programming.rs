use std::cmp;

use crate::{Knapsack, Set};

pub fn create_bottom_up_table(set: &Set, capacity: usize) -> Vec<Vec<u16>> {
    let n = set.len();
    let mut v = vec![vec![0; capacity + 1]; n + 1];
    for i in 1..=n {
        let item = set[i - 1];

        for j in 0..=capacity {
            if j == 0 || i == 0 {
                v[i][j] = 0
            } else if j < item.weight as usize {
                v[i][j] = v[i - 1][j]
            } else {
                v[i][j] = cmp::max(v[i - 1][j], item.value + v[i - 1][j - item.weight as usize]);
            }
        }
    }
    return v;
}

// NOTE: I really did not like to put this code here, since I find it ugly mixing the
// benchmarking stuff with the actual algorithm. But I think I have no choice here?
// I thought of making a data type for the DP table, but I feel like that's too much work and can
// be more ugly.
#[derive(Default)]
pub struct TopDownTableMetrics {
    pub cache_miss: usize,
    pub cache_hit: usize,
}

pub fn create_top_down_memoized_table(
    set: &Set,
    capacity: usize,
) -> (Vec<Vec<i32>>, TopDownTableMetrics) {
    // This is the only function that can call this, so might as well put it inside of it's parent.
    fn mf_knapsack(
        set: &Set,
        v: &mut Vec<Vec<i32>>,
        metrics: &mut TopDownTableMetrics,
        i: usize,
        j: usize,
    ) -> usize {
        if v[i][j] >= 0 {
            metrics.cache_hit += 1;
            return v[i][j] as usize;
        }

        metrics.cache_miss += 1;
        let item = set[i - 1];

        let value = if j < item.weight as usize {
            mf_knapsack(set, v, metrics, i - 1, j)
        } else {
            cmp::max(
                mf_knapsack(set, v, metrics, i - 1, j),
                item.value as usize + mf_knapsack(set, v, metrics, i - 1, j - item.weight as usize),
            )
        };

        v[i][j] = value as i32;
        return value;
    }

    let mut metrics = TopDownTableMetrics {
        cache_miss: 0,
        cache_hit: 0,
    };

    let n = set.len();
    let mut v = vec![vec![-1; capacity + 1]; n + 1];

    for i in 0..=n {
        v[i][0] = 0;
    }
    for j in 0..=capacity {
        v[0][j] = 0;
    }

    mf_knapsack(set, &mut v, &mut metrics, n, capacity);

    return (v, metrics);
}

pub fn backtrack<T: Eq>(set: &Set, capacity: usize, v: Vec<Vec<T>>) -> Knapsack {
    let mut knapsack = Knapsack::new();
    let mut i = set.len();
    let mut j = capacity;

    while i > 0 && j > 0 {
        let item = set[i - 1];

        if v[i][j] != v[i - 1][j] {
            knapsack.insert(item);
            j -= item.weight as usize;
        }

        i -= 1;
    }
    return knapsack;
}

// NOTE: I need to measure time efficiency of the backtracking algorithm separately so ...

pub fn bottom_up(set: &Set, capacity: usize) -> Knapsack {
    let v = create_bottom_up_table(set, capacity);
    return backtrack(set, capacity, v);
}

pub fn top_down_memoized(set: &Set, capacity: usize) -> Knapsack {
    let (v, _) = create_top_down_memoized_table(set, capacity);
    return backtrack(set, capacity, v);
}
