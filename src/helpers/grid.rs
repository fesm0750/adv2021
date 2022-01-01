//! A two-dimensional array using a flat internal representation.
//!
//! This is a row major implementation, consecutive elements across the x
//! dimension are next to each other, whereas columns are strided.
//!
//! `x` represents variation in row elements (which column the value is in),
//! whereas `y` represents a change in column elements (which row is it in). The
//! grid may be indexed with a tuple using `get_from2d((x,y))` , for example:
//!
//! - get_from2d((5, 0)) returns the sixth element of the first row. It can also be interpreted as
//!   the the element at column 5 and row 0.
//!
//! - get_from2d((1, 5)) returns the second element of the sixth row. In other words, the element at
//!   column 1 and row 5.
//!
//! # Indexing
//!
//! Implements the Index trait, so the grid may be read by a tuple inside
//! square brackets. Example:
//!
//! ```
//! use adv20::helpers::grid::Grid;
//! let mut grid = Grid::new(5, 5, 0u8);
//! let v = grid.get_mut(2, 2);
//! *v = 100;
//! assert_eq!(grid[(2,2)], 100);
//! ```
//!
//! ## Beware
//!
//! If no inferring is made, the Default type for tuples in rust is i32.
//!
//! # Panics
//!
//! Panics if the indexing inside square brackets is done with negative values.

use std::ops::AddAssign;

// use std::{convert::TryInto, fmt::Debug, ops::Index};

// use super::base2d::Base2d;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Grid<T> {
    flat: Vec<T>,
    pub len_x: usize,
    pub len_y: usize,
}

impl<T: Clone> Grid<T> {
    /// creates a new grid with al the elements having the `init`ial value
    pub fn new(len_x: usize, len_y: usize, init: T) -> Grid<T> {
        Grid {
            flat: vec![init; len_x * len_y],
            len_x,
            len_y,
        }
    }

    // pub fn new_bordered<I>(len_x: usize, len_y: usize, border: T, iter: I) ->
    // Grid<T> where
    //     I: IntoIterator<Item = T>,
    // {
    //     let mut iter = iter.into_iter();
    //     let mut grid = Grid::new(len_x + 2, len_y + 2, border);
    //     for y in 1..=len_y {
    //         for x in 1..=len_x {
    //             let tile = grid.get_mut(x, y);
    //             *tile = iter.next().unwrap();
    //         }
    //     }
    //     grid
    // }

    // if the iterator does not have enough items to complete the last line, the
    // remanining elements will be completed with the border value.
    pub fn new_bordered_with_x<I>(len_x: usize, border: T, iter: I) -> Grid<T>
    where
        I: IntoIterator<Item = T>,
        T: Copy,
    {
        let mut iter = iter.into_iter().peekable();
        let mut flat = Vec::<T>::new();
        let len = len_x + 2;
        flat.extend(vec![border; len]); // upper border
        while iter.peek().is_some() {
            flat.push(border); // left border
            flat.extend(iter.by_ref().take(len_x));
            flat.push(border); // right border
        }
        flat.extend(vec![border; 2 * len - flat.len() % len]); // complete last line and add lower border

        Grid::from_vec(len, flat.len() / len, flat)
    }

    /// If vector `v` is larger than `len_x` * `len_y`, the extra elements are
    /// truncated.
    ///
    /// # Safety
    ///
    /// - The input vector `v` must have at least `len_x` * `len_y` lenght.
    /// Otherwise the program may panic while trying to access the elements of
    /// the inner vector;
    /// - Extra elements will be discarded.
    pub fn from_vec(len_x: usize, len_y: usize, mut v: Vec<T>) -> Grid<T> {
        debug_assert!(v.len() >= len_x * len_y);
        v.truncate(len_x * len_y);

        Grid {
            flat: v,
            len_x,
            len_y,
        }
    }

    // pub fn extend_with_x()
    // pub fn extend_with_y()
}

impl<T> Grid<T> {
    //------------------------------
    // Getters for single elements
    //------------------------------

