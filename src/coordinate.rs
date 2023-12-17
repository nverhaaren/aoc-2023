use std::fmt::{Display, Formatter, Write};
use std::iter;
use std::ops::{Add, Bound, Index, IndexMut, RangeBounds};
use thiserror::Error;
use crate::util::CheckedAdd;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl From<Direction> for ICoordinate<2> {
    fn from(value: Direction) -> Self {
        match value {
            Direction::North => ICoordinate([-1, 0]),
            Direction::East => ICoordinate([0, 1]),
            Direction::South => ICoordinate([1, 0]),
            Direction::West => ICoordinate([0, -1]),
        }
    }
}

impl Direction {
    pub const fn reflect_over_row(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            _ => self,
        }
    }

    pub const fn reflect_over_col(self) -> Self {
        match self {
            Direction::East => Direction::West,
            Direction::West => Direction::East,
            _ => self,
        }
    }

    pub const fn transpose(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::South,
            Direction::South => Direction::East,
            Direction::West => Direction::North,
        }
    }

    pub const fn transpose_secondary(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::North,
            Direction::South => Direction::West,
            Direction::West => Direction::South,
        }
    }

    pub const fn rotate_clockwise(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub const fn rotate_counter_clockwise(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub struct ICoordinate<const N: usize>([isize; N]);

impl<const N: usize> ICoordinate<N> {
    pub const fn new(data: [isize; N]) -> Self {
        assert!(N > 0);
        Self(data)
    }

    pub const fn origin() -> Self {
        assert!(N > 0);
        Self([0; N])
    }

    pub fn taxicab_dist(&self, other: &Self) -> usize {
        self.0.iter().copied()
            .zip(other.0.iter().copied())
            .map(|(a, b)| a.abs_diff(b))
            .sum()
    }

    pub fn king_dist(&self, other: &Self) -> usize {
        // In 2D this is number of moves for a chess king to get from self to other
        self.0.iter().copied()
            .zip(other.0.iter().copied())
            .map(|(a, b)| a.abs_diff(b))
            .max().expect("Invariant violated")
    }

    /// Requires that the range contain at least one element - if so returns Ok(new value), or else
    /// returns Err(existing value)
    pub fn bound_axis(&mut self, axis: usize, bound: impl RangeBounds<isize>) -> Result<isize, isize> {
        let target = &mut self.0[axis];
        let start = match bound.start_bound() {
            Bound::Excluded(s) => Bound::Included(s.checked_add(1).ok_or(*target)?),
            b => b.cloned(),
        };
        let end = match bound.end_bound() {
            Bound::Excluded(e) => Bound::Included(e.checked_sub(1).ok_or(*target)?),
            b => b.cloned(),
        };
        match (start, end) {
            (Bound::Included(s), Bound::Included(e)) => if e < s {
                return Err(*target);
            },
            _ => (),
        };
        match start {
            Bound::Excluded(_) => unreachable!(),
            Bound::Included(s) => if *target < s {
                *target = s;
            },
            Bound::Unbounded => (),
        };
        match end {
            Bound::Excluded(_) => unreachable!(),
            Bound::Included(e) => if *target > e {
                *target = e;
            },
            Bound::Unbounded => (),
        };
        Ok(*target)
    }
}

impl<const N: usize> Default for ICoordinate<N> {
    fn default() -> Self {
        Self::origin()
    }
}

impl<const N: usize> From<[isize; N]> for ICoordinate<N> {
    fn from(value: [isize; N]) -> Self {
        Self::new(value)
    }
}

impl<const N: usize> From<ICoordinate<N>> for [isize; N] {
    fn from(value: ICoordinate<N>) -> Self {
        value.0
    }
}

impl<const N: usize> AsRef<[isize; N]> for ICoordinate<N> {
    fn as_ref(&self) -> &[isize; N] {
        &self.0
    }
}

impl<const N: usize> AsMut<[isize; N]> for ICoordinate<N> {
    fn as_mut(&mut self) -> &mut [isize; N] {
        &mut self.0
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]

pub struct UCoordinate<const N: usize>([usize; N]);

impl<const N: usize> UCoordinate<N> {
    pub fn new(data: [usize; N]) -> Self {
        assert!(N > 0);
        Self(data)
    }

    pub const fn origin() -> Self {
        assert!(N > 0);
        Self([0; N])
    }

    pub fn taxicab_dist(&self, other: &Self) -> usize {
        self.0.iter().copied()
            .zip(other.0.iter().copied())
            .map(|(a, b)| a.abs_diff(b))
            .sum()
    }

    pub fn king_dist(&self, other: &Self) -> usize {
        // In 2D this is number of moves for a chess king to get from self to other
        self.0.iter().copied()
            .zip(other.0.iter().copied())
            .map(|(a, b)| a.abs_diff(b))
            .max().expect("Invariant violated")
    }

    /// Requires that the range contain at least one element - if so returns Ok(new value), or else
    /// returns Err(existing value)
    pub fn bound_axis(&mut self, axis: usize, bound: impl RangeBounds<usize>) -> Result<usize, usize> {
        let target = &mut self.0[axis];
        let start = match bound.start_bound() {
            Bound::Excluded(s) => Bound::Included(s.checked_add(1).ok_or(*target)?),
            b => b.cloned(),
        };
        let end = match bound.end_bound() {
            Bound::Excluded(e) => Bound::Included(e.checked_sub(1).ok_or(*target)?),
            b => b.cloned(),
        };
        match (start, end) {
            (Bound::Included(s), Bound::Included(e)) => if e < s {
                return Err(*target);
            },
            _ => (),
        };
        match start {
            Bound::Excluded(_) => unreachable!(),
            Bound::Included(s) => if *target < s {
                *target = s;
            },
            Bound::Unbounded => (),
        };
        match end {
            Bound::Excluded(_) => unreachable!(),
            Bound::Included(e) => if *target > e {
                *target = e;
            },
            Bound::Unbounded => (),
        };
        Ok(*target)
    }
}

impl<const N: usize> Default for UCoordinate<N> {
    fn default() -> Self {
        Self::origin()
    }
}

impl<const N: usize> From<[usize; N]> for UCoordinate<N> {
    fn from(value: [usize; N]) -> Self {
        Self::new(value)
    }
}

impl<const N: usize> From<UCoordinate<N>> for [usize; N] {
    fn from(value: UCoordinate<N>) -> Self {
        value.0
    }
}

impl<const N: usize> AsRef<[usize; N]> for UCoordinate<N> {
    fn as_ref(&self) -> &[usize; N] {
        &self.0
    }
}

impl<const N: usize> AsMut<[usize; N]> for UCoordinate<N> {
    fn as_mut(&mut self) -> &mut [usize; N] {
        &mut self.0
    }
}

impl<const N: usize> TryFrom<UCoordinate<N>> for ICoordinate<N> {
    type Error = UCoordinate<N>;
    fn try_from(value: UCoordinate<N>) -> Result<Self, Self::Error> {
        let mut result = [0isize; N];
        for (idx, u) in value.0.iter().copied().enumerate() {
            let i: isize = u.try_into().map_err(|_| value)?;
            result[idx] = i;
        }
        Ok(result.into())
    }
}

impl<const N: usize> TryFrom<ICoordinate<N>> for UCoordinate<N> {
    type Error = ICoordinate<N>;
    fn try_from(value: ICoordinate<N>) -> Result<Self, Self::Error> {
        let mut result = [0usize; N];
        for (idx, i) in value.0.iter().copied().enumerate() {
            let u: usize = i.try_into().map_err(|_| value)?;
            result[idx] = u;
        }
        Ok(result.into())
    }
}

impl Add<Direction> for UCoordinate<2> {
    type Output = UCoordinate<2>;
    fn add(self, rhs: Direction) -> Self::Output {
        let [row, col] = self.0;
        match rhs {
            Direction::North => Self([row - 1, col]),
            Direction::East => Self([row, col + 1]),
            Direction::South => Self([row + 1, col]),
            Direction::West => Self([row, col - 1]),
        }
    }
}

impl CheckedAdd<Direction> for UCoordinate<2> {
    fn checked_add(&self, v: &Direction) -> Option<Self::Output> {
        let [row, col] = self.0;
        let v = *v;
        Some(match v {
            Direction::North => Self([row.checked_sub(1)?, col]),
            Direction::East => Self([row, col.checked_add(1)?]),
            Direction::South => Self([row.checked_add(1)?, col]),
            Direction::West => Self([row, col.checked_sub(1)?]),
        })
    }
}

// TODO: is_adjacent, shoelace/pick, etc

// More conversions

// 2D
impl From<ICoordinate<2>> for (isize, isize) {
    fn from(value: ICoordinate<2>) -> Self {
        (value.as_ref()[0], value.as_ref()[1])
    }
}

impl From<(isize, isize)> for ICoordinate<2> {
    fn from(value: (isize, isize)) -> Self {
        [value.0, value.1].into()
    }
}

impl From<UCoordinate<2>> for (usize, usize) {
    fn from(value: UCoordinate<2>) -> Self {
        (value.as_ref()[0], value.as_ref()[1])
    }
}

impl From<(usize, usize)> for UCoordinate<2> {
    fn from(value: (usize, usize)) -> Self {
        [value.0, value.1].into()
    }
}

// Grid

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

    pub fn bound_coordinate(&self, coordinate: &mut UCoordinate<2>) {
        coordinate.bound_axis(0, 0..self.rows()).expect("logic error");
        coordinate.bound_axis(1, 0..self.cols()).expect("logic error");
    }

    pub fn bounded_add(&self, coordinate: &UCoordinate<2>, direction: Direction) -> UCoordinate<2> {
        let mut result = coordinate.checked_add(&direction).unwrap_or(*coordinate);
        self.bound_coordinate(&mut result);
        result
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

#[derive(Error, Debug)]
pub enum GridLoadError {
    #[error("a grid must contain at least one element")]
    Empty,
    #[error("all rows must have the same number of columns")]
    Jagged,
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