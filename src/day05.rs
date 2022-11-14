//! Day 05
//!
//! # Problem:
//!
//! Given a field which positions can be represented by uint numbers, process the lines given in the input file and
//! determine:
//!
//! 1. the points which at least two vertical or horizontal lines overlap;
//!
//! 2. the points which at least two lines overlap (vertical, horizontal or diagonal).
use crate::helpers::{
    base2d::Base2d,
    grid::Grid,
    read,
    utils::{self, delta::Delta},
};
type Line<'a> = (&'a Base2d<u16>, &'a Base2d<u16>);

pub fn run() {
    let input: Vec<_> = parse_input().collect();

    // calculates max lenghs to create a grid
    let (len_x, len_y) = input.iter().fold((0, 0), |(lx, ly), b| (lx.max(b.x), ly.max(b.y)));

    // +1 because initial position is (0, 0)
    let mut grid = Grid::new((len_x + 1) as usize, (len_y + 1) as usize, 0u16);
    let count_overlaps_straight = overlaps_straight_lines(&mut grid, &input);
    let count_overlaps_all = overlaps_diagonal_lines(&mut grid, &input);

    println!("Day 05");
    println!("Count of overlaps for straight lines: {}", count_overlaps_straight);
    println!("Count of overlaps for all lines: {}", count_overlaps_all);
    println!();
}

/// Fills a grid using only straight lines and returns the number of positions where at least two lines intercept.
///
/// #Inputs
/// `grid`: a `Grid` which uses `u16` variables an a (x, y) coordinate systems.
/// `input`: a list of points `Base2d` where each two of them represents a line.
fn overlaps_straight_lines(grid: &mut Grid<u16>, input: &[Base2d<u16>]) -> usize {
    fill_grid(grid, input, false);
    grid.iter().filter(|&&v| v > 1).count()
}

/// Fills a grid using only diagonal lines and returns the number of positions where at least two lines intercept.
///
/// #Inputs
/// `grid`: a `Grid` which uses `u16` variables an a (x, y) coordinate systems.
/// `input`: a list of points `Base2d` where each two of them represents a line.
fn overlaps_diagonal_lines(grid: &mut Grid<u16>, input: &[Base2d<u16>]) -> usize {
    fill_grid(grid, input, true);
    grid.iter().filter(|&&v| v > 1).count()
}

/// returns an iterator yielding a point
fn parse_input() -> impl Iterator<Item = Base2d<u16>> {
    let input = read::file_to_lines("day05").unwrap();

    input
        .flatten()
        .map(|s| {
            let inner: Vec<_> = s.split(" -> ").map(str::parse::<Base2d<u16>>).flatten().collect();
            inner.into_iter()
        })
        .flatten()
}

//--------------------------------------------------------------------
// Helpers
//--------------------------------------------------------------------

/// method for filling the grid by straight lines (columns and rows) or diagonal lines (at 45°).
/// Each pair at the `input` list defines a line and `go_diagonal` flag defines if the function will
/// fill the grid by the diagonal lines (`true`) or straight lines (`false`).
fn fill_grid(grid: &mut Grid<u16>, input: &[Base2d<u16>], go_diagonal: bool) {
    let lines = utils::pairs_zip(input);
    for (p0, p1) in lines {
        if !go_diagonal {
            if p0.is_same_column(p1) {
                fill_column(grid, p0, p1);
            } else if p0.is_same_row(&p1) {
                fill_row(grid, p0, p1);
            }
        } else if !p0.is_same_column(p1) && !p0.is_same_row(&p1) {
            fill_diagonal(grid, p0, p1);
        }
    }
}

/// helper method to update a column line in the grid based on the coordinates given by p0 and p1.
/// The x coordinate from p0 is soleny used the determine the column.
fn fill_column(grid: &mut Grid<u16>, p0: &Base2d<u16>, p1: &Base2d<u16>) {
    let x = p0.x;
    let (y0, y1) = utils::min_max(p0.y, p1.y);
    (y0..=y1).for_each(|y| *grid.get_mut(x.into(), y.into()) += 1);
}

/// helper method to update a row line in the grid based on the coordinates given by p0 and p1. The y coordinate from p0
/// is soleny used the determine the column.
fn fill_row(grid: &mut Grid<u16>, p0: &Base2d<u16>, p1: &Base2d<u16>) {
    let y = p0.y;
    let (x0, x1) = utils::min_max(p0.x, p1.x);
    (x0..=x1).for_each(|x| *grid.get_mut(x.into(), y.into()) += 1);
}

/// helper method to update diagonal lines in the grid (diagonals at 45°), based on the coordinates given by points p0
/// and p1.
fn fill_diagonal(grid: &mut Grid<u16>, p0: &Base2d<u16>, p1: &Base2d<u16>) {
    let dx = Delta::new(p0.x, p1.x, 1);
    let dy = Delta::new(p0.y, p1.y, 1);

    let (mut x, mut y) = p0.tuple();
    *grid.get_mut(x.into(), y.into()) += 1;
    loop {
        x += dx;
        y += dy;
        *grid.get_mut(x.into(), y.into()) += 1;
        if (x, y) == p1.tuple() {
            break;
        }
    }
}

/// from a iterator over points returns a iterator of pairs representing lines.
//? make a test using chunks_exact
fn pairs(input: &[Base2d<u16>]) -> impl Iterator<Item = Line> {
    let iter1 = input.iter().step_by(2);
    let iter2 = input.iter().skip(1).step_by(2);
    iter1.zip(iter2)
}

//--------------------------------------------------------------------
// Tests and Benches
//--------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref TEST_INPUT: Vec<Base2d<u16>> = vec![
            Base2d::new(0, 9),
            Base2d::new(5, 9),
            Base2d::new(8, 0),
            Base2d::new(0, 8),
            Base2d::new(9, 4),
            Base2d::new(3, 4),
            Base2d::new(2, 2),
            Base2d::new(2, 1),
            Base2d::new(7, 0),
            Base2d::new(7, 4),
            Base2d::new(6, 4),
            Base2d::new(2, 0),
            Base2d::new(0, 9),
            Base2d::new(2, 9),
            Base2d::new(3, 4),
            Base2d::new(1, 4),
            Base2d::new(0, 0),
            Base2d::new(8, 8),
            Base2d::new(5, 5),
            Base2d::new(8, 2),
        ];
    }

    #[test]
    fn test_overlaps() {
        let mut grid = Grid::new(10, 10, 0u16);
        let ans1 = overlaps_straight_lines(&mut grid, &TEST_INPUT);
        assert_eq!(ans1, 5);
        let ans2 = overlaps_diagonal_lines(&mut grid, &TEST_INPUT);
        assert_eq!(ans2, 12);
    }
}
