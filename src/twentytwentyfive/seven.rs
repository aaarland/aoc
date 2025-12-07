use std::{
    collections::HashMap,
    fmt::{self, Display},
    io,
};

use crate::{define_solution_with_callback, solutions::UpdateFn};
define_solution_with_callback!(DaySeven, part_one, part_two);

#[derive(Debug, PartialEq, Copy, Clone)]
enum Tile {
    Empty,
    Splitter,
    BeamEntry,
    Beam,
}
impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = self.to_string();
        write!(f, "{}", c)
    }
}
impl Into<char> for Tile {
    fn into(self) -> char {
        let c: char = match self {
            Empty => '.',
            BeamEntry => 'S',
            Beam => '|',
            Splitter => '^',
        };
        c
    }
}
use Tile::*;
impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Empty,
            'S' => BeamEntry,
            '|' => Beam,
            '^' => Splitter,
            _ => panic!("Not a valid tile"),
        }
    }
}
struct Tachyon<'a> {
    manifold: Vec<Vec<Tile>>,
    update: Option<UpdateFn<'a>>,
}
impl<'a> fmt::Debug for Tachyon<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tachyon")
            .field("manifold", &self.manifold)
            .finish()
    }
}
impl<'a> Tachyon<'a> {
    fn new(lines: Vec<String>, update: Option<UpdateFn<'a>>) -> Self {
        Self {
            manifold: lines
                .iter()
                .map(|next| next.chars().map(|c| Tile::from(c)).collect())
                .collect(),
            update,
        }
    }
    fn render(&mut self) -> Result<(), io::Error> {
        if let Some(update) = &mut self.update {
            update(
                self.manifold
                    .iter()
                    .map(|row| row.iter().map(|tile| -> char {Into::into(*tile)}).collect())
                    .collect(),
            );
        }
        Ok(())
    }
}

fn shoot_beam(tachyon: &mut Tachyon, x: isize, y: isize, counter: usize) -> usize {
    if y < 0
        || y as usize >= tachyon.manifold.len()
        || x < 0
        || x as usize >= tachyon.manifold[y as usize].len()
    {
        return counter;
    }
    let current = &tachyon.manifold[y as usize][x as usize];

    if *current == Beam {
        return counter;
    }

    if *current == Splitter {
        return 1 + shoot_beam(tachyon, x - 1, y, counter) + shoot_beam(tachyon, x + 1, y, counter);
    }

    let _ = tachyon.render();
    tachyon.manifold[y as usize][x as usize] = Beam;
    return shoot_beam(tachyon, x, y + 1, counter);
}
fn part_one(lines: Vec<String>, update: Option<UpdateFn>) -> String {
    let mut tachyon = Tachyon::new(lines, update);
    let y = tachyon
        .manifold
        .iter()
        .position(|tiles| tiles.iter().any(|tile| *tile == BeamEntry))
        .unwrap();
    let x = tachyon.manifold[y]
        .iter()
        .position(|tile| *tile == BeamEntry)
        .unwrap();
    let final_count = shoot_beam(&mut tachyon, x as isize, y as isize, 0);
    // sleep(Duration::from_secs(10));
    final_count.to_string()
}

fn shoot_beam_part_two(
    tachyon: &mut Tachyon,
    x: isize,
    y: isize,
    counter: usize,
    memo: &mut HashMap<(isize, isize), usize>,
) -> usize {
    let _ = tachyon.render();
    if let Some(current_count) = memo.get(&(y, x)) {
        return counter + current_count;
    }
    if y < 0
        || y as usize >= tachyon.manifold.len()
        || x < 0
        || x as usize >= tachyon.manifold[y as usize].len()
    {
        memo.insert((y, x), counter + 1);
        return counter + 1;
    }
    let current = &tachyon.manifold[y as usize][x as usize];

    if *current == Splitter {
        let result = shoot_beam_part_two(tachyon, x - 1, y, counter, memo)
            + shoot_beam_part_two(tachyon, x + 1, y, counter, memo);
        memo.insert((y, x), result);
        return result;
    }

    tachyon.manifold[y as usize][x as usize] = Beam;
    let result = shoot_beam_part_two(tachyon, x, y + 1, counter, memo);
    memo.insert((y, x), result);
    return result;
}
fn part_two(lines: Vec<String>, update: Option<UpdateFn>) -> String {
    let mut tachyon = Tachyon::new(lines, update);
    let y = tachyon
        .manifold
        .iter()
        .position(|tiles| tiles.iter().any(|tile| *tile == BeamEntry))
        .unwrap();
    let x = tachyon.manifold[y]
        .iter()
        .position(|tile| *tile == BeamEntry)
        .unwrap();
    let final_count =
        shoot_beam_part_two(&mut tachyon, x as isize, y as isize, 0, &mut HashMap::new());
    // sleep(Duration::from_secs(10));
    final_count.to_string()
}
