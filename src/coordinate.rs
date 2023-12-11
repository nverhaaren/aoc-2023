#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub struct ICoordinate<const N: usize>([isize; N]);

impl<const N: usize> ICoordinate<N> {
    pub const fn new(data: [isize; N]) -> Self {
        Self(data)
    }

    pub fn taxicab_dist(&self, other: &Self) -> usize {
        self.0.iter().copied()
            .zip(other.0.iter().copied())
            .map(|(a, b)| a.abs_diff(b))
            .sum()
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
        Self(data)
    }

    pub fn taxicab_dist(&self, other: &Self) -> usize {
        self.0.iter().copied()
            .zip(other.0.iter().copied())
            .map(|(a, b)| a.abs_diff(b))
            .sum()
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

// TODO: is_adjacent, directions, etc

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
