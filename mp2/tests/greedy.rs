use mp2::greedy;
use mp2::{Item, Set};

#[test]
pub fn smallest_weight_first() {
    let set = Set::new(vec![
        Item {
            value: 60,
            weight: 10,
        },
        Item {
            value: 100,
            weight: 20,
        },
        Item {
            value: 120,
            weight: 30,
        },
    ]);
    let capacity = 50;
    let knapsack = greedy::smallest_weight_first(&set, capacity);
    assert_eq!(
        knapsack.items(),
        vec![
            Item {
                value: 60,
                weight: 10,
            },
            Item {
                value: 100,
                weight: 20,
            },
        ]
    )
}

#[test]
pub fn greatest_worth_first() {
    let set = Set::new(vec![
        Item {
            value: 60,
            weight: 10,
        },
        Item {
            value: 100,
            weight: 20,
        },
        Item {
            value: 120,
            weight: 30,
        },
    ]);
    let capacity = 50;

    let knapsack = greedy::greatest_worth_first(&set, capacity);
    assert_eq!(
        knapsack.items(),
        vec![
            Item {
                value: 60,
                weight: 10,
            },
            Item {
                value: 100,
                weight: 20,
            },
        ]
    )
}

#[test]
pub fn largest_value_first() {
    let set = Set::new(vec![
        Item {
            value: 60,
            weight: 10,
        },
        Item {
            value: 100,
            weight: 20,
        },
        Item {
            value: 120,
            weight: 30,
        },
    ]);
    let capacity = 50;

    let knapsack = greedy::largest_value_first(&set, capacity);
    assert_eq!(
        knapsack.items(),
        vec![
            Item {
                value: 120,
                weight: 30,
            },
            Item {
                value: 100,
                weight: 20,
            },
        ]
    )
}
