extern crate ramp;
extern crate rand;

use crate::small_primes;

use ramp::Int;
use ramp::RandomInt;
use rand::rngs::OsRng;


pub fn gen_prime() -> Int {
    let mut rng = OsRng::new().expect("Failed to get OS random generator");
    let n = 2048;
    let mut candidate = rng.gen_uint(n);
    candidate.set_bit(0, true);
    candidate.set_bit((n - 1) as u32, true);
    if is_prime(&candidate) == true {
        return candidate;
    }
    return gen_prime();
}

fn div_small_primes(numb: &Int) -> bool {
    for p in small_primes::SMALL_PRIMES.iter() {
        let prime = &Int::from(*p);
        if numb == prime {
            return true;
        }
        if numb % prime == 0 {
            return false;
        }
    }
    true
}

fn little_fermat(candidate: &Int) -> bool {
    let mut rng = OsRng::new().expect("Failed to get OS random generator");
    let random: Int = rng.gen_uint_below(candidate);
    let result = Int::pow_mod(&random, &(candidate - &Int::one()), candidate);
    result == Int::one()
}

fn rewrite(num: &Int) -> (Int, Int) {
    let mut ess = 0;
    while num.bit(ess) {
        ess += 1;
    }
    let dee = num >> (ess as usize);
    (Int::from(ess), dee)
}

fn miller_rabin(candidate: &Int, limit: usize) -> bool {
    let mut rng = OsRng::new().expect("Failed to get OS random generator");
    let (s, d) = rewrite(&(candidate - &Int::one()));
    let one = Int::one();
    let two = &one + &one;
    for _ in 0..limit {
        let basis = rng.gen_int_range(&two, &(candidate - &two));
        let mut y = Int::pow_mod(&basis, &d, candidate);

        if y == one || y == (candidate - &one) {
            continue;
        } else {
            for _ in one.clone()..s - one.clone() {
                y = Int::pow_mod(&y, &two, candidate);
                if y == one {
                    return false;
                } else if y == candidate - &one {
                    break;
                }
            }
            return false;
        }
    }
    true
}

fn is_prime(candidate: &Int) -> bool {
    // First, simple trial divide
    if !div_small_primes(candidate) {
        return false;
    }

    // Second, Fermat's little theo test on the candidate
    if !little_fermat(candidate) {
        return false;
    }

    // Finally, Miller-Rabin test
    if !miller_rabin(candidate, 5) {
        return false;
    }
    true
}

#[cfg(test)]
#[test]
fn test_div_small_primes() {
    let prime = Int::from(1303);
    let other_prime = Int::from(17881);
    let not_prime = Int::from(17883);
    let mut result = div_small_primes(&prime);
    assert_eq!(result, true);
    result = div_small_primes(&other_prime);
    assert_eq!(result, true);
    result = div_small_primes(&not_prime);
    assert_eq!(result, false);
}

#[test]
fn test_little_fermat() {
    let prime = Int::from(492876847);
    let not_prime = Int::from(492876849);
    let mut result = little_fermat(&prime);
    assert_eq!(result, true);
    result = little_fermat(&not_prime);
    assert_eq!(result, false);
}

#[test]
fn test_miller_rabin() {
    let prime = Int::from(492876847);
    let not_prime = Int::from(492876849);
    let mut result = miller_rabin(&prime, 5);
    assert_eq!(result, true);
    result = miller_rabin(&not_prime, 5);
    assert_eq!(result, false);
}

#[test]
fn test_titanic_prime() {
    let ten = Int::from(10);
    let nine_nine_nine = 999 as usize;
    let seven = Int::from(7);
    let thirteen = Int::from(13);
    let prime = ten.pow(nine_nine_nine) + seven;
    let not_prime = ten.pow(nine_nine_nine) + thirteen;
    let mut result = is_prime(&prime);
    assert_eq!(result, true);
    result = is_prime(&not_prime);
    assert_eq!(result, false);
}


#[test]
#[ignore] // it is too expensive
fn test_gen_prime_trial_division() {
    let possible_prime = gen_prime();
    if possible_prime <= 3 {
        return assert!(possible_prime > 1);
    } else if &possible_prime % 2 == 0 || &possible_prime % 3 == 0 {
        return assert!(false);
    }
    let mut i = Int::from(5);
    while (&i.square()) <= &possible_prime {
        if &possible_prime % &i == 0 || &possible_prime % (&i + 2) == 0 {
            return assert!(false);
        }
        i = &i + 6;
    }
    return assert!(true);
}
