# Advent of Code: Day-01

## Problem 1
- we have 2 lists, equal lengths.
- we need to sort both lists in ascending, and find the absolute difference between corresponding elements.
- sum all the differences.

## Solution Steps
- create 2 vectors.
- read the input file.
- split the input by new lines, for each line, split the list items by whitespace.
- trim the whitespace and convert the items to integers.
- add to the corresponding vectors.
- sort both vectors.
- subtract one vector from another.
- sum the absolute differences.
