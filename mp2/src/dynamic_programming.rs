use std::{cmp, usize};

use crate::{Knapsack, Set};

pub fn bottom_up(set: &Set, capacity: usize) -> Knapsack {
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

pub fn top_down_memoized(set: &Set, capacity: usize) -> Knapsack {
    let n = set.len();
    let mut v = vec![vec![-1; capacity + 1]; n + 1];

    for i in 0..=n {
        v[i][0] = 0;
    }
    for j in 0..=capacity {
        v[0][j] = 0;
    }

    mf_knapsack(set, &mut v, n, capacity);

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

fn mf_knapsack(set: &Set, v: &mut Vec<Vec<i32>>, i: usize, j: usize) -> usize {
    if v[i][j] < 0 {
        let item = set[i - 1];
        let value;

        if j < item.weight as usize {
            value = mf_knapsack(set, v, i - 1, j)
        } else {
            value = cmp::max(
                mf_knapsack(set, v, i - 1, j),
                item.value as usize + mf_knapsack(set, v, i - 1, j - item.weight as usize),
            )
        }
        v[i][j] = value as i32
    }

    return v[i][j] as usize;
}
