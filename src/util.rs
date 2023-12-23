use std::io;
use std::io::{BufRead, BufReader};
use std::marker::PhantomData;
use std::ops::{Add, Sub};
use std::str::FromStr;
use itertools::Itertools;

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

pub trait Parser {
    type Parsed<'a, 'p> where Self: 'p;
    type Err;
    fn parse<'a, 'p>(&'p self, s: &'a str) -> Result<Self::Parsed<'a, 'p>, Self::Err>;

    // future: impl Iterator version
    fn parse_lines_to_vec<'a, 'p>(
        &'p self,
        it: impl IntoIterator<Item=&'a str>
    ) -> Result<Vec<Self::Parsed<'a, 'p>>, Self::Err> {
        it.into_iter()
            .map(|s| self.parse(s))
            .try_collect()
    }
}

pub struct FromStrParser<T: FromStr>(PhantomData<T>);

impl<T: FromStr> FromStrParser<T> {
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T: FromStr> Parser for FromStrParser<T> {
    type Parsed<'a, 'p> = T where Self: 'p;
    type Err = T::Err;

    fn parse<'a, 'p>(&'p self, s: &'a str) -> Result<Self::Parsed<'a, 'p>, Self::Err> {
        s.parse()
    }
}

pub fn get_lines_from_stdin() -> io::Result<Vec<String>> {
    let stdin = io::stdin();
    let reader = BufReader::with_capacity(256, stdin.lock());
    reader.lines().try_collect()
}
