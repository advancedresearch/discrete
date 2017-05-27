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

### How to use discrete spaces in problem solving

Imagine 4 people living in 3 different houses. How many combinations are there,
and can you list all of them?

This kind of problems occur frequently in the real world.
One common property is that they contain lots of unknown variables and uncertainties.
Our human brains are poorly adapted to think of many possibilities at once,
but by using computers we can sometimes use brute force.

Solution:
```
4^3 = 64

// Each digit position represent a person and the value is where the person lives.
0000
0001
0002
0010
0011
0012
0020
...
3333
```

This discrete space of this kind can be constructed by the type `DimensionN`.
There are 4 dimensions, one for each people, which all has a size of 3.

Now, consider another problem:

Imagine 4 couples living in 3 different houses. One house contains maximum 2 couples but no house is empty.
How many combinations are there, and can you list all of them?

Notice the similarity to the first problem, where people are replaced by couples and
a constraint is added that renders some combinations invalid.

One approach is to use the same algorithm as in the first problem and filter out
all solutions that does not satisfy the constraints.
Each digit represents a couple instead of a person.

Another approach is to use 8 people with the same algorithm,
and pick only solutions where a house contains two or four people.

Both solutions are valid answers, but they answer different questions.
The first approach ignores the arrangement of individuals, while the second approach ignores the arrangement of couples.
How we understand something can depend on which algorithm we use,
but in an informal settings this can be ambiguous.

A discrete space is a non-ambiguous way to represent all possibilities from the topology and dimension of the solution space.
Each state in the space corresponds to a single location or a sub-structure of a larger problem.
In other words, natural numbers behave as placeholders for something like generic types in programming.

The structure of the discrete space do not automatically give you the answer,
but it makes it easier to examine a problem from every perspective once you know how to define it.

One benefit with this approach is that you can start with a low dimension to make sure you understand the problem,
and then expand to the real size of the space of possibilities afterwards.
When a mathematical formula exists for e.g. counting possibilites,
discrete spaces are used to test the first few numbers in the formula.

Sometimes a large solution space contains symmetries such that it can be contracted to a smaller space.
This can help improve the performance of search and analysis.
When solving problems you are often not aware of these symmetries at first,
but you can start with a general space and then add assumptions of symmetry to make the space smaller.
A technique often used is to split a problem into symmetric parts and asymmetric parts,
such that more efficient algorithms can be used on the simpler cases.
