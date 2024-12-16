use image::{DynamicImage, GenericImage};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::{thread, usize};

use crate::{define_solution, sprites};

define_solution!(DaySix, part_one, part_two);

use crate::twentytwentyfour::six::Direction::{East, North, South, West};
#[derive(Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn(self) -> Self {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
}
type Pos = (i32, i32);

#[derive(Clone)]
struct Guard {
    position: Pos,
    direction: Direction,
}

#[derive(Clone)]
enum Cell {
    Blank,
    Obstruction,
    Visited,
}

struct Board {
    cells: Vec<Vec<Cell>>,
    guard: Guard,
    visited: HashSet<(i32, i32)>,
    walk_counter: i32,
    queue: Vec<PartialBoard>,
}

#[derive(Clone)]
struct PartialBoard {
    cells: Vec<Vec<Cell>>,
    guard: Guard,
    walk_counter: i32,
}

impl Board {
    fn new(lines: Vec<String>, queue: Vec<PartialBoard>) -> Self {
        let mut guard = None;
        let cells = lines
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '#' => Cell::Obstruction,
                        '.' => Cell::Blank,
                        _ => {
                            guard = Some(Guard {
                                position: (x as i32, y as i32),
                                direction: North,
                            });
                            Cell::Visited
                        }
                    })
                    .collect()
            })
            .collect::<Vec<Vec<_>>>();

        Board {
            guard: guard.unwrap(),
            cells,
            visited: HashSet::new(),
            walk_counter: 0,
            queue,
        }
    }
    fn walk(&mut self) -> bool {
        let (x, y) = self.guard.position;
        let direction = &self.guard.direction;
        let height = self.cells.len() as i32;
        let width = self.cells[0].len() as i32;

        match direction {
            North if y - 1 < 0 => return false,
            South if y + 1 >= height => return false,
            East if x + 1 >= width => return false,
            West if x - 1 < 0 => return false,
            _ => {}
        };

        match direction {
            North => self.visit_cell(x, y - 1),
            East => self.visit_cell(x + 1, y),
            South => self.visit_cell(x, y + 1),
            West => self.visit_cell(x - 1, y),
        };
        true
    }

    fn solve(&mut self) -> usize {
        while self.walk() {}
        self.visited.len()
    }

    fn visit_cell(&mut self, x: i32, y: i32) {
        match self.cells[y as usize][x as usize] {
            Cell::Obstruction => self.guard.direction = self.guard.direction.turn(),
            Cell::Blank | Cell::Visited => {
                self.walk_counter += 1;
                self.visited.insert((x, y));
                self.guard.position = (x, y);
                self.cells[y as usize][x as usize] = Cell::Visited;
                self.queue.push(PartialBoard {
                    cells: self.cells.clone(),
                    guard: self.guard.clone(),
                    walk_counter: self.walk_counter,
                });
            }
        }
    }
}

fn generate_image(board: PartialBoard, image: &mut DynamicImage, sprites: &Vec<DynamicImage>) {
    let sprite_size = 32;
    let empty_tile = &sprites[0];
    let guard = &sprites[1];
    let obstruction = &sprites[2];
    let visited = &sprites[3];
    for (y, row) in board.cells.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            let sprite = if (x as i32, y as i32) == board.guard.position {
                match board.guard.direction {
                    North => guard.clone(),
                    East => guard.clone().rotate90(),
                    South => guard.clone().rotate180(),
                    West => guard.clone().rotate270(),
                }
            } else {
                match tile {
                    Cell::Blank => empty_tile.clone(),
                    Cell::Visited => visited.clone(),
                    Cell::Obstruction => obstruction.clone(),
                }
            };
            match image.copy_from(&sprite, x as u32 * sprite_size, y as u32 * sprite_size) {
                Ok(_) => {}
                Err(e) => println!("Error: {:?}", e),
            }
        }
    }
    image
        .save(format!("frames/out_{}.png", board.walk_counter))
        .unwrap();
}

fn part_one(lines: Vec<String>) -> String {
    let queue = Vec::new();
    let mut board = Board::new(lines, queue);
    let solution = board.solve().to_string();
    //let queue = Arc::new(Mutex::new(board.queue));
    //let handles: Vec<_> = (0..=4)
    //    .map(|_| {
    //        let queue_copy = Arc::clone(&queue);
    //        let height = board.cells.len();
    //        let width = board.cells[0].len();
    //        thread::spawn(move || {
    //            let sprite_size = 32;
    //            let sprites = sprites::load_sprite("sprites/guard.png", sprite_size, 1, 4).unwrap();
    //            let mut image = image::DynamicImage::new_rgba8(
    //                width as u32 * sprite_size,
    //                height as u32 * sprite_size,
    //            );
    //            loop {
    //                let board = {
    //                    let mut lock = queue_copy.lock().unwrap();
    //                    let msg = lock.pop();
    //                    msg
    //                };
    //                match board {
    //                    Some(board) => generate_image(board, &mut image, &sprites),
    //                    None => return,
    //                }
    //            }
    //        })
    //    })
    //    .collect();
    //for handle in handles {
    //    handle
    //        .join()
    //        .unwrap_or_else(|_| println!("Error with thread"))
    //}
    solution
}

fn part_two(lines: Vec<String>) -> String {
    "".into()
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[tokio::test]
    async fn test_part_one() {
        let lines = utils::read_db(6, 2024, &"example".into()).await;
        assert_eq!(part_one(lines), "41");
    }

    #[tokio::test]
    async fn test_part_two() {}
}
