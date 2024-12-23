use mp2::dynamic_programming;
use mp2::{Item, Set};

#[test]
fn bottom_up_test() {
    let set = Set::new(vec![
        Item {
            weight: 2,
            value: 29,
        },
        Item {
            weight: 2,
            value: 23,
        },
        Item {
            weight: 1,
            value: 18,
        },
        Item {
            weight: 1,
            value: 13,
        },
        Item {
            weight: 1,
            value: 15,
        },
    ]);
    let capacity = 5;

    let knapsack = dynamic_programming::bottom_up(&set, capacity);

    assert_eq!(
        knapsack.items(&set),
        vec![
            Item {
                weight: 1,
                value: 15
            },
            Item {
                weight: 1,
                value: 13,
            },
            Item {
                weight: 1,
                value: 18
            },
            Item {
                weight: 2,
                value: 29
            }
        ]
    )
}
