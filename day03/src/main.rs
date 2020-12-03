use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::ops::Index;

#[derive(Debug, Clone, std::cmp::PartialEq)]
pub enum Cell {
    Empty,
    Tree,
}

pub type Point2D = (usize, usize);

#[derive(Debug, std::cmp::PartialEq)]
pub struct Grid {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn from_string(grid_string: &str) -> Grid {
        let mut cells = Vec::new();
        let mut width = 0;
        let mut height = 0;

        for grid_line in grid_string.lines() {
            height += 1;
            let mut line_length = 0;
            for cell_char in grid_line.chars() {
                line_length += 1;
                match cell_char {
                    '.' => cells.push(Cell::Empty),
                    '#' => cells.push(Cell::Tree),
                    _ => assert!(false),
                }
            }
            assert!(width == 0 || width == line_length);
            width = line_length;
        }

        Grid {
            cells,
            width,
            height,
        }
    }

    pub fn cast_ray(self, start: Point2D, direction: Point2D) -> Ray {
        Ray {
            grid: self,
            current: start,
            direction,
        }
    }
}

impl Index<Point2D> for Grid {
    type Output = Cell;
    fn index(&self, point: Point2D) -> &Self::Output {
        &self.cells[point.0 + self.width * point.1]
    }
}

pub struct Ray {
    grid: Grid,
    current: Point2D,
    direction: Point2D,
}

impl Iterator for Ray {
    type Item = Cell;
    fn next(&mut self) -> Option<Self::Item> {
        self.current.0 = (self.current.0 + self.direction.0) % self.grid.width;
        self.current.1 = self.current.1 + self.direction.1;
        match self.current.1 >= self.grid.height {
            true => None,
            false => Some(self.grid[self.current].clone()),
        }
    }
}

fn main() -> io::Result<()> {
    let mut file = File::open("input").unwrap();
    let mut input_string = String::new();
    file.read_to_string(&mut input_string)?;

    let mut trees_product = 1;

    for direction in &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        let grid = Grid::from_string(&input_string);
        let nr_trees_encountered = grid
            .cast_ray((0, 0), *direction)
            .filter(|c| *c == Cell::Tree)
            .count();
        println!(
            "Direction {:?}: Found {} trees on the way.",
            direction, nr_trees_encountered
        );
        trees_product *= nr_trees_encountered;
    }
    println!("Product: {}", trees_product);

    Ok(())
}

#[test]
fn test_ray_cast() {
    let grid = Grid::from_string(".#..\n#.#.\n..#.\n");
    let visited_cells: Vec<Cell> = grid.cast_ray((0, 0), (1, 1)).collect();
    assert_eq!(visited_cells, [Cell::Empty, Cell::Tree]);

    // example from aoc website
    let grid = Grid::from_string("..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#\n");
    assert_eq!(
        grid.cast_ray((0, 0), (3, 1))
            .filter(|c| *c == Cell::Tree)
            .count(),
        7
    );
}

#[test]
fn test_grid_access() {
    assert_eq!(Grid::from_string("#")[(0, 0)], Cell::Tree);
    let grid = Grid::from_string(".#..\n#.#.\n..#.\n");
    assert_eq!(grid[(0, 0)], Cell::Empty);
    assert_eq!(grid[(1, 0)], Cell::Tree);
    assert_eq!(grid[(2, 0)], Cell::Empty);
    assert_eq!(grid[(0, 1)], Cell::Tree);
    assert_eq!(grid[(2, 2)], Cell::Tree);
    assert_eq!(grid[(3, 2)], Cell::Empty);
}

#[test]
fn test_grid_from_string() {
    assert_eq!(
        Grid::from_string(""),
        Grid {
            cells: [].to_vec(),
            width: 0,
            height: 0
        }
    );
    assert_eq!(
        Grid::from_string("."),
        Grid {
            cells: [Cell::Empty].to_vec(),
            width: 1,
            height: 1
        }
    );
    assert_eq!(
        Grid::from_string("#"),
        Grid {
            cells: [Cell::Tree].to_vec(),
            width: 1,
            height: 1
        }
    );
    assert_eq!(
        Grid::from_string(".#..\n#.#.\n..#.\n"),
        Grid {
            cells: [
                Cell::Empty,
                Cell::Tree,
                Cell::Empty,
                Cell::Empty,
                Cell::Tree,
                Cell::Empty,
                Cell::Tree,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Tree,
                Cell::Empty
            ]
            .to_vec(),
            width: 4,
            height: 3
        }
    );
}
