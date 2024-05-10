#[derive(Debug)]
pub struct Matrix {
    dim: usize,
    data: Vec<i32>,
}

impl Matrix {
    pub fn from(dim: usize, data: Vec<i32>) -> Self {
        Self { dim, data }
    }

    pub fn random(dim: usize, cap: i32) -> Self {
        let data = (0..dim * dim).map(|_| rand::random::<i32>() % cap).collect();
        Self { dim, data }
    }
}

impl std::ops::Index<(usize, usize)> for Matrix {
    type Output = i32;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.data[i * self.dim + j]
    }
}

impl std::ops::Mul<Matrix> for Matrix {
    type Output = Self;

    #[cfg(not(feature = "single-threaded"))]
    fn mul(self, rhs: Self) -> Self {
        use std::thread;
        assert_eq!(self.dim, rhs.dim);
        let dim = self.dim;
        // result data to store.
        let mut result_data = vec![0; self.dim * self.dim];

        let lhs = std::sync::Arc::new(self);
        let rhs = std::sync::Arc::new(rhs);
        thread::scope(|s| {
            result_data
                .chunks_mut(dim)
                .enumerate()
                .map(|(index, data)| {
                    let lhs = lhs.clone();
                    let rhs = rhs.clone();
                    s.spawn(move || {
                        for j in 0..dim {
                            for k in 0..dim {
                                data[j] += lhs[(index, k)] * rhs[(k, j)];
                            }
                        }
                    })
                })
                .collect::<Vec<_>>()
                .into_iter()
                .for_each(|handle| {
                    handle.join().unwrap();
                });
        });

        Self {
            dim,
            data: result_data,
        }
    }

    #[cfg(feature = "single-threaded")]
    fn mul(self, rhs: Self) -> Self {
        assert_eq!(self.dim, rhs.dim);
        let dim = self.dim;
        // result data to store.
        let mut result_data = vec![0; self.dim * self.dim];

        for index in 0..dim {
            for j in 0..dim {
                for k in 0..dim {
                    result_data[index * dim + j] += self[(index, k)] * rhs[(k, j)];
                }
            }
        }

        Self {
            dim,
            data: result_data,
        }
    }
}

#[test]
fn test_matrix_mul() {
    let a = Matrix::from(2, vec![1, 2, 3, 4]);
    let b = Matrix::from(2, vec![5, 6, 7, 8]);
    let c = a * b;
    assert_eq!(c.data, vec![19, 22, 43, 50]);
}

#[test]
fn test_matrix_mul2(){
    let a = Matrix::from(3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let b = Matrix::from(3, vec![9, 8, 7, 6, 5, 4, 3, 2, 1]);
    let c = a * b;
    assert_eq!(c.data, vec![30, 24, 18, 84, 69, 54, 138, 114, 90])
}

