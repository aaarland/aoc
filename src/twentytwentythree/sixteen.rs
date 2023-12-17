use image::{DynamicImage, GenericImage};

use crate::{solutions::Solution, sprites};
pub struct DaySixteen;

impl Solution for DaySixteen {
    fn solve(&self, lines: Vec<String>) -> () {
        let time = std::time::Instant::now();
        let part_one = part_one(lines.clone());
        let elapsed = time.elapsed();
        println!("Part one: {:?}", part_one);
        println!("Part one time: {:?}", elapsed);
    }
}

#[derive(Debug, Clone)]
enum Tiles {
    Empty,
    MirrorForward,
    MirrorBackward,
    SplitVertical,
    SplitHorizontal,
}

#[derive(Debug, Clone, PartialEq, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, PartialEq)]
struct Beam {
    x: i32,
    y: i32,
    current_direction: Direction,
}

struct Beams {
    beams: Vec<Beam>,
    visited: Vec<Vec<Vec<Direction>>>,
    grid: Vec<Vec<Tiles>>,
}

impl Beams {
    fn new(grid: Vec<Vec<Tiles>>) -> Self {
        Self {
            beams: vec![Beam::new(-1, 0, None)],
            visited: vec![vec![vec![]; grid[0].len()]; grid.len()],
            grid,
        }
    }
    fn traverse_grid(&mut self) -> i32 {
        loop {
            let mut new_beams = Vec::new();
            let mut remove_beams = Vec::new();
            for beam in self.beams.iter_mut() {
                beam.move_forward();
                if beam.x >= self.grid[0].len().try_into().unwrap()
                    || beam.y >= self.grid.len().try_into().unwrap()
                    || beam.x < 0
                    || beam.y < 0
                {
                    remove_beams.push(beam.clone());
                    continue;
                }
                if self.visited[beam.y as usize][beam.x as usize].contains(&beam.current_direction)
                {
                    remove_beams.push(beam.clone());
                    continue;
                }
                self.visited[beam.y as usize][beam.x as usize].push(beam.current_direction.clone());
                let x = beam.x as usize;
                let y = beam.y as usize;
                let tile = &self.grid[y][x];
                match tile {
                    Tiles::Empty => {}
                    Tiles::MirrorForward => match beam.current_direction {
                        Direction::North => {
                            beam.current_direction = Direction::East;
                        }
                        Direction::East => {
                            beam.current_direction = Direction::North;
                        }
                        Direction::South => {
                            beam.current_direction = Direction::West;
                        }
                        Direction::West => {
                            beam.current_direction = Direction::South;
                        }
                    },
                    Tiles::MirrorBackward => match beam.current_direction {
                        Direction::North => {
                            beam.current_direction = Direction::West;
                        }
                        Direction::East => {
                            beam.current_direction = Direction::South;
                        }
                        Direction::South => {
                            beam.current_direction = Direction::East;
                        }
                        Direction::West => {
                            beam.current_direction = Direction::North;
                        }
                    },
                    Tiles::SplitVertical => match beam.current_direction {
                        Direction::East => {
                            beam.current_direction = Direction::North;
                            new_beams.push(Beam::new(beam.x, beam.y, Some(Direction::South)));
                        }
                        Direction::West => {
                            beam.current_direction = Direction::South;
                            new_beams.push(Beam::new(beam.x, beam.y, Some(Direction::North)));
                        }
                        _ => {}
                    },
                    Tiles::SplitHorizontal => match beam.current_direction {
                        Direction::North => {
                            beam.current_direction = Direction::East;
                            new_beams.push(Beam::new(beam.x, beam.y, Some(Direction::West)));
                        }
                        Direction::South => {
                            beam.current_direction = Direction::West;
                            new_beams.push(Beam::new(beam.x, beam.y, Some(Direction::East)));
                        }
                        _ => {}
                    },
                }
            }
            for beam in remove_beams {
                let index = self
                    .beams
                    .iter()
                    .position(|current_beam| *current_beam == beam)
                    .unwrap();
                self.beams.remove(index);
            }
            for beam in new_beams {
                self.beams.push(beam);
            }
            if self.beams.len() == 0 {
                break;
            }
        }
        // generate_image(self.grid.clone(), self.visited.clone());
        self.visited
            .iter()
            .map(|row| {
                row.iter()
                    .map(|col| if col.len() > 0 { 1 } else { 0 })
                    .sum::<usize>() as i32
            })
            .sum()
    }
}

