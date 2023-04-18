# Notes

The program searches through the entire stack from the bottom up.

If an arithmetic operator has been added, it will loop through until it finds two numbers, even if there are other variables in between.
I.e. if the stack is `[ 1, "hello", 2, +]` the program will extract 1 and 2, and push the result to the back, resulting in the stack now being `["hello", 3]`
