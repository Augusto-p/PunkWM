use num_bigint::{BigUint, RandBigInt};
use num_traits::{One, Zero};
use rand::thread_rng;

fn is_probably_prime(n: &BigUint, k: u32) -> bool {
    if *n < BigUint::from(4u32) {
        return *n == BigUint::from(2u32)
            || *n == BigUint::from(3u32);
    }

    if n % 2u32 == BigUint::zero() {
        return false;
    }

    let one = BigUint::one();
    let two = BigUint::from(2u32);

    let mut d = n - &one;
    let mut r = 0;

    while &d % &two == BigUint::zero() {
        d /= &two;
        r += 1;
    }

    let mut rng = thread_rng();

    'outer: for _ in 0..k {
        let a = rng.gen_biguint_range(&two, &(n - &two));

        let mut x = a.modpow(&d, n);

        if x == one || x == n - &one {
            continue;
        }

        for _ in 0..(r - 1) {
            x = x.modpow(&two, n);

            if x == n - &one {
                continue 'outer;
            }
        }

        return false;
    }

    true
}

pub fn generate_prime(bits: u64) -> BigUint {
    let mut rng = thread_rng();

    loop {
        let mut p = rng.gen_biguint(bits);

        // hacerlo impar
        p |= BigUint::one();

        if is_probably_prime(&p, 20) {
            return p;
        }
    }
}
