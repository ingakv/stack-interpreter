# **bprog** :: simple concatenative, stack-based interpreter

* [Assignment walkthrough video](https://youtu.be/Dw0dWN3yehM)
* Check `haskell/rpn-calc` project for basics for parsing and using stack to represent your program state.
* check `haskell/fib-state` project for basics of how to incorporate State monad into your program.

## Important

* **DO NOT USE** `parsec` library. All programs **must** be parsed simply by `words`.
* Report in your specification document all the design decisions and assumptions.
* Precisely report in your self-assessment what features are implemented and tested, and what you have not implemented and why.


## Overview

We will implement a simple concatenative, stack-based, programming language interpreter. The interpreter is called `bprog`. `bprog` will accept instruction from standard input and execute them following the semantics of the language that we defined below. It can be used as interactive CLI, or, it can be fed with a file containing a program and it will execute it.  

In the CLI mode, the program should parse and interpret the input line by line, in an infinite loop. In the `input file` mode it should work with the entire input. We will discuss those two modes below.

You can implement the language in Haskell (recommended) or Rust, or any other programming language that you chose. Do as many features of the language as you need for the example applications below, eg. `fizzbuzz`, `factorial` or `guess the number`. Describe your choices in the readme file.  The system should have built-in standard functions described in the specification. 
In addition, it can have "standard library" functions, defined in the language itself. Those are read and defined before the user program gets executed. This should be specified in `prelude.bprog` file, that is read automatically by default by the interpreter. You can place there convenience functions that are useful for a given task.



# bprog

The `bprog` program is represented as a sequence of arguments and operations. This sequence from text needs to be `parsed` and converted into an internal representation that can subsequently be used and manipulated by the interpreter. The interpreter needs two things - the program, and the current version of the operand stack (where the arguments to operations are stored). To make it simple, 

## 

## Parser

After all the program text is split into tokens, you should convert it into an internal representation, known as Abstract Syntax Tree (AST).
In our case, for mostly postfix language, you may not need to use a tree, but a simple list representing all the program elements.







## Symbols dictionary and the operand stack

To interpret the functions and variables you need to be able to recognised all already defined symbols. For that, you will use a dictionary that maps symbols to specific values. You will also use stack. There is one global stack that is initially empty. The code blocks are executed always in the context of the global stack and a global dictionary. We use very primitive scoping rules with everything in a single global context. Remember, that:
* symbols can be re-bound to new values (we do not keep track of types and symbols are mutable!)
* unknown symbol evaluates to itself, whereas bound symbols evaluate to what they are bound. 
For example: 
* `age print` prints `age` (a symbol)
* `age 10 := age print` prints `10` (a value to which symbol age is now bound)
* `counter { " hello " print } times` will crash, as the times expects an integer as the first argument, and instead, it got a symbol (that evaluates to itself, which is, a symbol)
* `counter 10 := counter { " Hello World " print } times` is a valid program and it will print `Hello World` string 10 times.





# Error handling

Note: trying to pop an item from an empty stack should result in the program panic (crash). 

For this assignment the error handling is left unspecified, however, you should try to think how to provide meaningful messages back to the user, from both, the program parsing stage, as well as from the program execution. 

The interpreter should stop on error and you should try to provide a meaningful error message to the user.

Example types of errors you might consider. Note: this is NOT an exhaustive list, and, you can parametrise the errors with context string to give the user context on WHERE exactly the error occurred.

```
-- | Represents program execution errors.
data ProgramError =
     StackEmpty
   | UnknownSymbol
   | ExpectedBool
   | ExpectedBoolOrNumber
   | ExpectedEnumerable
   | ExpectedQuotation
   | ExpectedList
   | ExpectedVariable
   | DivisionByZero
   | ProgramFinishedWithMultipleValues
   | NumberConversionError
     deriving (Eq, Show)

-- | Represents parser errors.
data ParserError =
    IncompleteString
  | IncompleteList
  | IncompleteQuotation
```


## Program execution

The program in `bprog` are expected to produce a SINGLE value on top of the value stack. If the program terminates with zero values on the stack, or with multiple values on the stack, this is considered an Error, and should be reported back to the user.  The single value produced by the program should ALWAYS be printed back to the user (even if the program does not have any IO operations). 


### Minimal subset (D)

* ability to handle integers and integer arithmetic (from calculator example)
* ability to handle strings
* parsing strings: `parseInteger`, `parseFloat`
* bools: `&&`, `||`
* stack: `swap`, `dup`, `pop`
* list: `head`, `tail`, `empty`, `length`, `cons`, `append`


### Minimal subset for ambitious minimalists (C)

Same as the minimal subset above plus:
* quotations: `exec`
* control flow: `if`, `times`
* control flow with lists: `map`, `foldl`, `each`


### IO

In Haskell, IO makes it impossible for automating the tests, therefore, IO is optional. For people that want to go for B and A, you must have automated tests for EVERYTHING except `print` and `read` and for those three commands, you need to keep them separated from the main execution and interpretation contexts, such that you can have automated tests for everything else. 

In Rust, mixing IO with core interpreter logic is easier, so it is not a big deal.



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




# Tests

Below is a set of tests demonstrating `bprog` programs and the expected output of the interpreter. 

This is a copy and paste from my own test implementation. `t` is an utility function that is implemented with `it` for `Hspec` testing framework such that I do not have to repeat the boilerplate code.  It takes the string of a program and expected output of the interpreter, and checks if there was no error and if the output is as expected.  The tests do not test "everything" yet, I've update it shortly. 

Please implement the same official TESTS such that you can list in your submission which tests you pass and which do not pass. 

```
officialTests =
  describe "official tests" $ do
    describe "literals" $ do
        t "3"                           "3"
        t "121231324135634563456363567" "121231324135634563456363567"
        t "1.0"                         "1.0"
        t "0.0"                         "0.0"
        t "-1"                          "-1"
        t "-1.1"                        "-1.1"
        t "False"                       "False"
        t "True"                        "True"
        t "[ [ ] [ ] ]"                 "[[],[]]"
        t "[ False [ ] True [ 1 2 ] ]"  "[False,[],True,[1,2]]"
        t "\" [ so { not if ] and } \"" "\"[ so { not if ] and }\""

    describe "quotation literals" $ do
        t "{ 20 10 + }"             "{ 20 10 + }"
        t "{ { print } exec }"      "{ { print } exec }"
        t "[ { + } { 10 + } { 20 10 + } ]"   "[{ + },{ 10 + },{ 20 10 + }]"

    describe "simple arithmetic" $ do
        t "1 1 +"               "2"
        t "10 20 *"             "200"
        t "20 2 div"            "10"
        t "20 2 /"              "10.0"

    describe "arithmetic with type coercion" $ do
        t "1 1.0 +"             "2.0"
        t "10 20.0 *"           "200.0"
        t "20 2.0 div"          "10"
        t "20.0 2.0 div"        "10"
        t "True 0 + False 0 + =="   "False" -- optional check if True and False are coerced differently

    describe "bool operations" $ do
        t "False False &&"      "False"
        t "False True ||"       "True"
        t "False not"           "True"
        t "True not"            "False"

    describe "comparisons" $ do
        t "20 10 <"             "False"
        t "20 10 >"             "True"
        t "20 10 >="            "True"
        t "10 20 >="            "False"
        t "10 10 >="            "True"
        t "20 10.0 >"           "True"
        t "20 10.0 >="          "True"
        t "10 10.0 >="          "True"
        t "20.0 20.0 >"         "False"
        t "10 10 =="            "True"
        t "10 10.0 =="          "True"
        t "True True =="        "True"
        t "True 40 40 == =="    "True"
        t "\" abba \" \" abba \" ==" "True"
        t "[ ] [ ] =="          "True"
        t "[ 1 2 ] [ 1 2 ] =="  "True"
        t " [ [ ] ] [ [ ] ] ==" "True"

    describe "stack operations" $ do
        t "10 20 swap pop"          "20"
        t "10 dup dup + swap pop"   "20"
        t "10 20 swap dup + div"    "1"

    describe "length" $ do
        t "\" hello \" length"              "5"
        t "\" hello world \" length"        "11"
        t "[ 1 2 3 [ ] ] length"            "4"
        t "{ 10 20 + } length"              "3"

    describe "String parsing" $ do
        t "\" 12 \" parseInteger"           "12"
        t "\" 12.34 \" parseFloat"          "12.34"
        t "\" adam bob charlie \" words"    "[\"adam\",\"bob\",\"charlie\"]"

    describe "lists" $ do
        t "[ 1 2 3 ]"           "[1,2,3]"
        t "[ 1 \" bob \" ]"     "[1,\"bob\"]"
        t "[ 1 2 ] empty"       "False"
        t "[ ] empty"           "True"
        t "[ 1 2 3 ] head"      "1"
        t "[ 1 2 3 ] length"    "3"
        t "[ 1 2 3 ] tail"      "[2,3]"
        t "1 [ ] cons"          "[1]"
        t "1 [ 2 3 ] cons"      "[1,2,3]"
        t "[ 1 ] [ 2 3 ] append" "[1,2,3]"
        t "[ 1 2 ] [ ] append"  "[1,2]"
        t "[ 1 ] [ 2 3 ] cons"  "[[1],2,3]"

    describe "list quotations" $ do
        t "[ 1 2 3 ] map { 10 * }"                              "[10,20,30]"
        t "[ 1 2 3 ] map { 1 + }"                               "[2,3,4]"
        t "[ 1 2 3 4 ] map { dup 2 > if { 10 * } { 2 * } }"     "[2,4,30,40]"
        t "[ 1 2 3 ] each { 10 * } [ ] cons cons cons"          "[10,20,30]"
        t "[ 1 2 3 4 ] each { 10 * } + + +"                     "100"
        t "10 [ 1 2 3 ] each { + }"                             "16"
        t "10 [ 1 2 3 ] each + "                                "16"
        t "[ 1 2 3 4 ] 0 foldl { + }"                           "10"
        t "[ 1 2 3 4 ] 0 foldl +"                               "10"
        t "[ 2 5 ] 20 foldl { div }"                            "2"

        {-- note no { } needed for 1 instruction code -}
        t "[ \" 1 \" \" 2 \" \" 3 \" ] each { parseInteger } [ ] cons cons cons" "[1,2,3]"
        t "[ \" 1 \" \" 2 \" \" 3 \" ] each parseInteger [ ] 3 times cons"       "[1,2,3]"
        t "[ 1 2 3 4 ] 0 foldl +"                               "10"
        t "[ 2 5 ] 20 foldl div"                                "2"

    describe "assignments" $ do
        t "age"                             "age"
        t "age 10 := age"                   "10"
        t "10 age swap := age"              "10"
        t "[ 1 2 3 ] list swap := list"     "[1,2,3]"
        t "age 20 := [ 10 age ]"            "[10,20]"
        t "' age"                           "age"
        t "age 10 := ' age 20 := age"       "20"
        t "age 10 := ' age eval"            "10"

        t "inc { 1 + } fun 1 inc"           "2"
        t "mul10 { 10 * } fun inc { 1 + } fun 10 inc mul10" "110"

    describe "quotations" $ do
        t "{ 20 10 + } exec"                "30"
        t "10 { 20 + } exec"                "30"
        t "10 20 { + } exec"                "30"
        t "{ { 10 20 + } exec } exec"       "30"
        t "{ { 10 20 + } exec 20 + } exec"  "50"

    describe "if with quotation blocks" $ do
        t "True if { 20 } { }"               "20"
        t "True if { 20 10 + } { 3 }"        "30"
        t "10 5 5 == if { 10 + } { 100 + }"  "20"
        t "False if { } { 45 }"              "45"
        t "True if { False if { 50 } { 100 } } { 30 }" "100"

    describe "if without quotation, more ergonomic expressions" $ do
        t "True if 20 { }"                 "20"
        t "True if { 20 10 + } 3"          "30"
        t "10 10 5 5 == if + { 100 + }"    "20"
        t "False if { } 45"                "45"
        t "True if { False if 50 100 } 30" "100"

    describe "times" $ do
        t "1 times { 100 50 + }"                               "150"
        t "5 times { 1 } [ ] 5 times { cons } 0 foldl { + }"   "5"
        t "5 times 1     [ ] 5 times   cons   0 foldl   +  "   "5"
        t "5 times { 10 } + + + +"                             "50"
        t "5 times 10 4 times +"                               "50"

    describe "loop" $ do
        t "1 loop { dup 4 > } { dup 1 + } [ ] 5 times { cons }"         "[1,2,3,4,5]"
        t "1 loop { dup 4 > } { dup 1 + } [ ] 5 times   cons  "         "[1,2,3,4,5]"
        t "[ 1 ] loop { dup length 9 > }  { dup head 1 + swap cons }"   "[10,9,8,7,6,5,4,3,2,1]"

        t "odd { dup 2 div swap 2 / == if False True } fun \
        \  2 odd"                                                       "False"

        t "odd { dup 2 div swap 2 / == if False True } fun \
        \ 3 odd"                                                        "True"

        t "toList { [ ] swap times cons } fun \
        \ 1 2 3 4 \
        \4 toList"                                                      "[1,2,3,4]"

        t "gen1toNum { ' max swap := 1 loop { dup max > } { dup 1 + } } fun \
        \ 3 gen1toNum + + +"                                            "10"
        
        t "gen1toNum { ' max swap := 1 loop { dup max >= } { dup 1 + } } fun \
        \ 3 gen1toNum + +"                                              "6"
        
        t "odd { dup 2 div swap 2 / == if False True } fun \
         \ toList { [ ] swap times cons } fun \
         \ gen1toNum { ' max swap := 1 loop { dup max > } { dup 1 + } } fun \
         \ 4 gen1toNum 5 toList map odd"                                "[True,False,True,False,True]"

    describe "extra programs" $ do
        t "drop { times tail } fun \
        \  [ 1 2 3 4 5 ] 3 drop"         "[4,5]"

```
