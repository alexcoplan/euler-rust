// Solutions to some Project Euler problems in Rust.
// Primarily a learning exercise to help learn me a Rust.

// {{{ problem 1

fn fizzbuzz_sum(n : u64) -> u64 {
    let mut sum = 0;
    for i in 0..n {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }

    return sum;
}

fn p1() -> u64 {
    return fizzbuzz_sum(1000);
}

// }}}
// {{{ problem 2

// Let's implement an iterator for the fibonacci numbers.

struct FibIter {
    x: u64,
    y: u64,
}

impl FibIter {
    fn new() -> FibIter {
        FibIter { x: 1, y: 1 }
    }
}

impl Iterator for FibIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let nxt = self.x + self.y;

        self.x = self.y;
        self.y = nxt;
        return Some(self.x);
    }
}

// Sum of even-valued fibonacci numbers not exceeding max_term.
fn p2_internal(max_term : u64) -> u64 {
    let fib = FibIter::new();
    let mut sum = 0;

    for k in fib {
        if k > max_term {
            return sum;
        }
        if k % 2 != 0 {
            continue;
        }

        sum += k;
    }

    panic!();
}

fn p2() -> u64 {
    return p2_internal(4 * 1000 * 1000);
}


// }}}
// {{{ main

fn main() {
    println!("Problem 1: {}", p1());
    println!("Problem 2: {}", p2());
}

// }}}
// {{{ unit tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(fizzbuzz_sum(10), 23);
        assert_eq!(p1(), 233168); 
    }

    #[test]
    fn test_p2() {
        let mut iter = FibIter::new();

        let answers = [1, 2, 3, 5, 8, 13, 21, 34, 55, 89];
        for &ans in answers.iter() {
            assert_eq!(iter.next(), Some(ans))
        }

        assert_eq!(p2_internal(89), 44);
        assert_eq!(p2(), 4613732);
    }
}

// }}}

// vim: foldmethod=marker
