## Problem

So, we need to return a sorted list merging two given ones.
The main problem is Rust and it's ownership policy.

## Solution

My solution is pretty conventional and similar with the merge sort algorithm
itself. In each iteration it compares heads of two links and takes the node with
less value. The head of that list shifts. And so on until both lists get empty.

## Complexity

Time complexity is O(n).

Why?  
The number of iteration either less than or equal to the sum length of two
lists.  
Why less?  
Because if the head of one of the lists is null, while the length of another
list is greater than 1, then we can simply take it's head and the rest follows.
Return.

## Reflection

What could be improved? Well, maybe some Rust native features might be used
which I'm aware of. But at the point of algorithm itself, I don't really think
it can take less than O(n).
