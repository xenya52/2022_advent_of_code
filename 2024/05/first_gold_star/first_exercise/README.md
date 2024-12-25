# The idea
I'll separate the problem into multible parts:
1. Read file input
2. Create a function to get the rules out of the file
3. Create a function to get the "updates" from the file
4. Iterrate over the updates and only save the rules that are valid
5. Create a function to get the sum between each valid update and use it into the function from step four
6. Create a function that will return the sum of all the valid updates and use it into the function from step four
= **The sum of all valid updates**

# The rules
l | r
1. l must be printed before r
2. So if i have 42|54 42|65 42|76 than the chain is valid if every number stands behind 42
