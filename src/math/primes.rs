pub fn prime_table(upto: usize) -> Vec<bool> {
    let mut pr = vec![true; upto + 1];
    pr[0] = false;
    pr[1] = false;
    for i in 2..=upto {
        if pr[i] {
            for j in (i + i..=upto).step_by(i) {
                pr[j] = false;
            }
        }
    }
    pr
}

pub fn primes(upto: usize) -> Vec<u32> {
    let is_prime = prime_table(upto);
    (2..=upto).filter(|&n| is_prime[n]).map(|n| n as u32).collect()
}
