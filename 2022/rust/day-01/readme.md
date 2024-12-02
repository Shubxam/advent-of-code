## Day-01

Problem Statement:
- Given a file with integers of each line.
- each integer value represents the caloric value of each food item.
- foods for elves are seperated by the blankLine.

Solution Steps:
- parse the file into string.
- add the consecutive integer values.
- when encountered a space, store the last value in a vector and restart from 0.
- Find the max value in the vector.
