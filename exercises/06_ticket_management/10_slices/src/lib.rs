// TODO: Define a function named `sum` that takes a reference to a slice of `u32` and returns the sum of all
//  elements in the slice.

// my solutions
// pub fn sum(s: &[i32]) -> i32 {
//     let mut r: i32 = 0;
//     for i in s.iter() {
//         r = r + *i;
//     }
//     return r;
// }

// better solutions
pub fn sum(slice: &[u32]) -> u32 {
    slice.iter().sum()
}

// support generic
// use std::ops::Add;

// pub fn sum<T>(s: &[T]) -> T
// where
//     T: Add<Output = T> + Default + Copy,
// {
//     let mut r: T = T::default(); // 使用 Default trait 来初始化 r
//     for &i in s.iter() {
//         r = r + i; // 加法运算
//     }
//     return r;
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let v = vec![];
        assert_eq!(sum(&v), 0);
    }

    #[test]
    fn one_element() {
        let v = vec![1];
        assert_eq!(sum(&v), 1);
    }

    #[test]
    fn multiple_elements() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(sum(&v), 15);
    }

    #[test]
    fn array_slice() {
        let v = [1, 2, 3, 4, 5];
        assert_eq!(sum(&v), 15);
    }
}
