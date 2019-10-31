extern crate ramp;
extern crate rand;

pub mod small_primes;

use ramp::traits::Integer;
use ramp::Int;
use ramp::RandomInt;
use rand::rngs::OsRng;

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

//fn find_large_prime() {}

fn carmichael_of_primes(prime_1: &Int, prime_2: &Int) -> Int {
    let divisor = gcd(&(prime_1 - 1), &(prime_2 - 1));
    return (prime_1 - 1) * (prime_2 - 1) / divisor;
}

struct Keys {
    public: Int,
    private: Int,
}

fn generate_keys(totient: &Int) -> Keys {
    let small_rand = 1; // TODO change this
    let large_rand = 10; // TODO change this
    let private = 1 + small_rand * totient;
    let public = 1 + large_rand * totient;
    return Keys { private, public };
}

fn encrypt(message: &Int, public: &Int, n: &Int) -> Int {
    return Int::pow_mod(&message, &public, n);
}

fn decrypt(encrypted_message: &Int, private: &Int, n: &Int) -> Int {
    return Int::pow_mod(&encrypted_message, &private, n);
}

fn gcd(u: &Int, v: &Int) -> Int {
    if u == v {
        return u.clone();
    }

    if u == &Int::zero() {
        return v.clone();
    }

    if v == &Int::zero() {
        return u.clone();
    }

    // look for factors of 2
    if u.is_even() {
        if v.is_odd() {
            return gcd(&(u >> 1), v);
        } else {
            return gcd(&(u >> 1), &(v >> 1)) << 1;
        }
    }

    if v.is_even() {
        // u is odd, v is even
        return gcd(u, &(v >> 1));
    }

    // reduce larger argument
    if u > v {
        return gcd(&((u - v) >> 1), v);
    }

    return gcd(&((v - u) >> 1), u);
}

fn gen_prime() -> Int {
    let mut rng = OsRng::new().expect("Failed to get OS random generator");
    let n = 1024;
    let mut candidate = rng.gen_uint(n);
    candidate.set_bit(0, true);
    candidate.set_bit((n - 1) as u32, true);
    if is_prime(&candidate) == true {
        return candidate;
    }
    return gen_prime();
}

fn convert_to_int(str: String) -> Int {
    let bytes = str.as_bytes().to_vec();

    let joined = bytes
        .iter()
        .map(|&el| format!("{:03o}", el))
        .collect::<Vec<String>>()
        .join("");

    return Int::from_str_radix(&joined, 8).expect("damn");
}

fn split_into_threes(str: String) -> Vec<String> {
    if str.len() <= 3 {
        return vec![str];
    }
    let (first, last) = str.split_at(3);
    let mut arr: (Vec<String>) = vec![first.to_string()];
    arr.append(&mut split_into_threes(last.to_string()));
    return arr;
}

fn convert_to_str(int: Int) -> String {
    let octal = int.to_str_radix(8, false);
    let arr = split_into_threes(octal)
        .iter()
        .map(|el| u8::from_str_radix(&el, 8).unwrap())
        .collect::<Vec<u8>>();
    return String::from_utf8(arr).unwrap();
}

#[cfg(test)]
#[test]
fn test_convert_to_int() {
    let str = "String and back again.";

    let partial_result = convert_to_int(str.to_string());
    let arr: (Vec<u8>) = vec![
        83, 116, 114, 105, 110, 103, 32, 97, 110, 100, 32, 98, 97, 99, 107, 32, 97, 103, 97, 105,
        110, 46,
    ];
    let joined = arr
        .iter()
        .map(|&el| format!("{:03o}", el))
        .collect::<Vec<String>>()
        .join("");
    let expected = Int::from_str_radix(&joined, 8).expect("damn");

    assert_eq!(expected, partial_result);

    let final_result = convert_to_str(partial_result);
    assert_eq!(str.to_string(), final_result);
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
    println!("possible prime");
    println!("{}", possible_prime);
    println!("");
    let mut i = Int::from(5);
    while (&i.square()) <= &possible_prime {
        // println!("{}", i);
        // println!("");
        if &possible_prime % &i == 0 || &possible_prime % (&i + 2) == 0 {
            return assert!(false);
        }
        i = &i + 6;
    }
    return assert!(true);
}

#[test]
fn test_gcd_12() {
    let p = &Int::from(3060);
    let q = &Int::from(3228);
    let divisor = gcd(p, q);
    let expected = Int::from(12);
    assert_eq!(expected, divisor);
}

#[test]
fn test_gcd_2() {
    let p = &Int::from(8080);
    let q = &Int::from(10662);
    let divisor = gcd(p, q);
    let expected = Int::from(2);
    assert_eq!(expected, divisor);
}

#[test]
fn test_carmichael_of_primes_12() {
    let p = &Int::from(3061);
    let q = &Int::from(3229);
    let expected = Int::from(823140);
    let result = carmichael_of_primes(p, q);
    assert_eq!(expected, result);
}

#[test]
fn test_carmichael_of_primes_2() {
    let p = &Int::from(8081);
    let q = &Int::from(10663);
    let expected = Int::from(43074480);
    let result = carmichael_of_primes(p, q);
    assert_eq!(expected, result);
}

#[test]
fn test_rsa() {
    let p = &Int::from(3061);
    let q = &Int::from(3229);
    let n = p * q;
    let k: Int = carmichael_of_primes(p, q);

    let keys = generate_keys(&k);
    let public = keys.public;
    let private = keys.private;

    let message = Int::from(43770);
    let encrypted_message = encrypt(&message, &public, &n);
    let decoded = decrypt(&encrypted_message, &private, &n);
    let x = Int::to_str_radix(&decoded, 10, false);
    println!("{}", x);
    assert_eq!(message, decoded);
}

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
fn test_encryption_and_decryption() {
    let poem = "To fling my arms wide
                In some place of the sun,
                To whirl and to dance
                Till the white day is done.
                Then rest at cool evening
                Beneath a tall tree
                While night comes on gently,
                    Dark like me—
                That is my dream!";

    let p = &gen_prime();
    let q = &gen_prime();
    let n = p * q;
    let k: Int = carmichael_of_primes(p, q);

    let keys = generate_keys(&k);
    let public = keys.public;
    let private = keys.private;

    let message = convert_to_int(poem.to_string());

    let encrypted = encrypt(&message, &public, &n);
    let decoded_as_ints = decrypt(&encrypted, &private, &n);
    let decoded = convert_to_str(decoded_as_ints);
    println!("{}", decoded);
    assert_eq!(poem, decoded);
}
