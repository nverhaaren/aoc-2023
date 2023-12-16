use std::iter;
use std::ops::{Add, Bound, RangeBounds};
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
            Bound::Included(s) => if *target < s {
                *target = s;
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
            Bound::Included(s) => if *target < s {
                *target = s;
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

pub type Grid<T> = Vec<Vec<T>>;

pub fn transpose_grid<T>(grid: Grid<T>) -> Grid<T> {
    let mut rotated = rotate_grid_clockwise(grid);
    for row in rotated.iter_mut() {
        row.reverse();
    }
    rotated
}

pub fn rotate_grid_clockwise<T>(mut grid: Grid<T>) -> Grid<T> {
    if grid.is_empty() {
        return grid;
    }
    let mut new_grid: Vec<_> = iter::repeat_with(|| vec![]).take(grid[0].len()).collect();
    while let Some(last) = grid.pop() {
        for (idx, t) in last.into_iter().enumerate() {
            new_grid[idx].push(t);
        }
    }
    new_grid
}

#[cfg(test)]
mod test_transforms {
    use super::*;

    fn simple_grid() -> Grid<i32> {
        return vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
        ]
    }

    #[test]
    fn test_transpose() {
        assert_eq!(transpose_grid(simple_grid()), vec![
            vec![1, 4],
            vec![2, 5],
            vec![3, 6],
        ]);
    }

    #[test]
    fn test_rotate_clockwise() {
        assert_eq!(rotate_grid_clockwise(simple_grid()), vec![
            vec![4, 1],
            vec![5, 2],
            vec![6, 3],
        ]);
    }
}