Three steps:

1. Find two large primes _p_ and _q_.
1. Compute _n = pq_ and _k = φ(n)_.
   - _φ_ is the totient. _φ(n)_ is the number of integers k from 1 to n (inclusive) for which the greatest common divisor is 1.
   - The number 9, for example, has 6 totatives [1,2,4,5,7,8]. 3 and 6 share with 9 the greatest common divisor of 3 and 9 and 9 share the greatest common divisor of 9. _φ(9) = 6_.
   - The totient of a prime _p_ is always _p - 1_, since all positive integers less than it are relatively prime.
   - The totient of a product is the product of the totients: _φ(mn) = φ(m)φ(n)_ (in our case, _(p - 1)\*( q - 1)_)
   - We can use carmichael which is smaller number that meets the requirement that (something happen). The carmichael of a number n (defined as  _λ(n)_) is the smallest number m that any of n's totatives a will work in the following formula _a^m % n = 1_. The carmichael of the product of two primes  λ(pq) is the lcm((p-1)(n-1)). See proof*
1. Find two numbers e* and \_d* such that _ed === 1 (mod k)_.
   - what does this mean?
   - it means that _e_ is _1 + ik_ and _d_ is _1 + jk_, where _j_ and _i_ are integers.
1. If we have a message _m_ smaller than our _n_ (the product of _p_ and _q_ from above), we can encrypt it with _c = m^e % n \_and decrypt the result with _c^d % n\_
