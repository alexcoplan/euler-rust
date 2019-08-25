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

    sum
}

fn p1() -> u64 {
    fizzbuzz_sum(1000)
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
        Some(self.x)
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
    p2_internal(4 * 1000 * 1000)
}


// }}}
// {{{ problem 3

fn isqrt_ceil(n : u64) -> u64 {
    (n as f64).sqrt().ceil() as u64
}

fn is_prime(n : u64) -> bool {
    let max = isqrt_ceil(n);
    for i in 2..max {
        if n % i == 0 {
            return false
        }
    }
    true
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
    largest_prime_factor(600_851_475_143)
}

// }}}
// {{{ problem 4

struct DigIter {
    x: u64,
}

impl DigIter {
    fn new(n: u64) -> DigIter {
        DigIter { x: n }
    }
}

impl Iterator for DigIter {
    type Item = u8;

    fn next(&mut self) -> Option <Self::Item> {
        if self.x == 0 {
            return None
        }

        let dig = (self.x % 10) as u8;
        self.x /= 10;
        Some(dig)
    }
}

fn collect_u8<I>(arr : &mut[u8], iter: I) -> usize
    where I: Iterator<Item = u8> {
    let mut i = 0;
    for x in iter {
        assert!(i < arr.len());
        arr[i] = x;
        i += 1;
    }
    i
}

fn is_palindrome(x : u64) -> bool {
    let mut digits = [0; 32];
    let n = collect_u8(&mut digits, DigIter::new(x));

    if n < 2 {
        return true;
    }

    let max = n/2 + 1;
    for i in 0..max {
        if digits[i] != digits[n-1-i] {
            return false;
        }
    }
    true
}

// Finds the largest palindrome made from the product of two
// n-digit numbers.
fn p4_internal(digits : u32) -> u64 {
    assert!(digits > 0);
    let base = 10_u64;
    let start = base.pow(digits - 1);
    let end = base.pow(digits);

    let mut ans = 0;

    for i in start..end {
        for j in i..end {
            let cand = i * j;
            if cand > ans && is_palindrome(cand) {
                ans = cand;
            }
        }
    }

    ans
}

fn p4() -> u64 {
    p4_internal(3)
}

// }}}
// {{{ problem 5

fn gcd(a : u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

fn lcm(a : u64, b: u64) -> u64 {
    (a / gcd(a,b)) * b
}

fn lcm_iter<I>(mut iter: I) -> u64
    where I: Iterator<Item = u64> {
    let mut ans = iter.next().unwrap();

    for x in iter {
        ans = lcm(ans, x);
    }
    ans
}

// Computes the smallest positive number that is divisible
// by all of the numbers from 1 to n inclusive.
fn p5_internal(n : u64) -> u64 {
    lcm_iter(1..=n)
}

fn p5() -> u64 {
    p5_internal(20)
}

// }}}
// {{{ problem 6

fn abs_diff(a : u64, b : u64) -> u64 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn p6_internal(n : u64) -> u64 {
    let sum : u64 = (1..=n).sum();
    let sum_of_squares : u64 = (1..=n).map(|x| x*x).sum();
    let square_of_sum = sum * sum;

    abs_diff(square_of_sum, sum_of_squares)
}

fn p6() -> u64 {
    p6_internal(100)
}

// }}}
// {{{ main

fn run_all() {
    let solutions = [
        p1,
        p2,
        p3,
        p4,
        p5,
        p6,
    ];

    for (i, soln) in solutions.iter().enumerate() {
        println!("Problem {}: {}", i+1, soln());
    }
}

fn main() {
    run_all();
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

    #[test]
    fn test_p4() {
        let mut digits = [0; 3];
        let iter = DigIter::new(123);
        assert_eq!(collect_u8(&mut digits, iter), 3);
        assert_eq!(digits, [3,2,1]);

        assert_eq!(is_palindrome(0), true);
        assert_eq!(is_palindrome(11), true);
        assert_eq!(is_palindrome(12), false);
        assert_eq!(is_palindrome(909), true);
        assert_eq!(is_palindrome(908), false);
        assert_eq!(is_palindrome(9009), true);
        assert_eq!(is_palindrome(9129), false);
        assert_eq!(p4_internal(2), 9009);
        assert_eq!(p4(), 906609);
    }

    #[test]
    fn test_p5() {
        assert_eq!(p5_internal(10), 2520);
        assert_eq!(p5(), 232792560);
    }

    #[test]
    fn test_p6() {
        assert_eq!(p6_internal(10), 2640);
        assert_eq!(p6(), 25164150);
    }
}

// }}}

// vim: foldmethod=marker
