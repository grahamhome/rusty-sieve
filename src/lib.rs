mod tests;

// First attempt
pub fn primes_up_to_v1(upper_bound: u64) -> Vec<u64> {
    let mut candidates: Vec<(u64, bool)> = [(2_u64, if upper_bound < 2 { false } else { true })]
        .into_iter()
        .chain((3..=upper_bound).step_by(2).map(|c| (c, true)))
        .collect();
    let mut p: u64 = 3;
    while p.pow(2) <= upper_bound {
        let candidates_to_mark: Vec<u64> = (p..=upper_bound).step_by(p as usize * 2).collect();
        candidates = candidates
            .iter_mut()
            .map(|(c, is_prime)| {
                if c != &p && candidates_to_mark.contains(c) {
                    (*c, false)
                } else {
                    (*c, *is_prime)
                }
            })
            .collect();
        p = candidates.iter().find(|c| c.0 > p && c.1).unwrap().0;
    }
    candidates
        .into_iter()
        .filter_map(|(c, is_prime)| if is_prime { Some(c) } else { None })
        .collect()
}

/// A more concise solution
pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut sieve = vec![true; (upper_bound + 1) as usize];
    for n in 2..upper_bound {
        if n.pow(2) <= upper_bound {
            let mut p = n * 2;
            while p <= upper_bound {
                sieve[p as usize] = false;
                p += n;
            }
        }
    }
    sieve
        .iter()
        .enumerate()
        .skip(2)
        .filter_map(|(i, p)| if *p { Some(i as u64) } else { None })
        .collect()
}
