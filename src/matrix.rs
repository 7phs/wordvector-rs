pub struct Matrix<T> {
    data: Vec<T>,
    sz: usize,
}

impl<T> Matrix<T>
where T: Clone + Default
{
    pub fn new(sz: usize) -> Matrix<T> {
        let mut data: Vec<T> = Vec::new();
        data.resize(sz * sz, T::default());

        Matrix {
            data,
            sz,
        }
    }

    pub fn as_matrix_mut(&mut self) -> Vec<&mut [T]> {
        self.data.chunks_mut(self.sz).collect()
    }

    pub fn as_matrix(&self) -> Vec<&[T]> {
        self.data.chunks(self.sz).collect()
    }
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn test_matrix() {
        let mut matrix: Matrix<f32> = Matrix::new(10);

        {
            let mut m = matrix.as_matrix_mut();

            m[3][4] = 0.15f32;
            m[2][5] = 15.1024f32;
        }

        let m = matrix.as_matrix();
        assert_eq!(m[3][4], 0.15f32, "check item [3][4]");
        assert_eq!(m[2][5], 15.1024f32, "check item [2][5]");
    }
}