use crate::math_types::{ColVec, Mat};
use anyhow::{anyhow, Result};

pub fn rectangle_solve(x: Mat, y: &ColVec) -> Result<ColVec> {
    solve1(x, y)
}

fn solve1(x: Mat, y: &ColVec) -> Result<ColVec> {
    // println!("x : {:?}", x.shape());
    let qr = x.qr();
    // println!("q: {:?}", qr.q().shape());

    let mut y = Mat::transpose(&qr.q()) * y;
    // println!("tr(q)*y: {}", y);

    let r = qr.unpack_r();
    // println!("r: {:?}", r.shape());

    let solved = r.solve_upper_triangular_mut(&mut y);
    if solved {
        Ok(y)
    } else {
        Err(anyhow!("Cannot solve"))
    }
}

fn solve2(x: Mat, y: &ColVec) -> Result<ColVec> {
    // println!("x : {:?}", x.shape());
    let qr = x.qr();

    // println!("q: {:?}", qr.q().shape());

    let mut y = y.clone();
    qr.q_tr_mul(&mut y);
    // println!("tr(q)*y: {}", y);

    let r = qr.unpack_r();
    // println!("r: {:?}", r.shape());

    y.resize_vertically_mut(r.shape().0, f64::NAN);

    let solved = r.solve_upper_triangular_mut(&mut y);
    if solved {
        Ok(y)
    } else {
        Err(anyhow!("Cannot solve"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_qr() {
        let x = Mat::from_row_slice(
            12,
            2,
            &[
                //
                -4., 20., 35., 5., //
                -4., -30., -15., 55., //
                -8., 40., -80., -65., //
                23., -15., 30., 15., //
                3., -5., 10., 15., //
                -2., -1., 5., 15., //
            ],
        );

        let y = ColVec::from_row_slice(&[1.; 12]);

        println!("x: {x}");

        let qr = x.clone().qr();
        let q = qr.q();
        let r = qr.r();

        println!("q: {q}");
        println!("r: {r}");

        println!("tr(q)*q: {}", Mat::transpose(&q) * &q);

        let a = &q * r;

        println!("a: {a}");
    }
}
