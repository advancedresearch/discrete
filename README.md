# discrete
Combinatorial phantom types for discrete mathematics

This library is for constructing algorithms by composition that maps to and from natural numbers.

For example, a pair is a tuple `(a, b)` where `b > a`.
It can represent an undirected edge between two nodes in a graph.

A pair can be mapped to and from a natural number.
This can be used to store and retrieve information associated with the edge.

In this library, a pair is represented as the type `Pair`.
When you need all pair of pairs, you write `Pair<Of<Pair>>`.

### Why?

Discrete spaces that maps to and from natural numbers have many nice mathematical properties.
For example, it can enumerate all possible structures by listing the natural numbers up to a limit.
Two structures are equal if they map to the same natural number.

In principle you could just use numbers, but it would be very hard to write the correct algorithm.
This library gives you the correct algorithms from the type composition.
