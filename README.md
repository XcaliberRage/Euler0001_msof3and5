# Euler0001_msof3and5

Solution to https://projecteuler.net/problem=1

If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
Find the sum of all the multiples of 3 or 5 below 1000.

My first approach was the complex method which eventually got me the correct answer.

I decided to compare the speed of that to the simple approach of just checking every number from 1-1000, turns out the complex approach is ~30 times quicker, but does use a 1000 int array so 1000 bytes more on the heap!
I enjoyed exploring both options as sometimes speed counts, sometimes space counts!
