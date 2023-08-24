# Ponder This

My attempts to solve IBM's [ponder this](https://research.ibm.com/haifa/ponderthis/index.shtml) challenge.

# Challenges
## Aug 2023 (✔️)

### Counting Square-Triangular numbers below 10^100
- Found `BigUint` in rust to allow usage of numbers larger than $2^64 - 1$
- Computed first ~10 square-triangular numbers naively, using a "ping-pong" technique between `n` and `m`
- From these identified numbers, was able to recognize a pattern and compute beyond in a much more efficient manner using DP.

