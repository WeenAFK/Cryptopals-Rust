use std::collections::HashSet;

/// Returns all (not necessarily prime) factors of x.
pub fn factors(x: usize) -> HashSet<usize> {
    let mut set = HashSet::new();
    set.insert(1); // all numbers divisible by 1
    set.insert(x); // all numbers divisible by themselves
    let max = x / 2;
    for i in 2..max {
        if x % i == 0 {
            set.insert(i);
        }
    }
    set
}
