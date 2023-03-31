mod tests;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut candidates: Vec<(u64, bool)> = [(2_u64, if upper_bound < 2 {false} else {true})]
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
