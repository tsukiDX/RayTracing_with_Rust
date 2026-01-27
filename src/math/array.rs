use std::ops::{Index, IndexMut};

#[derive(Copy, Clone)]
pub struct Array<T, const N: usize> {
    pub data: [T; N]
}

impl<T: Copy + Default, const N: usize> Array<T, N> {
    pub fn new() -> Self {
        Self {
            data: [T::default(); N]
        }
    }
}

impl<T, const N: usize> Index<usize> for Array<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for Array<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut  self.data[index]
    }
}