use crate::fit_and_predict::FitAndPredict;
use crate::r_binding::conversion::RVector;
use anyhow::Result;
use conversion::{from_colvec, to_colvec, to_mat};
use extendr_api::prelude::*;

mod conversion;

/// Return version string
/// @export
#[extendr]
fn full_version() -> String {
    crate::version::full_version()
}

struct LinearRegression(crate::linear_regression::LinearRegression);

/// First linear regression implementation test
/// @export
#[extendr]
impl LinearRegression {
    pub fn fit(y: RVector<f64>, x: RMatrix<f64>) -> Result<Self> {
        Ok(LinearRegression(
            crate::linear_regression::LinearRegression::fit(&to_colvec(y)?, to_mat(x)?)?,
        ))
    }

    pub fn predict(&self, x: RMatrix<f64>) -> Result<Robj> {
        // Could be a named list; cf https://docs.rs/extendr-api/latest/extendr_api/
        // let list = list!(a = 1, b = 2);

        let (a, b) = self.0.predict(&to_mat(x)?);
        Ok(r!(List::from_values(&[
            r!(from_colvec(a)),
            r!(from_colvec(b))
        ])))
    }
}

#[extendr]
pub fn linear_regression_fit1(y: ArrayView1<f64>, x: ArrayView2<f64>) {
    println!("{y:?}");
    println!("{x:?}");
}

#[extendr]
pub fn linear_regression_fit2(y: RArray<f64, [usize; 2]>, x: RMatrix<f64>) {
    println!("{y:?}");
    println!("{x:?}");
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod libkrigingtemplate;
    fn full_version;
    impl LinearRegression;
    fn linear_regression_fit1;
    fn linear_regression_fit2;
}
