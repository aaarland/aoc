use std::{
    collections::HashMap, fmt::Display, io::{self, Stdout, Write, stdout}, thread::{self, sleep}, time::Duration
};

use crate::define_solution;
use crossterm::{
    ExecutableCommand,
    cursor::{Hide, MoveTo, Show},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
define_solution!(DaySeven, part_one, part_two);

#[derive(Debug, PartialEq)]
enum Tile {
    Empty,
    Splitter,
    BeamEntry,
    Beam,
}
impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c: char = match self {
            Empty => '.',
            BeamEntry => 'S',
            Beam => '|',
            Splitter => '^',
        };
        write!(f, "{}", c)
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
#[derive(Debug)]
struct Tachyon {
    manifold: Vec<Vec<Tile>>,
    stdout: Stdout,
}
impl From<Vec<String>> for Tachyon {
    fn from(value: Vec<String>) -> Self {
        let mut stdout = stdout();

        stdout.execute(EnterAlternateScreen).unwrap();
        stdout.execute(Hide).unwrap();
        // Clear the buffer fully once
        stdout.execute(Clear(ClearType::All)).unwrap();
        Self {
            manifold: value
                .iter()
                .map(|next| next.chars().map(|c| Tile::from(c)).collect())
                .collect(),
            stdout,
        }
    }
}
impl Drop for Tachyon {
    fn drop(&mut self) {
        self.stdout.execute(Show).unwrap();
        self.stdout.execute(LeaveAlternateScreen).unwrap();
    }
}
impl Tachyon {
    fn render(&mut self) -> Result<(), io::Error> {
        self.stdout.execute(Clear(ClearType::All))?;
        self.stdout.execute(MoveTo(0, 0))?;
        for (y, row) in self.manifold.iter().enumerate() {
            self.stdout.execute(MoveTo(0, y as u16))?;
            for ch in row {
                write!(self.stdout, "{}", ch)?;
            }
        }
        thread::sleep(Duration::from_millis(1));
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

    // let _ = tachyon.render();
    tachyon.manifold[y as usize][x as usize] = Beam;
    return shoot_beam(tachyon, x, y + 1, counter);
}
fn part_one(lines: Vec<String>) -> String {
    let mut tachyon = Tachyon::from(lines);
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

fn shoot_beam_part_two(tachyon: &mut Tachyon, x: isize, y: isize, counter: usize, memo: &mut HashMap<(isize, isize), usize>) -> usize {
    let _ = tachyon.render();
    if let Some(current_count) = memo.get(&(y, x)) {
        return counter + current_count;
    }
    if y < 0
        || y as usize >= tachyon.manifold.len()
        || x < 0
        || x as usize >= tachyon.manifold[y as usize].len()
    {
        memo.insert((y,x), counter + 1);
        return counter + 1;
    }
    let current = &tachyon.manifold[y as usize][x as usize];

    if *current == Splitter {
        let result =shoot_beam_part_two(tachyon, x - 1, y, counter, memo) + shoot_beam_part_two(tachyon, x + 1, y, counter, memo);
        memo.insert((y,x), result);
        return result;
    }

    tachyon.manifold[y as usize][x as usize] = Beam;
    let result =shoot_beam_part_two(tachyon, x, y + 1, counter, memo);
    memo.insert((y,x), result);
    return result;
}
fn part_two(lines: Vec<String>) -> String {
    let mut tachyon = Tachyon::from(lines);
    let y = tachyon
        .manifold
        .iter()
        .position(|tiles| tiles.iter().any(|tile| *tile == BeamEntry))
        .unwrap();
    let x = tachyon.manifold[y]
        .iter()
        .position(|tile| *tile == BeamEntry)
        .unwrap();
    let final_count = shoot_beam_part_two(&mut tachyon, x as isize, y as isize, 0, &mut HashMap::new());
    // sleep(Duration::from_secs(10));
    final_count.to_string()
}
