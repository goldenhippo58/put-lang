use std::fmt;
use std::ops::Add;

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
}
