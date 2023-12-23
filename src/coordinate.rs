pub mod grid;

use std::ops::{Add, Bound, RangeBounds, Sub};
use itertools::Itertools;
use crate::util::{CheckedAdd, CheckedSub};

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
    pub const ALL: [Direction; 4] =
        [Direction::North, Direction::East, Direction::South, Direction::West];


    pub const fn opposite(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

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

impl<const N: usize> Add<ICoordinate<N>> for ICoordinate<N> {
    type Output = Self;
    fn add(self, rhs: ICoordinate<N>) -> Self::Output {
        let mut result = self.0.clone();
        for (target, to_add) in result.iter_mut().zip_eq(rhs.0.iter().copied()) {
            *target += to_add;
        }
        Self(result)
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

impl<const N: usize> Add<UCoordinate<N>> for UCoordinate<N> {
    type Output = Self;
    fn add(self, rhs: UCoordinate<N>) -> Self::Output {
        let mut result = self.0.clone();
        for (target, to_add) in result.iter_mut().zip_eq(rhs.0.iter().copied()) {
            *target += to_add;
        }
        Self(result)
    }
}

impl<const N: usize> CheckedAdd<UCoordinate<N>> for UCoordinate<N> {
    fn checked_add(&self, v: &UCoordinate<N>) -> Option<Self::Output> {
        let mut result = self.0.clone();
        for (target, to_add) in result.iter_mut().zip_eq(v.0.iter().copied()) {
            *target = target.checked_add(to_add)?
        }
        Some(Self(result))
    }
}

impl<const N: usize> Sub<UCoordinate<N>> for UCoordinate<N> {
    type Output = Self;
    fn sub(self, rhs: UCoordinate<N>) -> Self::Output {
        let mut result = self.0.clone();
        for (target, to_add) in result.iter_mut().zip_eq(rhs.0.iter().copied()) {
            *target -= to_add;
        }
        Self(result)
    }
}

impl<const N: usize> CheckedSub<UCoordinate<N>> for UCoordinate<N> {
    fn checked_sub(&self, v: &UCoordinate<N>) -> Option<Self::Output> {
        let mut result = self.0.clone();
        for (target, to_add) in result.iter_mut().zip_eq(v.0.iter().copied()) {
            *target = target.checked_sub(to_add)?
        }
        Some(Self(result))
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

pub fn twice_shoelace(it: impl Iterator<Item=ICoordinate<2>> + Clone + ExactSizeIterator) -> usize {
    // TODO: without using circular windows I suspect I can relax these bounds - have a single
    // iteration cover shoelace, boundary, and pick?
    it.circular_tuple_windows::<(_, _)>()
        .map(|(a, b)| {
            let a: (_, _) = a.into();
            let b: (_, _) = b.into();
            a.0 * b.1 - b.0 * a.1
        })
        .sum::<isize>()
        .abs() as usize
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
