use crate::math_types::{ColVec, Mat};
use anyhow::Result;

pub trait FitAndPredict {
    fn fit(v: &ColVec, x: Mat) -> Result<Self>
    where
        Self: Sized;

    fn predict(&self, x: &Mat) -> (ColVec, ColVec);
}
