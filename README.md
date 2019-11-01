Three steps:

1. Find two large primes _p_ and _q_.
1. Compute _n = pq_ and _k = φ(n)_.
   - _φ_ is the totient. _φ(n)_ is the number of integers k from 1 to n (inclusive) for which the greatest common divisor is 1.
   - The number 9, for example, has 6 totatives [1,2,4,5,7,8]. 3 and 6 share with 9 the greatest common divisor of 3 and 9 and 9 share the greatest common divisor of 9. _φ(9) = 6_.
   - The totient of a prime _p_ is always _p - 1_, since all positive integers less than it are relatively prime.
   - The totient of a product is the product of the totients: _φ(mn) = φ(m)φ(n)_ (in our case, _(p - 1)\*( q - 1)_)
1. Find two numbers e* and \_d* such that _ed === 1 (mod k)_.
   - what does this mean?
   - it means that _e_ is _1 + ik_ and _d_ is _1 + jk_, where _j_ and _i_ are integers.
1. If we have a message _m_ smaller than our _n_ (the product of _p_ and _q_ from above), we can encrypt it with _m^e % n \_and decrypt the result with _(m^e % n)^d % n\_


### Other random stuff
say we want to find the ones place of 7^222, aka 7^222 (mod 10)
7 and 10 are coprime and φ(10) is 4
7^4=== 1 (mod 10)

=== means congruence
_a_ and _b_ are congruent modulo _n_
if _a - b_ is divisible by _n_

eg 37 === 57 (mod 10), since 37-57 is -20 which is a multiple of 10, or
since they both have remainders of 7 when divided by 10
Chinese remainder theorem

ed === 1 (mod k)

e === 1 (mod k)
AND
d === 1 (mod k)

primes p = 7, q = 11
_n = pq_ is 77 and _k = φ(n)_ is 60 = φ(77)

_ed === 1 (mod 77)_

78 and 155

const congruence = (mod) => {

const e = 1 + mod
const d = 1 + (2 \* mod)

return {e, d}
}

p=61 and q=53
n=3233
k = 60\*52 = 3120

d = 1 + 3120
e = 1 + 2\*3120

message m ("hello" = 43770)
secret = m^e (mod n)
text = secret^d (mod n)

p =
131
q=127

p = 3061
q = 3229
e_totient = (p-1)\*(q-1)

totient = ((p-1) % 4 === 0 && (q-1) % 4 === 0 ) ? e_totient /4 : e_totient

### Relationship between an Euler totient _φ(p)_ and a Carmichael function _λ(p)_, where p is prime
1. For a prime number _p_, the Euler totient is _p - 1_.
    - if _p_ is prime _φ(p) = p - 1_
    - if _p_ and _q_ are prime _φ(pq) = φ(p)φ(q)_
    - if _p_ and _q_ are prime _φ(pq) = (p - 1)(q - 1)_    
1. For a prime number _p_, the Carmichael function is also _p - 1_.
     - if _p_ is prime _λ(p) = p - 1_
     - if _p_ and _q_ are prime, is this true? _λ(pq) = φ(p)φ(q) / gcd(p -1, q -1)_ 
     - if _p_ and _q_ are prime, is this true? _λ(pq) = (p - 1)(q - 1) / gcd(p - 1, q - 1)_


    - OR if _p_ and _q_ are prime, is this true? _λ(pq) = φ(p)φ(q) / gcd(p,q)_, where gcd is 2^k
      NO _φ(3061 * 3229) = 9877680_ and _λ(3061 * 3229) = 823140_. _9877680 = 823140 * 12_

    Maybe garbage:
    - if _p - 1 = m * 2^k_ and _q - 1 = n * 2^k_

    - might a prime p be defined as  _m * 2^k + 1_
    - wilson: can a factorial be described as _x ! = m * 2^k_?, of course, since 2^1... 2^n... x



if (p- 1) and (q - 1) are not coprime
then there is an _a_ such that _a|(p - 1)_ and _a|(q - 1)_

the least common multiple of (n,m) is a prime
the carmichael of that is the totient

_λ(lcm(a,b))=lcm(λ(a),λ(b))_
_λ(
    
)=lcm(λ(a),λ(b))_
