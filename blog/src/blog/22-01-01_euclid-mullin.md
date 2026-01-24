<!--
date: "2022-01-01"
tags: [math]
-->

# Euclid-Mullin Sequence

I want to talk about a cute-little sequence called Euclid-Mullin Sequence. This sequence comes from a thousand year-old proof on infinitude of primes!

Well, what is a prime number to begin with? They are defined as numbers $n$ such that only $n$ and $1$ can divide them with no remainder. They are quite famous 🤓

For example, $7$ can only be divided by $7$ and $1$, nothing else. $1$ is not considered to be a prime because it makes things a lot more convenient, but there are those who consider it a prime sometimes.

> To really jump into the world of primes, see this video by [Richard Borcherds](https://www.youtube.com/watch?v=VRrP4US7idg&list=PL8yHsr3EFj53L8sMbzIhhXSAOpuZ1Fov8&index=5), a field medalist!

It was long wondered whether there were infinitely many primes. One of the oldest proofs in the book ([literally](https://en.wikipedia.org/wiki/Proofs_from_THE_BOOK)) is Euclid's proof that shows indeed there are infinitely many primes.

1. Assume that the set of primes $P$ is finite: $P = {p_1, p_2, ..., p_k}$.
2. Calculate a new integer $n = p_1 . p_2 . ... . p_k + 1$.
3. We can then argue that there exists a smallest factor $q$ such that $1 < q <= n$. If $q$ were to divide any of the primes in $P$, it would have to divide $1$ too. Since this is not possible, $q$ must be prime.

If we start with a finite set $P = {2}$, and iteratively find the smallest factor of the number that is the product of our primes plus one, we get $3, 7, 43, 13, 53, 5, 6221671, 38709183810571, 139, \ldots$. This is called the [Euclid-Mullin Sequence](https://en.wikipedia.org/wiki/Euclid%E2%80%93Mullin_sequence) (see [OEIS A000945](https://oeis.org/A000945)). A very intriguing question we might ask here: **can we get every prime via this method?**

This happens to be extremely difficult to prove, and **no one knows the answer**. However, a probabilistic argument states that indeed we are almost certainly getting every prime within this sequence:

- Take a prime $q$. What is the probability that our generated $n$ is divisible by $q$?
- The answer is: $1/q$. To understand why, pick $q=2$: you are essentially asking what is the probability that $n$ is even. More precisely, in every $q$ integers, one of them will be divisible by $q$.
- The probability that $q$ does not divide $n$ is therefore $1-1/q$.
- Since we are doing this iteratively, we can ask: what is the probability that $q$ does not divide any of the numbers seen until step $t$? That would be $(1 - 1/q)^t$.

As you take a larger $t$, this approaches 0; meaning that eventually there _should_ be a number that is divisible by $q$. This is not a proof though, it is just an argument! We have no idea how to prove this for all primes.

If you would like to compute this sequence yourself, here is my Python implementation:

```py
def euclid_mullin(cnt: int) -> list[int]:
    from functools import reduce

    def find_smallest_prime_factor(n: int) -> int:
        # try all odd numbers step until you reach sqrt(n)
        i = 3
        while i * i <= n:
            # if i divides n, return that as the smallest prime factor
            if n % i == 0:
                return i
            i += 2
        # otherwise, n itself is a prime
        return n

    # initial prime set is just [2]
    primes: list[int] = [2]
    while len(primes) < cnt:
        # generate the new number. note that this number is always odd
        n: int = reduce(lambda x, y: x * y, primes) + 1
        # find the smallest prime factor of this number
        q: int = find_smallest_prime_factor(n)
        # add it to the list of primes
        primes.append(q)
    return primes


print("Primes:", euclid_mullin(10))
```
