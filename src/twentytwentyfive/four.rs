use crate::define_solution;

define_solution!(DayFour, part_one, part_two);

#[derive(PartialEq, Debug)]
enum Tile {
    Roll,
    Empty,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '@' => Self::Roll,
            _ => Self::Empty,
        }
    }
}

fn is_out_of_bounds(tiles: &Vec<Vec<Tile>>, i: i32, j: i32) -> bool {
    i < 0 || j < 0 || i as usize >= Vec::len(&tiles) || j as usize >= tiles[i as usize].len()
}

fn check_tile(tiles: &Vec<Vec<Tile>>, i: i32, j: i32) -> bool {
    if is_out_of_bounds(tiles, i, j) || tiles[i as usize][j as usize] == Tile::Empty {
        false
    } else {
        let mut rolls = 0;
        for x in i - 1..=i + 1 {
            for y in j - 1..=j + 1 {
                if !(x == i && y == j)
                    && !is_out_of_bounds(tiles, x, y)
                    && tiles[x as usize][y as usize] == Tile::Roll
                {
                    rolls += 1;
                }
            }
        }
        rolls < 4
    }
}

fn part_one(lines: Vec<String>) -> String {
    let tiles: Vec<Vec<Tile>> = lines
        .into_iter()
        .map(|line| line.chars().map(|ch| Tile::from(ch)).collect())
        .collect();
    tiles
        .iter()
        .enumerate()
        .fold(0, |acc, (i, row)| {
            acc + (0..row.len()).fold(0, |inner_acc, j| {
                if check_tile(&tiles, i as i32, j as i32) {
                    inner_acc + 1
                } else {
                    inner_acc
                }
            })
        })
        .to_string()
}

fn part_two(_lines: Vec<String>) -> String {
    "".into()
}
