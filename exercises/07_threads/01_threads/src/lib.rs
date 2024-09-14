// TODO: implement a multi-threaded version of the `sum` function
//  using `spawn` and `join`.
//  Given a vector of integers, split the vector into two halves and
//  sum each half in a separate thread.

// Caveat: We can't test *how* the function is implemented,
// we can only verify that it produces the correct result.
// You _could_ pass this test by just returning `v.iter().sum()`,
// but that would defeat the purpose of the exercise.
//
// Hint: you won't be able to get the spawned threads to _borrow_
// slices of the vector directly. You'll need to allocate new
// vectors for each half of the original vector. We'll see why
// this is necessary in the next exercise.
use std::thread;

// My IMPL
pub fn sum(v: Vec<i32>) -> i32 {
    // todo!()
    // let v: Vec<i32> = v.clone();
    let v: &'static Vec<i32> = Box::leak(Box::new(v.clone()));
    let v_len = v.len();
    let half_v: &[i32] = &v[0..v_len/2];
    let next_half_v = &v[v_len/2..];

    let halfHandler = thread::spawn(|| {
        let half_total: i32 = half_v.iter().sum();
        return half_total;
    });

    let nextHalfHandler = thread::spawn(|| {
        let half_total: i32 = next_half_v.iter().sum();
        return half_total;
    });

    return halfHandler.join().unwrap() + nextHalfHandler.join().unwrap();
}

// Solution IMPL
// pub fn sum1(v: Vec<i32>) -> i32 {
//     let v_len = v.len();
//     let (v1, v2) = v.split_at(v_len/2);
//     let v1 = v1.into_iter();
//     let v2 = v2.into_iter();
//     let handle1 = thread::spawn(move || { v1.into_iter().sum::<i32>()});
//     let handle2 = thread::spawn(move || { v2.into_iter().sum::<i32>()});
//     return handle1.join().unwrap() + handle2.join().unwrap();
// }
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
