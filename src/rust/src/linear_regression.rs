use crate::fit_and_predict::FitAndPredict;
use crate::math_types::{ColVec, Mat};
use crate::rectangle_solve::rectangle_solve;
use anyhow::{anyhow, Result};

pub struct LinearRegression {
    coef: ColVec,
    sig2: f64,
    stderrest: ColVec,
}

impl LinearRegression {
    pub fn coef(&self) -> &ColVec {
        &self.coef
    }

    pub fn sig2(&self) -> f64 {
        self.sig2
    }

    pub fn stderrest(&self) -> &ColVec {
        &self.stderrest
    }
}

impl FitAndPredict for LinearRegression {
    fn fit(y: &ColVec, x: Mat) -> Result<Self> {
        let (n, k) = x.shape();

        let coef = rectangle_solve(x.clone(), y)?;
        // println!("coef = {coef}");
        let resid = y - &x * &coef;
        let sig2_mat = (resid.transpose() * resid) / (n as f64 - k as f64);
        assert_eq!(sig2_mat.shape(), (1, 1)); // should be scalar
        let sig2 = *sig2_mat.as_scalar();
        let stderrest = Mat::map_diagonal(
            &Mat::try_inverse(x.transpose() * &x)
                .ok_or_else(|| anyhow!("Cannot inverse x^t * x"))?,
            f64::sqrt,
        );

        Ok(LinearRegression {
            coef,
            sig2,
            stderrest,
        })
    }

    fn predict(&self, x: &Mat) -> (ColVec, ColVec) {
        // should test that X.n_cols == fit.X.n_cols
        let y = x * &self.coef;
        let stderr_v = Mat::map_diagonal(
            &(x * Mat::from_diagonal(&self.stderrest) * Mat::transpose(x)),
            f64::sqrt,
        );
        (y, stderr_v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math_utils::norm_inf;
    use rand::thread_rng;
    use rand_distr::Distribution;
    use rstest::rstest;

    #[rstest]
    fn test(#[values(40, 100, 1000)] n: usize, #[values(3, 6)] m: usize) {
        let mut rng = thread_rng();

        let normal = rand_distr::Normal::new(0., 1.).unwrap();

        let sol = ColVec::from_fn(m, |_, _| normal.sample(&mut rng));

        let mut x = Mat::from_fn(n, m, |_, _| normal.sample(&mut rng));
        x.fill_column(0, 1.);

        // WHEN value is perfectly computed
        {
            let y = &x * &sol;

            // println!("sol = {sol}");
            // println!("x = {x}");
            // println!("y = {y}");

            let rl = LinearRegression::fit(&y, x.clone()).unwrap();
            let (y_pred, _) = rl.predict(&x);
            let eps = 1e-5;
            assert!(norm_inf(y - y_pred) < 10. * eps);
        }

        // WHEN value is computed with noise
        {
            let e = 1e-8;
            let noise = rand_distr::Normal::new(1.0, e).unwrap();

            let y = (&x * &sol).map(|v| v * noise.sample(&mut rng));

            // println!("sol = {sol}");
            // println!("x = {x}");
            // println!("y = {y}");

            let rl = LinearRegression::fit(&y, x.clone()).unwrap();
            let (y_pred, _) = rl.predict(&x);
            let eps = 1e-5;
            assert!(norm_inf(y - y_pred) < 10. * eps + 10. * e);
        }
    }
}
