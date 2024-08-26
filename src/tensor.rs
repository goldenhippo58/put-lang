use std::fmt;
use std::ops::{Add, Mul, Sub};

#[derive(Clone, Debug)]
pub struct Tensor {
    data: Vec<f64>,
    shape: Vec<usize>,
}

impl Tensor {
    pub fn new(data: Vec<f64>, shape: Vec<usize>) -> Self {
        assert_eq!(data.len(), shape.iter().product());
        Tensor { data, shape }
    }

    pub fn zeros(shape: Vec<usize>) -> Self {
        let size = shape.iter().product();
        Tensor {
            data: vec![0.0; size],
            shape,
        }
    }

    pub fn get(&self, indices: &[usize]) -> Option<f64> {
        let index = self.compute_index(indices)?;
        self.data.get(index).cloned()
    }

    pub fn set(&mut self, indices: &[usize], value: f64) -> Result<(), String> {
        let index = self.compute_index(indices).ok_or("Invalid indices")?;
        if let Some(elem) = self.data.get_mut(index) {
            *elem = value;
            Ok(())
        } else {
            Err("Index out of bounds".to_string())
        }
    }

    fn compute_index(&self, indices: &[usize]) -> Option<usize> {
        if indices.len() != self.shape.len() {
            return None;
        }
        let mut index = 0;
        let mut multiplier = 1;
        for (&dim, &idx) in self.shape.iter().zip(indices).rev() {
            if idx >= dim {
                return None;
            }
            index += idx * multiplier;
            multiplier *= dim;
        }
        Some(index)
    }

    pub fn matmul(&self, other: &Tensor) -> Result<Tensor, String> {
        if self.shape.len() != 2 || other.shape.len() != 2 {
            return Err("Both tensors must be 2-dimensional for matrix multiplication".to_string());
        }
        if self.shape[1] != other.shape[0] {
            return Err("Inner dimensions must match for matrix multiplication".to_string());
        }

        let m = self.shape[0];
        let n = other.shape[1];
        let p = self.shape[1];

        let mut result_data = vec![0.0; m * n];

        for i in 0..m {
            for j in 0..n {
                let mut sum = 0.0;
                for k in 0..p {
                    sum += self.data[i * p + k] * other.data[k * n + j];
                }
                result_data[i * n + j] = sum;
            }
        }

        Ok(Tensor::new(result_data, vec![m, n]))
    }

    pub fn transpose(&self) -> Tensor {
        if self.shape.len() != 2 {
            panic!("Transpose is currently only supported for 2D tensors");
        }
        let (rows, cols) = (self.shape[0], self.shape[1]);
        let mut new_data = vec![0.0; self.data.len()];
        for i in 0..rows {
            for j in 0..cols {
                new_data[j * rows + i] = self.data[i * cols + j];
            }
        }
        Tensor::new(new_data, vec![cols, rows])
    }

    pub fn apply<F>(&self, f: F) -> Tensor
    where
        F: Fn(f64) -> f64,
    {
        let new_data = self.data.iter().map(|&x| f(x)).collect();
        Tensor::new(new_data, self.shape.clone())
    }

    pub fn exp(&self) -> Tensor {
        self.apply(|x| x.exp())
    }

    pub fn log(&self) -> Tensor {
        self.apply(|x| x.ln())
    }

    pub fn mean(&self) -> f64 {
        self.data.iter().sum::<f64>() / self.data.len() as f64
    }

    pub fn variance(&self) -> f64 {
        let mean = self.mean();
        self.data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / self.data.len() as f64
    }

    pub fn std_dev(&self) -> f64 {
        self.variance().sqrt()
    }
}

impl Add for &Tensor {
    type Output = Tensor;

    fn add(self, other: &Tensor) -> Tensor {
        assert_eq!(self.shape, other.shape, "Shapes must match for addition");
        let data = self
            .data
            .iter()
            .zip(&other.data)
            .map(|(&a, &b)| a + b)
            .collect();
        Tensor::new(data, self.shape.clone())
    }
}

impl Sub for &Tensor {
    type Output = Tensor;

    fn sub(self, other: &Tensor) -> Tensor {
        assert_eq!(self.shape, other.shape, "Shapes must match for subtraction");
        let data = self
            .data
            .iter()
            .zip(&other.data)
            .map(|(&a, &b)| a - b)
            .collect();
        Tensor::new(data, self.shape.clone())
    }
}

