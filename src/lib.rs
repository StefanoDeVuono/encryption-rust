extern crate ramp;
extern crate rand;

pub mod small_primes;
pub mod get_prime;
pub mod poem;

use ramp::traits::Integer;
use ramp::Int;

struct Keys {
    public: (Int, Int),
    private: Int,
}

fn _carmichael_of_primes(prime_1: &Int, prime_2: &Int) -> Int {
    let n:Int = prime_1 - 1;
    let m = prime_2 - 1;
    return (&n).lcm(&m);
}

fn _extended_euclid_algorithm(a: Int, n: Int) -> Int {
    let mut t = Int::zero();
    let mut new_t = Int::one();
    let mut r = n.clone();
    let mut new_r = a;
    while new_r != Int::zero() {
        let q = r.div_floor(&new_r);
        let temp_t = t;
        t = new_t;
        let result_t = temp_t - &q * &t;
        new_t = result_t;

        let temp_r = r;
        r = new_r;
        let result_r = temp_r - &q * &r;
        new_r = result_r;
    }
    if r > Int::one() {
        return Int::zero();
    }
    if t < Int::zero() {
        return t + n;
    }
    return t.clone();
}

fn _generate_keys(p: &Int,q: &Int) -> Keys {
    let n = p * q;
    let k = _carmichael_of_primes(p, q);
    let e = Int::from(65537); // TODO check that 65537 is not a factor of k

    let public = (e.clone(), n);
    let private = _extended_euclid_algorithm(e.clone(), k);
    return Keys { private, public };
}

fn encrypt(message: &Int, public: &(Int, Int)) -> Int {
    return Int::pow_mod(&message, &public.0, &public.1);
}

fn decrypt(encrypted_message: &Int, private: &Int, n: &Int) -> Int {
    return Int::pow_mod(&encrypted_message, &private, n);
}

fn _split_into(str: String, chunk_size: usize) -> Vec<String> {
    if str.len() <= chunk_size {
        return vec![str];
    }
    let (first, last) = str.split_at(chunk_size);
    let mut arr: (Vec<String>) = vec![first.to_string()];
    arr.append(&mut _split_into(last.to_string(), chunk_size));
    return arr;
}

fn _convert_to_int(str: &str) -> Int {
    let arr = str.as_bytes();

    let mut int_str = String::new();
    for byte in arr {
        int_str.push_str(&format!("{:02x}", byte));
    }
    return Int::from_str_radix(&int_str, 16).unwrap();
}

fn _convert_to_str(int: Int) -> String {
    let octal = int.to_str_radix(16, false);

    let arr = _split_into(octal, 2)
        .iter()
        .map(|el|  u8::from_str_radix(&el, 16).unwrap() )
        .collect::<Vec<u8>>();
    return String::from_utf8(arr).unwrap();
}

#[cfg(test)]
#[test]
fn test_carmichael_of_primes_12() {
    let p = &Int::from(3061);
    let q = &Int::from(3229);
    let expected = Int::from(823140);
    let result = _carmichael_of_primes(p, q);
    assert_eq!(expected, result);
}

#[test]
fn test_carmichael_of_primes_2() {
    let p = &Int::from(8081);
    let q = &Int::from(10663);
    let expected = Int::from(43074480);
    let result = _carmichael_of_primes(p, q);
    assert_eq!(expected, result);
}

#[test]
fn test_extended_euclid_algorithm () {
    let result_1 = _extended_euclid_algorithm(Int::from(3233), Int::from(17));
    let expected_1 = Int::from(6);
    assert_eq!(expected_1, result_1);

    let result_2 = _extended_euclid_algorithm(Int::from(43721), Int::from(15361));
    let expected_2 = Int::from(2881);
    assert_eq!(expected_2, result_2);
}

#[test]
fn test_convert_to_int(){
    let line = "I mean...I...can fly\nLike a bird in the sky";
    let expected = Int::from_str_radix("49206d65616e2e2e2e492e2e2e63616e20666c790a4c696b652061206269726420696e2074686520736b79", 16).unwrap();
    let result = _convert_to_int(line);
    assert_eq!(expected, result);
}

#[test]
fn test_convert_to_str(){
    let int = Int::from_str_radix("49206d65616e2e2e2e492e2e2e63616e20666c790a4c696b652061206269726420696e2074686520736b79", 16).unwrap();
    let expected = "I mean...I...can fly\nLike a bird in the sky";
    let result = _convert_to_str(int);
    assert_eq!(expected, result);
}


#[test]
fn test_generate_keys(){
    let p = Int::from(19);
    let q = Int::from(7);
    let keys = _generate_keys(&p, &q);


    assert_eq!(keys.public.0, Int::from(65537));
    assert_eq!(keys.public.1, Int::from(133));

    assert_eq!(keys.private, Int::from(17));
}

#[test]
fn test_encryption_and_decryption() {
    let p = get_prime::gen_prime();
    let q = get_prime::gen_prime();
    let keys = _generate_keys(&p, &q);
    let public = keys.public;
    let private = keys.private;

    let message = Int::from(43770);
    let encrypted_message = encrypt(&message, &public);
    let decoded = decrypt(&encrypted_message, &private, &public.1);

    assert_eq!(message, decoded);
}

#[test]
fn test_rsa() {
    let p = get_prime::gen_prime();
    let q = get_prime::gen_prime();
    let keys = _generate_keys(&p, &q);
    let public = keys.public;
    let private = keys.private;

    let message = _convert_to_int(poem::POEM);

    let encrypted = encrypt(&message, &(public));
    let decoded_as_ints = decrypt(&encrypted, &private, &public.1);

    println!("{}", encrypted);
    println!("{}", decoded_as_ints);

    let decoded = _convert_to_str(decoded_as_ints);

    assert_eq!(poem::POEM, decoded);
}