    /// returns the value at position x,y.
    ///
    /// # Panics
    ///
    /// Panics if either index is out of bounds.
    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.flat[self.index(x, y)]
    }

    /// returns the value at position pos, where pos any type that can be
    /// represented as a Base2d<usize>.
    ///
    /// # Panics
    ///
    /// Panics if indexes are out of bounds.
    // pub fn get_from2d<V>(&self, pos: V) -> &T
    // where
    //     V: Into<Base2d<usize>>,
    // {
    //     let pos: Base2d<usize> = pos.into();
    //     self.get(pos.x, pos.y)
    // }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        let i = self.index(x, y); // must have an aux variable coz mutable borrow
        &mut self.flat[i]
    }

    pub fn update_at(&mut self, x: usize, y: usize, value: T) {
        let i = self.index(x, y); // must have an aux variable coz mutable borrow
        self.flat[i] = value;
    }

    // pub fn get_mut_from2d<V>(&mut self, p: V) -> &mut T
    // where
    //     V: Into<Base2d<usize>>,
    // {
    //     let p: Base2d<usize> = p.into();
    //     let i = self.index(p.x, p.y);
    //     &mut self.flat[i]
    // }

    /// returns the value at position `x, y`. Wraps around if either index is
    /// larger than its array dimension.
    ///
    /// # Examples
    ///
    /// For a 10x10 grid:
    ///
    /// - index (10, 5) yields the element at (0, 5);
    ///
    /// - index (8, 12) yields the element at (8, 2);
    ///
    /// - index (15, 15) yields the element at (5, 5).
    pub fn wrap(&self, x: usize, y: usize) -> &T {
        let nx = x % self.len_x;
        let ny = y % self.len_y;
        self.get(nx, ny)
    }

    /// returns the value at position `x, y`. If index `x` is larger than the x
    /// dimension of the grid, the index is wrapped around.
    ///
    /// # Panics
    ///
    /// Panics if the `y` index is out of bounds.
    pub fn wrap_x(&self, x: usize, y: usize) -> &T {
        let nx = x % self.len_x;
        self.get(nx, y)
    }

    /// returns the value at position `x, y`. If index `y` is larger than the y
    /// dimmension of the grid, the index is wrapped around.
    ///
    /// # Panics
    ///
    /// Panics if the `x` index is out of bounds.
    pub fn wrap_y(&self, x: usize, y: usize) -> &T {
        let ny = y % self.len_y;
        self.get(x, ny)
    }

    //------------------------------
    // Getters for multiple elements
    //------------------------------

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.flat.iter()
    }

    /// returns an iterator excluding the values at the borders
    pub fn inner_grid_iter(&self) -> impl Iterator<Item = &T> {
        let mut iterable = Vec::<&[T]>::new();
        for y in 1..&self.len_y - 1 {
            iterable.push(self.line_no_border(y, 1));
        }

        iterable.into_iter().flatten()
    }

    // pub fn get_position_iter(&self) -> impl Iterator<Item = (usize, usize)> {
    //     let iter = std::iter::repeat(1usize).zip(1..self.len_x - 1);
    //     for y in 2..&self.len_y - 1 {
    //         iter.chain(std::iter::repeat(y).zip(1..self.len_x - 1));
    //     }

    //     iter
    // }

    // returns an array slice for a line of the grid
    pub fn line(&self, y: usize) -> &[T] {
        &self.flat[self.index(0, y)..=self.index(self.len_x - 1, y)]
    }

    // returns an array slice for a line of the grid excluding the side borders
    pub fn line_no_border(&self, y: usize, border_size: usize) -> &[T] {
        &self.flat[self.index(border_size, y)..=self.index(self.len_x - border_size - 1, y)]
    }

    //------------------------------
    // Helpers
    //------------------------------

    /// returns the total size of the array (len_x * len_y)
    pub fn size(&self) -> usize {
        self.flat.len()
    }

    //------------------------------
    // Private
    //------------------------------

    /// returns the index for acessing the `flat` array from the coordinates `x`
    /// and `y`.
    fn index(&self, x: usize, y: usize) -> usize {
        self.len_x * y + x
    }
}

impl<T: AddAssign> Grid<T> {
    pub fn increment_by(&mut self, x: usize, y: usize, value: T) {
        let i = self.index(x, y); // must have an aux variable coz mutable borrow
        self.flat[i] += value;
    }
}

// impl<T, V> Index<V> for Grid<T>
// where
//     V: TryInto<Base2d<usize>>,
//     <V as TryInto<Base2d<usize>>>::Error: Debug,
// {
//     type Output = T;

//     fn index(&self, index: V) -> &Self::Output {
//         self.get_from2d(index.try_into().unwrap())
//     }
// }

// #[cfg(test)]
// pub mod tests {
//     use super::*;
//     use std::convert::TryFrom;

//     pub fn test() {
//         let grid = Grid::new(5, 5, 0u8);
//         println!("{}", usize::try_from(-3i32).unwrap());
//         let _ = grid[(3, 3)];
//     }
// }
