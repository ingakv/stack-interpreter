# prog2006-assignment-2

* Submission Deadline: **27.04.2023 - 23:59**
* Peer-review Deadline: **01.05.2023 - 23:59**
* Submission must be done through [the submission system](https://10.212.175.82)
   - https://10.212.175.82
   - Hash: `cg49jme8eh9dbc0db0sg`
* Programming language: Rust or Haskell 



# **bprog** :: simple concatenative, stack-based interpreter

**Note** - the specification below might be unclear or may need amendments. Use Issue tracker to post questions, clarification requests, or modification suggestions.

* [Assignment walkthrough video](https://youtu.be/Dw0dWN3yehM)
* Check Lecture from February 27, 2023.
* Check `haskell/rpn-calc` project for basics for parsing and using stack to represent your program state.
* check `haskell/fib-state` project for basics of how to incoroporate State monad into your program.

## Important

* **DO NOT USE** `parsec` library. All programs **must** be parsed simply by `words`.
* Report in your specification document all the design decisions and assumptions.
* Precisely report in your self-assessemnt what features are implemented and tested, and what you have not implemented and why.


## Overview

We will implement a simple concatenative, stack-based, programming language interpreter. The interpreter is called `bprog`. `bprog` will accept instruction from standard input and execute them following the semantics of the language that we defined below. It can be used as interactive CLI, or, it can be fed with a file containing a program and it will execute it.  

In the CLI mode, the program should parse and interpret the input line by line, in an infinite loop. In the `input file` mode it should work with the entire input. We will discuss those two modes below.

You can implement the language in Haskell (recommended) or Rust, or any other programming language that you chose. Do as many features of the language as you need for the example applications below, eg. `fizzbuzz`, `factorial` or `guess the number`. Describe your choices in the readme file.  The system should have built-in standard functions described in the specification. 
In addition, it can have "standard library" functions, defined in the language itself. Those are read and defined before the user program gets executed. This should be specified in `prelude.bprog` file, that is read automatically by default by the interpreter. You can place there convenience functions that are useful for a given task.



# bprog

The `bprog` program is represented as a sequence of arguments and operations. This sequence from text needs to be `parsed` and converted into an internal representation that can subsequently be used and manipulated by the interpreter. The interpreter needs two things - the program, and the current version of the operand stack (where the arguments to operations are stored). To make it simple, 

## Tokeniser (Lexer)

Instead of implementing a proper lexer, we will simply use the Haskell command `words`, and use space as a delimiter for all tokens in the language.

To make the task easy, we will use `words` to split the input into tokens. Normally, we write strings like this: `"This is a string"`, lists like that: `[1,2,3]` and tuples like: `(1,"text)`. However, that would require more complex lexer rules. Instead, we will simply use space as a delimiter, so thus we will have:
* symbols: `a_symbol` Note: because there are no restrictions on symbols, anything that is not a reserved keyword in the language can become a valid symbol, and therefore, a function name.
* strings: `" this is a string "`
* lists: `[ 1 2 3 ]`
* quotations (aka code blocks: `{ 1 2 + }`

Note, that the pretty printing of your internal representations can, and should use compact notation instead. So even though the bprog list is `[ 1 2 3 ]` you should represent it for debugging purposes as `[1,2,3]`. This way your pretty-printing will be consistent with the tests (see below).

## Parser

After all the program text is split into tokens, you should convert it into an internal representation, known as Abstract Syntax Tree (AST).
In our case, for mostly postfix language, you may not need to use a tree, but a simple list representing all the program elements.
It is up to you to decide the internal representation of your program.  Pass all the tokens from the previous step, 
and convert them into your internal representation. 
The representation matters to the next step, which is the Interpreter.

## Interpreter

The interpreter will take the internal representation of the parsed program and execute it. 

There are two modes for the program to operate: REPL mode, in which the `bprog` interprets code line by line and prints the current state of the operand stack; and NORMAL mode, in which the `bprog` takes entire program and interprets the entire thing, expecting to obtain a SINGLE value on top of the stack, that is subsequently printed out to the user.



## The stack

All arguments to functions are stored and taken from the stack, and all results of functions are put on top of the stack. The instructions can take arbitrary number of arguments from the stack, and produce and arbitrary number of values that are put on the stack. To describe the instructions we use a notation called "stack effects", that looks like that `( left -- right )` where `left` are the variables that are taken from the stack (first popped value on the right-hand side), and `right` depicts what is put onto the stack (from left to right, meaning, the right-most element will be on top of the stack). E.g. to describe addition `+` we write `( x y -- z)` where `x` and `y` are popped from the stack, and `z` is put back, on top.

Note: the stack effects do not mean anything, they are just human readable explanations.


## Stack operations

* `dup` ( x -- x x ) duplicates the top element on the stack
* `swap` ( x y -- y x ) swaps the two top elements on the stack
* `pop` ( x -- ) removes the top element from the stack


# Simple IO

We limit our language to TEXT only. All read/write operation operate on `String` types.

* `print` ( x -- ) takes the top element from the stack and prints it to the standard output.
* `read` ( -- x ) reads a line from standard input and puts it into the stack as string.


# String parsing

* `parseInteger` ( s -- i ) takes a string from stack and converts it to Integer and puts it onto the stack
* `parseFloat` ( s -- f ) same as above but for floats
* `words` ( s -- list ) takes a string from the stack, splits is with Haskell `words` or Rust `split_whitespace` command and puts a list of tokens onto the stack.


## Literals

All literals are simply pushed onto the stack.

* Integers, eg. `0` `1` `200`
* Floats, eg. `1.0` `200.0`
* Bools: `True` `False`
* Strings, delimited by double quotes `"` eg. " hello world "
* Lists, delimited by square brackets `[ ]`


## Arithmetic

Reuse the code from the calculator. Each operation has the number of operands (arguments) it takes from the stack provided in the round brackets. E.g. addition `+` takes two arguments, so it is written `+` (2). The operations are written in Reverse Polish notation (postfix notation), where the operation is at the end, and the operands are in front.

* `+` ( x y -- x_plus_y ) - addition
* `-` ( x y -- x_minus_y ) - subtraction
* `*` ( x y -- mul ) - multiplication
* `/` ( x y -- fdivision ) - floating point division
* `div` ( x y -- integer_division ) - integer division
* `<` ( x y -- bool) checks if `x < y`, and puts true or false on the stack
* `>` ( x y -- bool) checks if `x > y`, and puts true or false on the stack
* `==` ( x y -- bool ) checks if `x == y` and puts true or false on the stack




## Logical operations

* `True` - literal
* `False` - literal 
* `&&` ( x y -- bool ) - logical AND
* `||` ( x y -- bool ) - logical OR
* `not` ( x -- bool ) - logical NOT. I've implemented it such that it also works like a negation on numbers, so, if you call it: `10 not` the program will put `-10` on top of the stack.



## `Code block`, called `block`, or `quotation`

Code block (aka just `block` or `quotation`) is a program sequence delimited by curly braces. For example `{ 1 + }` is a quotation that increments the current top element on the stack by 1.
There is a function `exec` that picks a quotation from the top of the stack and executes it.

**Note**: in some of the functions quotation is expected, but, for single instruction quotation the curly braces can be skipped.  So, for example `3 times { 10 }` is the same as `3 times 10` because the quotation contains ONLY one instruction. The notation without curly braces for single instruction quotations is more ergonomic. `times`, `map`, `foldl`, `each`, `if` should all work with both, quotations for multiple instructions and for single values (no curly braces needed). 


## Lists 

Lists are delimited by square brackets. Lists can be nested. List can hold arbitrary types of arguments. Example of valid lists in `bprog`:

```
[ 1 2 3 ]
[ " hello " " world " ]
[ 1 " hello " 2 " world " [ False True ] hello_symbol ]
```


### List operations

* `head` ( list -- item ) takes a list and returns its head
* `tail` ( list -- tail ) takes a list and returns the tail
* `empty` ( list -- bool ) takes a list and returns true if the list is empty 
* `length` ( list -- len ) puts the length of a given list onto the stack
* `cons` ( item list -- list ) appends the item in front of the list
* `append` ( list1 list2 -- list3 ) concatenates both lists
* `each quotation` ( list -- ) takes a list an a code block, and executes the code block on each of the elements of the list, eg. `[ 1 2 3 ] each { println }` will print three lines with 1, 2, 3 respectively in each of the lines.
* `map quotation` ( list -- newlist ) takes a list, and a block, and executes the block on each of the elements of the list, forming a new list that is put on the stack. E.g. `[ 1 2 3 ] map { 10 * }` will result in a list `[ 10 20 30 ]`
* `foldl quotation` ( list initial_accumulator -- final_accumulator ) folds the list from left to right.  E.g. `[ 1 2 3 ] 0 foldl { + }` will result in `6` on top of the stack.


## Control flow

Control flow operations operate both, on the argument stack as well as on the program itself. For example, the boolean argument to the if-statement is taken from the stack, but, the `then_block` and `else_block` are taken from the program itself.

* `if then_block else_block` ( bool -- ) `if` expression takes a boolean value from the stack, and executes the `then_code_block` if true, or `else_code_block` if false. The executed block operates in the context of the global stack.
* `loop break block` ( -- ) execute the block until `break` becomes True. `break` and `block` are expected to be quotations. `break` evaluating to True or False does not leave that value on the stack (it is consumed by the `loop`)
* `times block` ( num -- ) repeat the block `num` times


### Examples

```
3 10 > if 
{ " 3 is MORE than 10 " print } 
{ " 3 is LESS then 10 " print }
```

```
5 5 ==
if
{ " hey! five is five " println }
{ }
```

Note, that the condition must be on the operand stack BEFORE `if` is called. Both blocks for `if` statement are needed, the THEN block and the ELSE block, but, one of the (or both) can be empty.  The code blocks are curly brace delimited.

Everything in `bprog` is white-space delimited, so you need white space between all the symbols, and white space between " and the string. 

Note also that this is also valid code (white space is needed, indentation is not needed):

```
{
  if
  { " there was True on the stack " print }
  { " there was False on the stack " print }
}
```

This is a code block, that you can assign a name, and use in your program later on. This code block says nothing about the argument stack, so, it can be applied in various contexts, or, assigned to a variable/function name, eg.:

```
{
  if
  { " there was True on the stack " print }
  { " there was False on the stack " print }
}
check_stack_and_print
swap
fun
```

Note, that assignment to variable expects the name to be deeper on the stack, and the value to be on top of the stack, this is why we had to do `swap` before `:=` or `fun`.



## Assignment to a symbol (variable)

There are two constructs: assignment `:=` and function definition `fun`.

Assignment `:=` takes two arguments, left hand side must be a `symbol` (aka `variable`), and right hand side can be any value different from a symbol, eg. number, bool, list or code_block.

Function definition `fun` takes two arguments, left hand side must be a `symbol` (aka `function name`), and the right hand side must be quotation (code block).

With the function definition, one can define a named code block (aka function). For example: 
* `inc { 1 + } fun` defines a new function called `inc` that increments the element on the stack.
* `sayhello { " hello " write } fun` defines a function `sayhello` that print `hello`.
* `name " Mariusz " :=` defines a symbol `name` (aka variable) that is of value `" Mariusz "`
* `age 10 :=` the symbol age now is of value `10`
* the above program can be also written as: `10 age swap :=`

Once a symbol is bound to a function or variable, using that symbol will either run the function or evaluate the variable to a value.
For example:
* `inc { 1 + } fun 1 inc` will execute the `inc` function on the argument on the stack, and produce 2 onto the stack.
* `age 10 := age` will bind age variable to value 10, and then put 10 on top of the stack (because we have used the variable `age` at the end)

In order to actually put a bound symbol onto the stack (without executing the function or without evaluating to a value in case of variable), we use `tick` operator, which is a single quote symbol `'`. Observe these two programs:
* `age 10 := age` -- produces 10 on top of the stack
* `age 10 := ' age` -- puts symbol age on top of the stack. The symbol `age` represents a variable, but the variable has not been evaluated to a value and the raw symbol is put onto the stack.  
* `age 10 := age 20 :=` -- this program is illegal, because assignment expects a symbol on the left-hand side, but instead, it gets 10.
* `age 10 := ' age 20 :=` -- this program defines a variable `age` and binds it to 10 first, then puts symbol `age` onto the stack, and re-binds it to value 20.
* Note, if the symbol has not been bound to function or variable, using it will put it onto the stack raw. Because unbound symbol evaluates to itself.

In order to evaluate bound symbol to a value, one can use `eval` function. Eval expects symbol on the stack, and returns the value of that symbol. In the case of variable it will be the value, and in the case of the function name it will be the quotation that is the function body. Observe:
* `age 10 := ' age eval` will bind age to value 10, then it will put age onto the stack as symbol, and then evaluate it to value 10. So, this program will end up with 10 on top of the stack.



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

The first line is composed of the symbol `odd` that is put onto the stack. Because the symbol `odd` is not bound to anything, it is treated as if it is preceeded with a tick. So the first line is exactly the same as if it was written:

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

1. It should be illegal. `each` and `map` should ONLY take a unary funcion as argument. 
2. It should be legal, and the missing arguments to those functions should be taken from the operand stack, each time the function is run.
3. It should be illegal for `map`. For `each f`, the call to the function `f` should be done for each of the list elements, with the current stack, such that the function can consume the missing elements from the stack, and put the partial results back to the stack. For example: `0 [ 1 2 3 ] each +` will execute as follows:
   - 0 and list goes onto the stack
   - each is executed with a function `+` on the element 1, and, the missing element is taken from the stack, in which case it is 0.
   - the result from the previous step is put onto the stack, and, the next item from the list is given to `+`. `2 +` is missing an element, and it is taken from the stack again, and `2 + 1`, which results to 3 is put back onto the stack.
   - the final list element, `3` is given to function `+`. Because again it is missing an operand and the current operand on the stack is 3, we end up with `3+3` wich results in final `6` put onto the stack.




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
