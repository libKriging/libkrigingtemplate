use crate::math_types::{ColVec, Mat};
use anyhow::{anyhow, Result};
use extendr_api::{RArray, RMatrix};

// Needs additional check to ensure ncols == 1
pub type RVector<T> = RArray<T, [usize; 2]>;

pub fn to_mat(x: RMatrix<f64>) -> Result<Mat> {
    Ok(Mat::from_row_slice(
        x.nrows(),
        x.ncols(),
        x.as_real_slice()
            .ok_or_else(|| anyhow!("Cannot extract data"))?,
    ))
}

pub fn to_colvec(x: RArray<f64, [usize; 2]>) -> Result<ColVec> {
    if x.ncols() == 1 {
        Ok(ColVec::from_row_slice(
            x.as_real_slice()
                .ok_or_else(|| anyhow!("Cannot extract data"))?,
        ))
    } else {
        Err(anyhow!("Single column matrix expected"))
    }
}

pub fn from_colvec(x: ColVec) -> Result<RVector<f64>> {
    Ok(RVector::new_matrix(x.nrows(), x.ncols(), |i, _j| {
        debug_assert_eq!(x.ncols(), 1);
        x[i]
    }))
}
