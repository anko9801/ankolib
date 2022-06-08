use std::ops::{Add, AddAssign, Index, IndexMut, Mul};

// MatrixNumber という trait があるらしい
struct Matrix2D(Vec<Vec<isize>>);

// row: 行 column: 列
impl Matrix2D {
    pub fn new(nums: &Vec<Vec<isize>>) -> Self {
        if nums.len() == 0 {
            return Self(vec![]);
        }
        let row_num = nums[0].len();
        for column in nums {
            if column.len() != row_num {
                panic!("The matrix is not aligned.");
            }
        }
        Self(nums.clone())
    }

    pub fn nrows(&self) -> usize {
        self.0.len()
    }

    pub fn ncols(&self) -> usize {
        match self.nrows() {
            0 => 0,
            _ => self[0].len(),
        }
    }

    pub fn check_size(&self, rhs: &Self) {
        if self.nrows() != rhs.nrows() {
            panic!("number of rows is not matched.");
        }
        if self.ncols() != rhs.ncols() {
            panic!("number of columns is not matched.");
        }
    }
}

impl Add for Matrix2D {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self {
        self.check_size(&rhs);

        for i in 0..self.nrows() {
            for j in 0..self.ncols() {
                self[i][j] += rhs[i][j];
            }
        }
        self
    }
}

impl AddAssign for Matrix2D {
    fn add_assign(&mut self, rhs: Self) {
        self.check_size(&rhs);

        for i in 0..self.nrows() {
            for j in 0..self.ncols() {
                self[i][j] += rhs[i][j];
            }
        }
    }
}

impl Mul for Matrix2D {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        self.check_size(&rhs);
        let row = self.nrows();
        let column = self.ncols();

        let mut res = Matrix2D::new(&vec![vec![0, column as isize]; row]);
        for j in 0..self.ncols() {
            for i in 0..rhs.ncols() {
                for k in 0..self.nrows() {
                    res[i][j] += self[i][k] * rhs[k][j];
                }
            }
        }
        res
    }
}

impl Index<usize> for Matrix2D {
    type Output = Vec<isize>;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Matrix2D {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Vec<isize> {
        &mut self.0[index]
    }
}
