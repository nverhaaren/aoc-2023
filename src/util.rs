use std::ops::{Add, Sub};

pub fn parse_number(s: &str) -> u64 {
    s.parse().expect(&format!("number parse issue {s:?}"))
}

pub fn parse_signed_number(s: &str) -> i64 {
    s.parse().expect(&format!("number parse issue {s:?}"))
}

pub trait InspectVal: Sized {
    fn inspect_val(self, f: impl FnOnce(&Self) -> ()) -> Self {
        f(&self);
        self
    }

    fn inspect_val_mut(mut self, f: impl FnOnce(&mut Self) -> ()) -> Self {
        f(&mut self);
        self
    }
}

impl<T> InspectVal for T {}

pub trait CheckedAdd<U>: Add<U> {
    fn checked_add(&self, v: &U) -> Option<Self::Output>;
}

pub trait CheckedSub<U>: Sub<U> {
    fn checked_sub(&self, v: &U) -> Option<Self::Output>;
}
