use std::collections::BTreeSet;

pub fn primes_up_to(num: i32) -> Vec<i32> {
    let mut primes: BTreeSet<i32> = BTreeSet::new();
    let mut nonprimes: BTreeSet<i32> = BTreeSet::new();
    for n in 2..(num+1) {
        if nonprimes.contains(&n) {
            continue;
        } else {
            primes.insert(n);
            let mut multiple = n;
            while multiple <= num {
                multiple += n;
                nonprimes.insert(multiple);
            }
        }
    }
    primes.iter().map(|n| n.clone()).collect()
}
