use std::error::Error;
use std::fmt::{Display, Formatter, Write};
use std::{io, iter};
use std::io::{BufRead, BufReader};
use std::ops::{Index, IndexMut};
use itertools::Itertools;
use crate::util::{CheckedAdd, CheckedSub};
use thiserror;
use super::{Direction, UCoordinate};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct Grid<T> {
    data: Vec<Vec<T>>,
    row_length: usize,
}

impl<T: Clone> Grid<T> {
    pub fn full(rows: usize, cols: usize, value: T) -> Self {
        assert!(rows > 0);
        assert!(cols > 0);
        let data = iter::repeat_with(move || vec![value.clone(); cols]).take(rows).collect();
        Self { data, row_length: cols }
    }
}

impl<T> Grid<T> {
    pub fn to_vec_of_vecs(self) -> Vec<Vec<T>> {
        self.data
    }

    pub fn try_from_vec_of_vecs(vecs: Vec<Vec<T>>) -> Result<Self, GridLoadError> {
        let first_len = vecs.first().ok_or(GridLoadError::Empty)?.len();
        if !vecs[1..].iter().all(|v| v.len() == first_len) {
            return Err(GridLoadError::Jagged);
        }
        if first_len == 0 {
            return Err(GridLoadError::Empty);
        }
        Ok(Self { data: vecs, row_length: first_len })
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.data.len(), self.row_length)
    }

    pub fn rows(&self) -> usize {
        self.data.len()
    }

    pub fn cols(&self) -> usize {
        self.row_length
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.data.get(row).and_then(|row| row.get(col))
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        self.data.get_mut(row).and_then(|row| row.get_mut(col))
    }

    pub fn get_row(&self, idx: usize) -> Option<&[T]> {
        self.data.get(idx).map(|v| v.as_slice())
    }

    pub fn get_row_mut(&mut self, idx: usize) -> Option<&mut[T]> {
        self.data.get_mut(idx).map(|v| v.as_mut_slice())
    }

    pub fn iter_rows(&self) -> impl Iterator<Item=&[T]> + '_ {
        self.data.iter()
            .map(|row| row.as_slice())
    }

    pub fn iter_rows_mut(&mut self) -> impl Iterator<Item=&mut [T]> + '_ {
        self.data.iter_mut()
            .map(|row| row.as_mut_slice())
    }

    pub fn iter_col(&self, idx: usize) -> Option<impl Iterator<Item=&T> + '_> {
        if idx < self.row_length {Some(
            self.data.iter()
                .map(move |row| &row[idx])
        )} else {
            None
        }
    }

    pub fn iter_col_mut(&mut self, idx: usize) -> Option<impl Iterator<Item=&mut T> + '_> {
        if idx < self.row_length {Some(
            self.data.iter_mut()
                .map(move |row| &mut row[idx])
        )} else {
            None
        }
    }

    pub fn iter_cols(&self) -> impl Iterator<Item=impl Iterator<Item=&T> + '_> + '_ {
        (0..self.row_length).into_iter()
            .map(|idx| self.iter_col(idx).unwrap())
    }

    // iter_cols_mut if needed

    pub fn is_in_bounds(&self, coordinate: &UCoordinate<2>) -> bool {
        (0..self.rows()).contains(&coordinate.0[0]) && (0..self.cols()).contains(&coordinate.0[1])
    }

    pub fn bound_coordinate<'a>(&self, coordinate: &'a mut UCoordinate<2>) -> &'a mut UCoordinate<2> {
        coordinate.bound_axis(0, 0..self.rows()).expect("logic error");
        coordinate.bound_axis(1, 0..self.cols()).expect("logic error");
        coordinate
    }

    pub fn bounded_add(&self, coordinate: &UCoordinate<2>, direction: Direction) -> UCoordinate<2> {
        let mut result = coordinate.checked_add(&direction).unwrap_or(*coordinate);
        self.bound_coordinate(&mut result);
        result
    }

    pub fn checked_add<U>(&self, coordinate: &UCoordinate<2>, other: &U) -> Option<UCoordinate<2>>
    where UCoordinate<2>: CheckedAdd<U, Output=UCoordinate<2>> {
        let result = coordinate.checked_add(other)?;
        if self.is_in_bounds(&result) {
            Some(result)
        } else {
            None
        }
    }

    pub fn checked_sub<U>(&self, coordinate: &UCoordinate<2>, other: &U) -> Option<UCoordinate<2>>
    where UCoordinate<2>: CheckedSub<U, Output=UCoordinate<2>> {
        let result = coordinate.checked_sub(other)?;
        if self.is_in_bounds(&result) {
            Some(result)
        } else {
            None
        }
    }

    pub fn neighbors<'a>(&'a self, coordinate: &UCoordinate<2>) -> impl Iterator<Item=(Direction, UCoordinate<2>)> + 'a {
        let coordinate = *coordinate;
        Direction::ALL.iter().copied()
            .map(move |d| (d, self.bounded_add(&coordinate, d)))
            .filter(move |(_d, n)| n != &coordinate)
    }

    pub fn rotate_clockwise(self) -> Self {
        let Self { mut data, row_length } = self;
        let old_rows = data.len();
        let mut new_data: Vec<_> = iter::repeat_with(|| vec![]).take(row_length).collect();
        while let Some(last) = data.pop() {
            for (idx, t) in last.into_iter().enumerate() {
                new_data[idx].push(t);
            }
        }
        Self { data: new_data, row_length: old_rows }
    }

    pub fn transpose(self) -> Self {
        let mut rotated = self.rotate_clockwise();
        for row in rotated.data.iter_mut() {
            row.reverse();
        }
        rotated
    }

    pub fn iter_idxs(&self) -> impl Iterator<Item=UCoordinate<2>> + '_ {
        (0..self.rows()).cartesian_product(0..self.cols())
            .map(|(r, c)| (r, c).into())
    }
}

impl<T> Index<UCoordinate<2>> for Grid<T> {
    type Output = T;

    fn index(&self, index: UCoordinate<2>) -> &Self::Output {
        &self.data[index.0[0]][index.0[1]]
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<UCoordinate<2>> for Grid<T> {
    fn index_mut(&mut self, index: UCoordinate<2>) -> &mut Self::Output {
        &mut self.data[index.0[0]][index.0[1]]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.iter_rows() {
            for cell in row {
                cell.fmt(f)?
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GridLoadError {
    #[error("a grid must contain at least one element")]
    Empty,
    #[error("all rows must have the same number of columns")]
    Jagged,
}

pub fn get_byte_grid_from_stdin() -> Result<Grid<u8>, Box<dyn Error>> {
    let stdin = io::stdin();
    let reader = BufReader::with_capacity(256, stdin.lock());
    let lines: Vec<_> = reader.lines()
        .map(|x| x.map(|s| s.into_bytes()))
        .try_collect()?;
    let grid = Grid::try_from_vec_of_vecs(lines)?;
    Ok(grid)
}

#[cfg(test)]
mod test_transforms {
    use super::*;

    fn simple_grid() -> Grid<i32> {
        return Grid::try_from_vec_of_vecs(vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
        ]).unwrap()
    }

    #[test]
    fn test_transpose() {
        assert_eq!(simple_grid().transpose().to_vec_of_vecs(), vec![
            vec![1, 4],
            vec![2, 5],
            vec![3, 6],
        ]);
    }

    #[test]
    fn test_rotate_clockwise() {
        assert_eq!(simple_grid().rotate_clockwise().to_vec_of_vecs(), vec![
            vec![4, 1],
            vec![5, 2],
            vec![6, 3],
        ]);
    }
}

