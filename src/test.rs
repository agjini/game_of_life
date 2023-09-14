#[cfg(test)]
mod tests {
    use crate::universe::components::{Cell, State, Universe};

    #[test]
    fn test_no_neighbor() {
        let cells = vec![Cell {
            x: 0,
            y: 0,
            state: State::Alive,
        }];
        let universe = Universe::snapshot(cells.iter());
        assert_eq!(universe.cells.len(), 1);
        assert_eq!(universe.get_alive_neighbours(&Cell { x: 0, y: 0, state: State::Dead }), 0);
    }

    #[test]
    fn test_one_neighbor() {
        let cells = vec![Cell {
            x: 0,
            y: 0,
            state: State::Alive,
        }, Cell {
            x: 0,
            y: 1,
            state: State::Alive,
        }];
        let universe = Universe::snapshot(cells.iter());
        assert_eq!(universe.cells.len(), 2);
        assert_eq!(universe.get_alive_neighbours(&Cell { x: 0, y: 0, state: State::Dead }), 1);
    }

    #[test]
    fn test_two_neighbors() {
        let cells = vec![Cell {
            x: 0,
            y: 0,
            state: State::Alive,
        }, Cell {
            x: 0,
            y: 1,
            state: State::Alive,
        }, Cell {
            x: 1,
            y: 1,
            state: State::Alive,
        }, Cell {
            x: 1,
            y: 3,
            state: State::Alive,
        }];
        let universe = Universe::snapshot(cells.iter());
        assert_eq!(universe.get_alive_neighbours(&cells[2]), 2);
    }
}

