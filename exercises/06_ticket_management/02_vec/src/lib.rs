use std::{process::id, result};

// Given a number `n`, return the `n+1`th number in the Fibonacci sequence.
//
// The Fibonacci sequence is defined as follows:
//
// - The first number of the sequence is 0.
// - The second number of the sequence is 1.
// - Every subsequent number is the sum of the two preceding numbers.
//
// So the sequence goes: 0, 1, 1, 2, 3, 5, 8, 13, 21, and so on.
//
// We expect `fibonacci(0)` to return `0`, `fibonacci(1)` to return `1`,
// `fibonacci(2)` to return `1`, and so on.
pub fn fibonacci(n: u32) -> u32 {
    // TODO: implement the `fibonacci` function
    //
    // Hint: use a `Vec` to memoize the results you have already calculated
    // so that you don't have to recalculate them several times.
    // todo!()
    let fibonacci = Fibonacci::new(30);
    let idx = {n as usize};
    return fibonacci.get(idx);
}

struct Fibonacci {
    results: Vec<u32>
}

impl Fibonacci {
    pub fn new(n: usize) -> Self {
        let mut results = vec![0, 1, 1];
        for i in 3..=n {
            let result = results[i-1] + results[i-2];
            results.push(result)
        }
        return Fibonacci{results}
    }

    pub fn get(&self, idx: usize) -> u32 {
        self.results[idx]
    }
}

#[cfg(test)]
mod tests {
    use crate::fibonacci;

    #[test]
    fn first() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn second() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn tenth() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn thirthieth() {
        assert_eq!(fibonacci(30), 832040);
    }
}
