// file: random.rs

use rand::{self, distributions::{Distribution, Uniform}};

use std::iter;

/// Create an iterator that always returns a value randomly selected value from the provided
/// `slice`. The returned iterator will never return `None`.
///
/// `iter()` returns an iterator that infinitely chooses values as long as the iterator is alive:
/// ```
/// use correcthorse::random::iter;
///
/// let v = vec![1, 2, 3, 4, 5];
/// let mut it = iter(&v);
///
/// let x = it.nth(100000).unwrap();
/// assert!(v.contains(x));
///
/// let x = it.nth(1).unwrap();
/// assert!(v.contains(x));
/// ```
pub fn iter<'a, T>(list: &'a [T]) -> Box<dyn Iterator<Item = &'a T> + 'a> {
    Box::new(iter::from_fn(|| Some(choice(list))))
} // end fn random_iter()

/// Choose a uniformly-selected random value from the provided `choices`.
///
/// ```
/// use correcthorse::random::choice;
///
/// let v = vec![1, 2, 3, 4, 5];
/// assert!(v.contains(choice(&v)));
/// ```
pub fn choice<T>(choices: &[T]) -> &T {
    //let i = rand::random::<usize>() % choices.len();
    let uniform = Uniform::new(0, choices.len());
    let i = uniform.sample(&mut rand::thread_rng());
    choices.get(i).unwrap()//iter().nth(i).unwrap()
} // end fn choice()

#[cfg(test)]
mod tests {
    #[test]
    fn test_choice() {
        let v = vec![1, 2, 3, 5, 6, 7, 9, 10, 11];
        for _ in 0..1000 {
            let x = super::choice(v.as_slice());
            assert!(v.contains(x));
        }
    } // end test_choice

    #[test]
    fn test_iter() {
        let v = vec![1, 2, 3, 5, 6, 7, 9, 10, 11];
        let _ = super::iter(v.as_slice())
            .take(1000)
            .inspect(|x| assert!(v.contains(x)))
            .collect::<Vec<_>>();
    } // end test_iter
} // end mod tests

// random.rs
