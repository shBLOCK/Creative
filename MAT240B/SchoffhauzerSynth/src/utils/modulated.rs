#![allow(unused)]

use derive_more::Display;
use std::fmt::Debug;
use std::ops::Add;

#[derive_aliases::derive(..Copy, Debug, Display, ..SerDe, Default)]
#[display("({value} + {modulation})")]
pub struct Modulated<T> {
    pub value: T,
    pub modulation: T,
}

impl<T: Add<T, Output = T> + Copy> Modulated<T> {
    pub fn modulated(self) -> T {
        self.value + self.modulation
    }
}

impl<T> Modulated<T> {
    pub fn into_modulated<I: Add<I, Output = I> + Copy>(self) -> I
    where
        T: Into<I>,
    {
        self.value.into() + self.modulation.into()
    }
}

impl<T> Modulated<T> {
    pub fn new(value: T, modulation: T) -> Self {
        Self { value, modulation }
    }

    pub fn map2<B, R>(&self, other: &Modulated<B>, mut f: impl FnMut(&T, &B) -> R) -> Modulated<R> {
        Modulated {
            value: f(&self.value, &other.value),
            modulation: f(&self.modulation, &other.modulation),
        }
    }
}

impl<T: Copy> Modulated<Option<T>> {
    pub fn unwrap_or(self, default: Modulated<T>) -> Modulated<T> {
        self.map2(&default, |a, b| a.unwrap_or(*b))
    }
}
