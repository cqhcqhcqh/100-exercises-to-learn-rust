// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    // todo!()
    let mid = v.len()/2;
    let v1 = &v[..mid];
    let v2 = &v[mid..];
    
    return thread::scope(|scope| {
        let handler1 = scope.spawn(|| {
            return v1.into_iter().sum::<i32>();
        });

        let handler2 = scope.spawn(|| {
            return v2.into_iter().sum::<i32>();
        });

        return handler1.join().unwrap() + handler2.join().unwrap();
    });
}

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
