use core::range::Step;
use num_traits::Num;
use std::ops::{Range, RangeInclusive};

pub mod db;
pub mod midi_note;
pub mod modulated;

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
