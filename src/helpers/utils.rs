use std::{
    cmp::{max, min},
    slice::ChunksExact,
};

/// given two input numbers, returns them in a tuple (min, max)
pub fn min_max<T: Ord + Copy>(a0: T, a1: T) -> (T, T) {
    (min(a0, a1), max(a0, a1))
}

/// from a collection, returns an iterator over a slice in (non-overlapping) chunks of 2 elements
/// internally, uses the `chuncks` method from the Slice.
pub fn pairs<T>(input: &[T]) -> ChunksExact<T> {
    input.chunks_exact(2)
}

/// from a collection, returns an iterator of pairs. Uses internally `zip` iterator adapter.
pub fn pairs_zip<T>(input: &[T]) -> impl Iterator<Item = (&T, &T)> {
    let iter1 = input.iter().step_by(2);
    let iter2 = input.iter().skip(1).step_by(2);
    iter1.zip(iter2)
}

/// based on Matklad code, available on <https://internals.rust-lang.org/t/representing-difference-between-unsigned-integers/13563/12>
/// An enum to permite subtract negative deltas or step from unsigned numbers.
pub mod delta {
    use std::ops::{Add, AddAssign, Sub, SubAssign};
    #[derive(Copy, Clone)]
    pub enum Delta<T> {
        Add(T),
        Sub(T),
    }

    impl<T: Ord + Sub<Output = T>> Delta<T> {
        pub fn new(old: T, new: T, step: T) -> Delta<T> {
            if new > old {
                Delta::Add(step)
            } else {
                Delta::Sub(step)
            }
        }
    }

    // This won't be coherent :-(
    // impl<T: AddAssign + SubAssign> AddAssign<Delta<T>> for T
    impl AddAssign<Delta<usize>> for usize {
        fn add_assign(&mut self, rhs: Delta<usize>) {
            match rhs {
                Delta::Add(amt) => *self += amt,
                Delta::Sub(amt) => *self -= amt,
            }
        }
    }

    impl AddAssign<Delta<u16>> for u16 {
        fn add_assign(&mut self, rhs: Delta<u16>) {
            match rhs {
                Delta::Add(amt) => *self += amt,
                Delta::Sub(amt) => *self -= amt,
            }
        }
    }
}
