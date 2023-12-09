use itertools::Itertools;

use crate::cell::Cell;
use crate::cell::State::{Alive, Dead};
use crate::universe::Universe;

#[test]
fn test_no_neighbor() {
    let universe = Universe::from_string("*");
    let actual = universe.iter().collect_vec();

    let expected = vec![Cell {
        x: 0,
        y: 0,
        state: Alive,
        live_neighbors: 0,
    }];

    itertools::assert_equal(expected, actual);
}

#[test]
fn test_one_neighbor() {
    let universe = Universe::from_string("**");
    let actual = universe.iter();

    let expected = vec![
        Cell {
            x: 0,
            y: 0,
            state: Alive,
            live_neighbors: 1,
        },
        Cell {
            x: 1,
            y: 0,
            state: Alive,
            live_neighbors: 1,
        },
        Cell {
            x: 0,
            y: 1,
            state: Dead,
            live_neighbors: 2,
        },
        Cell {
            x: 1,
            y: 1,
            state: Dead,
            live_neighbors: 2,
        },
    ];

    itertools::assert_equal(expected, actual);
}