impl Beam {
    fn new(x: i32, y: i32, direction: Option<Direction>) -> Self {
        Self {
            x,
            y,
            current_direction: direction.unwrap_or(Direction::East),
        }
    }
    fn move_forward(&mut self) {
        match self.current_direction {
            Direction::North => {
                self.y -= 1;
            }
            Direction::East => {
                self.x += 1;
            }
            Direction::South => {
                self.y += 1;
            }
            Direction::West => {
                self.x -= 1;
            }
        }
    }
}

impl From<char> for Tiles {
    fn from(c: char) -> Self {
        match c {
            '.' => Tiles::Empty,
            '/' => Tiles::MirrorForward,
            '\\' => Tiles::MirrorBackward,
            '|' => Tiles::SplitVertical,
            '-' => Tiles::SplitHorizontal,
            _ => panic!("Unknown tile type"),
        }
    }
}

fn generate_image(tiles: Vec<Vec<Tiles>>, visited: Vec<Vec<Vec<Direction>>>) {
    let sprite_size = 32;
    let sprites = sprites::load_sprite("sprites/lasers.png", sprite_size, 3, 3).unwrap();
    let height = tiles.len();
    let width = tiles[0].len();
    let mut image =
        image::DynamicImage::new_rgba8(width as u32 * sprite_size, height as u32 * sprite_size);
    let empty_tile = &sprites[0];
    let mirror_forward = &sprites[2];
    let mirror_backward = sprites[2].rotate90();
    let split_vertical = sprites[4].rotate90();
    let split_horizontal = &sprites[4];

    for (y, row) in tiles.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            let sprite = match tile {
                Tiles::Empty => empty_tile.clone(),
                Tiles::MirrorForward => mirror_forward.clone(),
                Tiles::MirrorBackward => mirror_backward.clone(),
                Tiles::SplitVertical => split_vertical.clone(),
                Tiles::SplitHorizontal => split_horizontal.clone(),
            };
            let sprite = match visited[y][x].len() {
                0 => sprite,
                _ => match_tile(tile, visited[y][x][0], sprites.clone()),
            };
            match image.copy_from(&sprite, x as u32 * sprite_size, y as u32 * sprite_size) {
                Ok(_) => {}
                Err(e) => println!("Error: {:?}", e),
            }
        }
    }
    image.save("output.png").unwrap();
}

fn match_tile(tile: &Tiles, direction: Direction, sprites: Vec<DynamicImage>) -> DynamicImage {
    let laser = sprites[1].clone();
    let mirror_forward_laser = sprites[3].clone();
    let mirror_backward_laser = sprites[3].rotate90();
    let split_laser = sprites[5].clone();
    let no_split_laser = sprites[6].clone();
    match direction {
        Direction::North => match tile {
            Tiles::Empty => laser.rotate90(),
            Tiles::MirrorForward => mirror_forward_laser.rotate180(),
            Tiles::MirrorBackward => mirror_backward_laser.rotate180(),
            Tiles::SplitVertical => no_split_laser.rotate90(),
            Tiles::SplitHorizontal => split_laser.rotate180(),
        },
        Direction::East => match tile {
            Tiles::Empty => laser,
            Tiles::MirrorForward => mirror_forward_laser,
            Tiles::MirrorBackward => mirror_backward_laser.rotate90(),
            Tiles::SplitVertical => split_laser.rotate90(),
            Tiles::SplitHorizontal => no_split_laser,
        },
        Direction::South => match tile {
            Tiles::Empty => laser.rotate90(),
            Tiles::MirrorForward => mirror_forward_laser,
            Tiles::MirrorBackward => mirror_backward_laser,
            Tiles::SplitVertical => no_split_laser.rotate90(),
            Tiles::SplitHorizontal => split_laser,
        },
        Direction::West => match tile {
            Tiles::Empty => laser,
            Tiles::MirrorForward => mirror_forward_laser,
            Tiles::MirrorBackward => mirror_backward_laser.rotate180(),
            Tiles::SplitVertical => no_split_laser,
            Tiles::SplitHorizontal => split_laser.rotate270(),
        },
    }
}

fn part_one(lines: Vec<String>) -> i32 {
    let tiles = lines
        .iter()
        .map(|line| line.chars().map(|c| c.into()).collect::<Vec<Tiles>>())
        .collect::<Vec<Vec<Tiles>>>();

    let mut beams = Beams::new(tiles);
    beams.traverse_grid()
}

fn part_two() {}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn test_part_one() {
        let lines = utils::read_file(&"2023/example16".into());
        assert_eq!(part_one(lines), 46);
    }

    #[test]
    fn test_part_two() {}
}