impl Mul for &Tensor {
    type Output = Tensor;

    fn mul(self, other: &Tensor) -> Tensor {
        assert_eq!(
            self.shape, other.shape,
            "Shapes must match for element-wise multiplication"
        );
        let data = self
            .data
            .iter()
            .zip(&other.data)
            .map(|(&a, &b)| a * b)
            .collect();
        Tensor::new(data, self.shape.clone())
    }
}

impl fmt::Display for Tensor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tensor(shape={:?}, data={:?})", self.shape, self.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tensor_creation() {
        let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]);
        assert_eq!(t.shape, vec![2, 2]);
        assert_eq!(t.data, vec![1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn test_tensor_zeros() {
        let t = Tensor::zeros(vec![2, 3]);
        assert_eq!(t.shape, vec![2, 3]);
        assert_eq!(t.data, vec![0.0; 6]);
    }

    #[test]
    fn test_tensor_get() {
        let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]);
        assert_eq!(t.get(&[0, 0]), Some(1.0));
        assert_eq!(t.get(&[1, 1]), Some(4.0));
        assert_eq!(t.get(&[2, 0]), None);
    }

    #[test]
    fn test_tensor_set() {
        let mut t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]);
        assert!(t.set(&[0, 1], 5.0).is_ok());
        assert_eq!(t.get(&[0, 1]), Some(5.0));
        assert!(t.set(&[2, 0], 6.0).is_err());
    }

    #[test]
    fn test_tensor_add() {
        let t1 = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]);
        let t2 = Tensor::new(vec![5.0, 6.0, 7.0, 8.0], vec![2, 2]);
        let result = &t1 + &t2;
        assert_eq!(result.data, vec![6.0, 8.0, 10.0, 12.0]);
    }

    #[test]
    fn test_tensor_subtraction() {
        let t1 = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]);
        let t2 = Tensor::new(vec![5.0, 6.0, 7.0, 8.0], vec![2, 2]);
        let result = &t1 - &t2;
        assert_eq!(result.data, vec![-4.0, -4.0, -4.0, -4.0]);
    }

    #[test]
    fn test_tensor_element_wise_multiplication() {
        let t1 = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]);
        let t2 = Tensor::new(vec![5.0, 6.0, 7.0, 8.0], vec![2, 2]);
        let result = &t1 * &t2;
        assert_eq!(result.data, vec![5.0, 12.0, 21.0, 32.0]);
    }

    #[test]
    fn test_tensor_matrix_multiplication() {
        let t1 = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]);
        let t2 = Tensor::new(vec![5.0, 6.0, 7.0, 8.0], vec![2, 2]);
        let result = t1.matmul(&t2).unwrap();
        assert_eq!(result.data, vec![19.0, 22.0, 43.0, 50.0]);
        assert_eq!(result.shape, vec![2, 2]);
    }

    #[test]
    fn test_tensor_transpose() {
        let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], vec![2, 3]);
        let transposed = t.transpose();
        assert_eq!(transposed.shape, vec![3, 2]);
        assert_eq!(transposed.data, vec![1.0, 4.0, 2.0, 5.0, 3.0, 6.0]);
    }

    #[test]
    fn test_tensor_exp() {
        let t = Tensor::new(vec![0.0, 1.0, 2.0], vec![3]);
        let exp_t = t.exp();
        assert!((exp_t.data[0] - 1.0).abs() < 1e-6);
        assert!((exp_t.data[1] - std::f64::consts::E).abs() < 1e-6);
        assert!((exp_t.data[2] - std::f64::consts::E.powi(2)).abs() < 1e-6);
    }

    #[test]
    fn test_tensor_log() {
        let t = Tensor::new(
            vec![1.0, std::f64::consts::E, std::f64::consts::E.powi(2)],
            vec![3],
        );
        let log_t = t.log();
        assert!((log_t.data[0]).abs() < 1e-6);
        assert!((log_t.data[1] - 1.0).abs() < 1e-6);
        assert!((log_t.data[2] - 2.0).abs() < 1e-6);
    }

    #[test]
    fn test_tensor_mean() {
        let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]);
        assert_eq!(t.mean(), 2.5);
    }

    #[test]
    fn test_tensor_variance_and_std_dev() {
        let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]);
        assert!((t.variance() - 1.25).abs() < 1e-6);
        assert!((t.std_dev() - 1.118033988749895).abs() < 1e-6);
    }
}
