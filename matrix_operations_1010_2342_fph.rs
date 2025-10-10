use std::ops::Add;
use std::fmt::{self, Debug};
use std::error::Error;
use thiserror::Error;
use serde::{Serialize, Deserialize};

// 定义一个矩阵结构体
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Matrix<T> 
where
    T: Add<Output = T> + Copy + Debug,
{
    width: usize,
    height: usize,
    data: Vec<T>,
}

// 定义矩阵错误
#[derive(Debug, Error)]
pub enum MatrixError {
    #[error("Invalid dimensions: cannot multiply {0}x{1} with {2}x{3}")]
    InvalidDimensions(usize, usize, usize, usize),
    #[error("Invalid dimensions: cannot add {0}x{1} with {2}x{3}")]
    InvalidAddDimensions(usize, usize, usize, usize),
    #[error("Invalid index: {0} out of bounds for {1}x{2} matrix")]
    InvalidIndex(usize, usize, usize),
}

impl<T> Matrix<T> 
where
    T: Add<Output = T> + Copy + Debug,
{
    // 创建一个新的矩阵
    pub fn new(width: usize, height: usize, value: T) -> Self {
        let data = vec![value; width * height];
        Matrix { width, height, data }
    }

    // 获取矩阵的宽度
    pub fn width(&self) -> usize {
        self.width
    }

    // 获取矩阵的高度
    pub fn height(&self) -> usize {
        self.height
    }

    // 获取矩阵元素
    pub fn get(&self, x: usize, y: usize) -> Result<&T, MatrixError> {
        if x >= self.width || y >= self.height {
            Err(MatrixError::InvalidIndex(x, y, self.width * self.height))
        } else {
            Ok(&self.data[y * self.width + x])
        }
    }

    // 设置矩阵元素
    pub fn set(&mut self, x: usize, y: usize, value: T) -> Result<(), MatrixError> {
        if x >= self.width || y >= self.height {
            Err(MatrixError::InvalidIndex(x, y, self.width * self.height))
        } else {
            self.data[y * self.width + x] = value;
            Ok(())
        }
    }

    // 矩阵加法
    pub fn add(&self, other: &Matrix<T>) -> Result<Matrix<T>, MatrixError> {
        if self.width != other.width || self.height != other.height {
            return Err(MatrixError::InvalidAddDimensions(
                self.width,
                self.height,
                other.width,
                other.height,
            ));
        }
        let mut result = Matrix::new(self.width, self.height, T::default());
        for (i, v) in self.data.iter().enumerate() {
            result.data[i] = v.add(&other.data[i]);
        }
        Ok(result)
    }

    // 矩阵乘法
    pub fn multiply(&self, other: &Matrix<T>) -> Result<Matrix<T>, MatrixError>
    where
        T: std::ops::Mul<Output = T> + Copy + From<u8> + Default,
    {
        if self.height != other.width {
            return Err(MatrixError::InvalidDimensions(
                self.width,
                self.height,
                other.width,
                other.height,
            ));
        }
        let mut result = Matrix::new(self.width, other.height, T::default());
        for y in 0..self.height {
            for x in 0..self.width {
                for k in 0..other.height {
                    result.data[x + k * self.width] = result.data[x + k * self.width].mul(
                        &self.data[y * self.width + k]
                    )
                    .add(&other.data[k * other.width + x]);
                }
            }
        }
        Ok(result)
    }
}

// 矩阵打印实现
impl<T: Debug> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{:?} ", self.get(x, y).unwrap())?;
            }
            write!(f, "
")?;
        }
        Ok(())
    }
}

// 测试矩阵运算
#[tokio::main]
async fn main() {
    let matrix_a = Matrix::new(2, 2, 1);
    let matrix_b = Matrix::new(2, 2, 1);
    let sum = matrix_a.add(&matrix_b).unwrap();
    let product = matrix_a.multiply(&matrix_b).unwrap();

    println!("Sum: 
{}", sum);
    println!("Product: 
{}", product);
}
