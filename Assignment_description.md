# **bprog** :: simple concatenative, stack-based interpreter

* [Assignment walkthrough video](https://youtu.be/Dw0dWN3yehM)
* Check `haskell/rpn-calc` project for basics for parsing and using stack to represent your program state.
* check `haskell/fib-state` project for basics of how to incorporate State monad into your program.




## Overview

We will implement a simple concatenative, stack-based, programming language interpreter. The interpreter is called `bprog`. `bprog` will accept instruction from standard input and execute them following the semantics of the language that we defined below. It can be used as interactive CLI, or, it can be fed with a file containing a program and it will execute it.  

In the CLI mode, the program should parse and interpret the input line by line, in an infinite loop. In the `input file` mode it should work with the entire input. We will discuss those two modes below.



## Symbols dictionary and the operand stack

To interpret the functions and variables you need to be able to recognized all already defined symbols. For that, you will use a dictionary that maps symbols to specific values. You will also use stack. There is one global stack that is initially empty. The code blocks are executed always in the context of the global stack and a global dictionary. We use very primitive scoping rules with everything in a single global context. Remember, that:
* symbols can be re-bound to new values (we do not keep track of types and symbols are mutable!)
* unknown symbol evaluates to itself, whereas bound symbols evaluate to what they are bound. 
For example: 
* `age print` prints `age` (a symbol)
* `age 10 := age print` prints `10` (a value to which symbol age is now bound)
* `counter { " hello " print } times` will crash, as the times expects an integer as the first argument, and instead, it got a symbol (that evaluates to itself, which is, a symbol)
* `counter 10 := counter { " Hello World " print } times` is a valid program and it will print `Hello World` string 10 times.

# Questions and Answers

## Program analysis


### Case one

```
' x 
10 
:=
x
' x
```

* Line 1: put `x` symbol on top of the stack
* Line 2: put `10` onto the stack
* Line 3: execute assignment operation.  Assignment operation is defined such that it expects on top of the stack (from top): 
   * a Value 
   * a symbol
* Line 4: `x` represents now a bound variable, and when using it, the interpreter will evaluate the variable to a value, and put the value on top of the stack. In this case, `10` will be put on top of the stack. 
* Line 5: put symbol `x` on top of the stack.

```
x 10 := x ' x
```

This program is EXACTLY the same as the first program above. This is because we have a rule in the language, which treats unbound symbols as if there were used with the tick operator: `'`.  Therefore, for any unbound symbols, there is a "syntactic sugar" that adds `'` to them.  The programmer does not have to do that. This simplifies the code, and only forces the programmer to use tick for bound symbols, and allows them to skip them when the symbol is not yet bound. the last tick is necessary, otherwise the `x` would be replaced with the variable value instead. This makes reading and writing some of the code easier.
* `x 10 :=` reads/writes more natural 
* `' x 10 :=` is exactly the same, but, is a bit more "tedious"

Is this a good design decision for our `bprog` language?



### Case two

```
' x 10 := x
' y { 10 } fun y
' z { 10 } := z
```

* Line 1: we define x as a variable, with value `10`, and then evaluate it and put `10` onto the stack.
* Line 2: we define new function `y`, assign a quotation `{ 10 }` to it, and subsequently we call the function `y`. This will put `10` on top of the stack.
* Line 3: we define a new variable `z`, and assign it to a quotation.  What will happen when we evaluate the variable that is bound to a quotation? Well, the value of the variable is the quotation. So, the quotation `{ 10 }` will be put onto the stack, when we evaluate `z` at the end of the line.

This example demonstrate the difference between using `fun` and using `:=`. It also demonstrate a difference between `evaluating` variables, and `executing` quotations.  To evaluate a variable we simply need to lookup a variable binding - what is the value a variable is bound to.  But, to execute a quotation, we need to give it the current operand stack, and RUN the quotation to calculate what the output is.




### Case three

```
odd { dup 2 div swap 2 / == if False True } fun
4 5 odd swap odd
```

The first line is composed of the symbol `odd` that is put onto the stack. Because the symbol `odd` is not bound to anything, it is treated as if it is proceeded with a tick. So the first line is exactly the same as if it was written:

```
' odd { dup 2 div swap 2 / == if False True } fun
```

Then, in the second line, we put 4 and 5 onto the stack, and CALL (RUN) our newly defined function twice. So, we will get `True` first, from running odd with 5. The stack will be `True 4` with True on top and 4 below. We swap the order to put 4 on top, and run `odd` again. This type the argument to `odd` will be 4 and the function will return `False`. So, after executing the program we will have `False` on top of the stack, and `True` below.

### Case four

```
plus { + } fun
```

This code defines new function called `plus`. In our language, when we do that, there is NO difference between `+` and `plus`. These two ARE exactly the same (from the programmer perspective).  So doing:
```
10 20 plus
10 20 +
```
is exactly the same. Note however, that this:
```
10 20 { + }
```
is not the same as the code above.  This code, puts three things onto the stack: on top we will have quotation, then we will have 20 and then 10 at the bottom.


### Case five

```
[ 1 2 3 ] map { 10 * }
```
This program produces a list `[ 10 20 30 ]` on the stack.


```
[ 1 2 3 ] each { 10 * }
```
This program produces values `30`, `20` and `10` on the stack (`30` on top).

How those programs should behave, when the function passed to `each` or `map` takes more than a single argument? 

1. It should be illegal. `each` and `map` should ONLY take a unary function as argument. 
2. It should be legal, and the missing arguments to those functions should be taken from the operand stack, each time the function is run.
3. It should be illegal for `map`. For `each f`, the call to the function `f` should be done for each of the list elements, with the current stack, such that the function can consume the missing elements from the stack, and put the partial results back to the stack. For example: `0 [ 1 2 3 ] each +` will execute as follows:
   - 0 and list goes onto the stack
   - each is executed with a function `+` on the element 1, and, the missing element is taken from the stack, in which case it is 0.
   - the result from the previous step is put onto the stack, and, the next item from the list is given to `+`. `2 +` is missing an element, and it is taken from the stack again, and `2 + 1`, which results to 3 is put back onto the stack.
   - the final list element, `3` is given to function `+`. Because again it is missing an operand and the current operand on the stack is 3, we end up with `3+3` which results in final `6` put onto the stack.


