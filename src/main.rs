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
// {{{ problem 3

fn isqrt_ceil(n : u64) -> u64 {
    return (n as f64).sqrt().ceil() as u64;
}

fn is_prime(n : u64) -> bool {
    let max = isqrt_ceil(n);
    for i in 2..max {
        if n % i == 0 {
            return false
        }
    }

    return true
}

fn largest_prime_factor(n : u64) -> u64 {
    // Start with ceil(sqrt(n)) and work downwards as this is the largest
    // possible non-trivial prime factor.
    let start = isqrt_ceil(n);
    for i in (2..start).rev() {
        if n % i != 0 {
            continue; // not a factor
        }

        if is_prime(i) {
            return i;
        }

        // XXX: could maybe optimise here by dividing through by i and keeping track of i's
        // factorisation.
    }

    panic!();
}

fn p3() -> u64 {
    return largest_prime_factor(600851475143)
}

// }}}
// {{{ main

fn main() {
    let solutions = [
        p1,
        p2,
        p3,
    ];

    for (i, soln) in solutions.iter().enumerate() {
        println!("Problem {}: {}", i+1, soln());
    }
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

    #[test]
    fn test_p3() {
        assert_eq!(largest_prime_factor(13195), 29);
        assert_eq!(p3(), 6857);
    }
}

// }}}

// vim: foldmethod=marker
