## Problem

So, we need to determine wether a given list is cycled or not, given that the
node of cycle entry may be any node and the length of list is not known if it's
cycled.

## Approach

Well, there are three solutions, I came up with (ok, the last one is borrowed):

### Solution 1: Luck

This solution approach is based on the assumption that the way the list is
allocated in leetcode includes that the address of each next node is greater in
the heap. So basically, the algorithm just iterates throw nodes and checks
whether the next node's address is lower. If it is, that means (according the
assumption) that the next node is the entry point of the cycle.

Well, the assumption was write, I won, yay

### Solution 2: Log

This solution approach simply logs every node address and iterates throw them.
In every iteration it check whether the address of the current node exists in
the log. If so => true. Met null pointer => false.
Well, this thing is the worst one I've ever programmed and it's the total
outsider by all params.

### Solution 3: Slow-Fast

Well, this one requires two pointers. First one iterated incrementing by 1. The
second one increments by 2. If the list is cycled, faster pointer meets the
slower sooner or later. If not, well, the faster pointer simply meets null
pointer.

## Time Complexity

Time Complexity: O(n)

Why O(n)?
1. The first one takes exactly n iterations, cuz the cycle entry is found just
   after the last node.
2. The second is something like O(2n). If iteration takes n, log search takes
   2(n+1)/n. So basically it's n.
3. The thirds one takes something like O(n), but I cannot calculate it exactly.

## Reflection

Well, I don't actually have anything to say.
