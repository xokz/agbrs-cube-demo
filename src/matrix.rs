#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]
#![deny(clippy::all)]

use agb::fixnum::{FixedNum, Number};
use crate::Num;
use crate::Vector3D;

pub type DefaultNum = Num<i32, 12>;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Matrix {
    pub width: usize,
    pub height: usize,
    pub elem: [[DefaultNum; 4]; 4],
}

impl Matrix {
    pub fn new(h: usize, w: usize) -> Matrix {
        Matrix {
            width: w,
            height: h,
            elem: [[agb::fixnum::Num::from_raw(0); 4]; 4],
        }
    }

    pub fn from_vertex(v: Vector3D<DefaultNum>) -> Matrix {
        let mut output = Matrix::new(1, 3);
        output.elem[0][0] = v.x;
        output.elem[1][0] = v.y;
        output.elem[2][0] = v.z;
        output
    }

    pub fn to_vertex(m: Matrix) -> Vector3D<DefaultNum> {
        Vector3D {
            x: m.elem[0][0],
            y: m.elem[1][0],
            z: m.elem[2][0],
        }
    }
}

pub trait MatrixMath {
    fn mul(&self, rhs: Matrix) -> Matrix;
}

/*
    first index for matrix is the x coordinate not the y
    [0,0][1,0][2,0][3,0]
    [0,1][1,1][2,1][3,1]
    [0,2][1,2][2,2][3,2]
    [0,3][1,3][2,3][3,3]
*/

impl MatrixMath for Matrix {
    fn mul(&self, rhs: Matrix) -> Matrix {
        if self.width != rhs.height {
            panic!("Improperly sized matricies.");
        }

        let mut product = Matrix::new(self.height, rhs.width);
        for x in 0..product.width {
            for y in 0..product.height {
                for i in 0..self.width {
                    product.elem[x][y] += self.elem[i][y] * rhs.elem[x][i];
                }
            }
        }
        product
    }
}
