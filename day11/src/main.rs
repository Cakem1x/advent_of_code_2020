use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::prelude::Read;
#[cfg(test)]
use std::iter::FromIterator;
use std::ops::{Index, IndexMut};

#[derive(PartialEq, Clone, Debug)]
pub enum State {
    Empty,
    Occupied,
    Floor,
}

impl State {
    fn from(c: char) -> State {
        match c {
            'L' => State::Empty,
            '#' => State::Occupied,
            '.' => State::Floor,
            _ => panic!("invalid char"),
        }
    }
}

pub struct Grid {
    cells: Vec<State>,
    width: usize,
    height: usize,
}

impl Index<&(usize, usize)> for Grid {
    type Output = State;

    fn index(&self, point: &(usize, usize)) -> &Self::Output {
        &self.cells[point.0 + self.width * point.1]
    }
}

impl IndexMut<&(usize, usize)> for Grid {
    fn index_mut(&mut self, point: &(usize, usize)) -> &mut Self::Output {
        &mut self.cells[point.0 + self.width * point.1]
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                match self[&(x, y)] {
                    State::Empty => write!(f, "L")?,
                    State::Occupied => write!(f, "#")?,
                    State::Floor => write!(f, ".")?,
                };
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Grid {
    pub fn from(grid_str: &str) -> Grid {
        let mut cells = Vec::new();
        let mut height = 0;
        let mut width = 0;
        for grid_line in grid_str.split('\n') {
            if grid_line.is_empty() {
                continue;
            }
            for grid_char in grid_line.chars() {
                cells.push(State::from(grid_char));
                if height == 0 {
                    width += 1; // only increment width for first iteration
                }
            }
            height += 1;
        }

        Grid {
            cells,
            width,
            height,
        }
    }

    fn get_adjacent_positions(&self, position: &(usize, usize)) -> HashSet<(usize, usize)> {
        let mut neighbors = HashSet::new();
        for y_offset in [-1, 0, 1].iter() {
            for x_offset in [-1, 0, 1].iter() {
                if *x_offset == 0 && *y_offset == 0 {
                    continue;
                }
                let neighbor_x = position.0 as i64 + x_offset;
                let neighbor_y = position.1 as i64 + y_offset;

                // check under/overflow
                if neighbor_x < self.width as i64
                    && neighbor_x >= 0
                    && neighbor_y < self.height as i64
                    && neighbor_y >= 0
                {
                    neighbors.insert((neighbor_x as usize, neighbor_y as usize));
                }
            }
        }
        return neighbors;
    }

    /// Returns the cell's new state or none, if its state won't change.
    fn next_cell_state_part1(
        &self,
        cell_position: &(usize, usize),
        neighbors: &HashSet<(usize, usize)>,
    ) -> Option<State> {
        match self[cell_position] {
            State::Floor => None, // floor never changes!
            State::Empty => {
                if !neighbors
                    .iter()
                    .any(|neighbor| self[neighbor] == State::Occupied)
                // If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
                {
                    Some(State::Occupied)
                } else {
                    None
                }
            }
            State::Occupied => {
                if neighbors
                    .iter()
                    .filter(|&neighbor| self[neighbor] == State::Occupied)
                    .count()
                    >= 4
                // If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
                {
                    Some(State::Empty)
                } else {
                    None
                }
            }
        }
    }

    /// Changes all states in the grid, according to the rules.
    /// Returns the number of cells that had their states changed.
    pub fn transition_part1(&mut self) -> usize {
        let mut new_states = HashMap::<(usize, usize), State>::new();

        // Find which cells need new states
        for y in 0..self.height {
            for x in 0..self.width {
                let current_position = (x, y);
                let neighbors = self.get_adjacent_positions(&current_position);
                let new_state = self.next_cell_state_part1(&current_position, &neighbors);
                if new_state.is_some() {
                    new_states.insert(current_position, new_state.unwrap());
                }
            }
        }

        // Apply new states
        for (position, state) in new_states.iter() {
            self[position] = state.clone();
        }

        return new_states.len();
    }
}

fn main() {
    let mut file = File::open("input").unwrap();
    let mut input_string = String::new();
    file.read_to_string(&mut input_string).unwrap();
    let mut grid = Grid::from(&input_string);
}

#[test]
fn test_visibility1() {
    let input_str = ".......#.\n...#.....\n.#.......\n.........\n..#L....#\n....#....\n.........\n#........\n...#.....";
    let grid = Grid::from(input_str);
    assert_eq!(
        grid.visibility[(3, 4)],
        HashSet::<(usize, usize)>::from_iter(
            [
                (7, 0),
                (3, 1),
                (1, 2),
                (2, 4),
                (8, 4),
                (4, 5),
                (0, 7),
                (3, 8),
            ]
            .iter()
            .cloned(),
        ),
    );
}

#[test]
fn test_visibility2() {
    let input_str = ".............\n.L.L.#.#.#.#.\n.............";
    let grid = Grid::from(input_str);
    assert_eq!(
        grid.visibility[(1, 1)],
        HashSet::<(usize, usize)>::from_iter([(3, 1),].iter().cloned(),),
    );
}

#[test]
fn test_visibility3() {
    let input_str = ".##.##.\n#.#.#.#\n##...##\n...L...\n##...##\n#.#.#.#\n.##.##.";
    let grid = Grid::from(input_str);
    assert_eq!(grid.visibility[(3, 3)], HashSet::<(usize, usize)>::new());
}

#[test]
fn test_part1_on_input() {
    let mut file = File::open("input").unwrap();
    let mut input_string = String::new();
    file.read_to_string(&mut input_string).unwrap();
    let mut grid = Grid::from(&input_string);

    while grid.transition_part1() != 0 {}

    assert_eq!(
        grid.cells
            .iter()
            .filter(|&state| *state == State::Occupied)
            .count(),
        2329
    );
}

#[test]
fn test_getting_neighbors_part1() {
    let input_str = "L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL\n";
    let grid = Grid::from(input_str);
    assert_eq!(
        grid.get_adjacent_positions(&(0, 0)),
        HashSet::<(usize, usize)>::from_iter([(0, 1), (1, 0), (1, 1)].iter().cloned())
    );
    assert_eq!(
        grid.get_adjacent_positions(&(3, 1)),
        HashSet::<(usize, usize)>::from_iter(
            [
                (2, 0),
                (2, 1),
                (2, 2),
                (3, 0),
                (3, 2),
                (4, 0),
                (4, 1),
                (4, 2)
            ]
            .iter()
            .cloned()
        )
    );
}

#[test]
fn test_part1_example() {
    let input_str = "L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL\n";
    let mut grid = Grid::from(input_str);
    let mut transitions_counter = 0;
    println!("State after {} transitions:\n{}", transitions_counter, grid);

    while grid.transition_part1() != 0 {
        transitions_counter += 1;
        println!("State after {} transitions:\n{}", transitions_counter, grid);
    }

    assert_eq!(
        grid.cells
            .iter()
            .filter(|&state| *state == State::Occupied)
            .count(),
        37
    );
}

#[test]
fn test_grid_from_str_and_formatting() {
    let input_str = "L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL\n";
    let grid = Grid::from(input_str);
    assert_eq!(grid.width, 10);
    assert_eq!(grid.height, 10);
    println!("{}", grid);
    //assert_eq!(false, true);
}
