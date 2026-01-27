use core::range::Step;
use num_traits::Num;
use std::ops::{Range, RangeInclusive};

pub mod db;
pub mod midi_note;
pub mod modulated;
pub mod envelope;
pub mod fallback;

pub trait Single<T> {
    fn single(value: T) -> Self;
}

impl<T: Copy + Step> Single<T> for RangeInclusive<T> {
    fn single(value: T) -> Self {
        value..=value
    }
}

impl<T: Num + Copy + Step> Single<T> for Range<T> {
    fn single(value: T) -> Self {
        value..(value + T::one())
    }
}

pub fn lerp<T: Num + Copy>(range: impl Into<RangeInclusive<T>>, t: T) -> T {
    let range = range.into();
    let (&a, &b) = (range.start(), range.end());
    a + (b - a) * t
}

#[macro_export]
macro_rules! const_pat {
    {$x:expr} => {__const_pat_value if (__const_pat_value == const { $x })};
}