use derive_more::{Display, From};
use num_traits::{AsPrimitive, Float};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive_aliases::derive(..Copy, Default, ..Ord, Debug, Display, From, ..NumOpsAssign, ..SerDe)]
#[display("{_0} dB")]
pub struct DB<F: Float + Debug + 'static>(pub F)
where
    f32: AsPrimitive<F>;

impl<F: Float + Debug + Serialize + Deserialize<'static> + 'static> DB<F>
where
    f32: AsPrimitive<F>,
{
    pub fn from_linear(linear: F) -> Self {
        Self(linear.log10() / 20.0.as_())
    }

    pub fn db(self) -> F {
        self.0
    }

    pub fn linear(self) -> F {
        10.0.as_().powf(self.0 / 20.0.as_())
    }
}

impl From<DB<f32>> for f32 {
    fn from(value: DB<f32>) -> Self {
        value.0
    }
}

impl From<DB<f64>> for f64 {
    fn from(value: DB<f64>) -> Self {
        value.0
    }
}
