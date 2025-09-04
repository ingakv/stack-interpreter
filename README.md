# BPROG - Stack Interpreter

This is a simple concatenative, stack-based, programming language interpreter that can do simple math, io, parsing, and simple code blocks, written in Rust.

## Usage

**To run normal mode:**
open terminal in project
`cargo run`

**To run REPL mode:**
`cargo run -- REPL`

**To run tests:**
`cargo test`



## Project Specifications

The program searches through the entire stack from the bottom up.

If an operator has been added, it will loop through until it finds two (or one, depending on the operator) variables of the correct type, even if there are other variables in between.
I.e., if the stack is `[1, "hello", 2, +]` the program will extract 1 and 2, and push the result to the back, resulting in the stack now being `["hello", 3]`.

Everything in `bprog` is white-space delimited, so you need white space between all the symbols, and white space between " and the string.



## Run modes

The program can run in two different modes:

* **NORMAL mode**
  * Write the input in postfix line for line. When you want to see the result, type in `:q`.
  * To see the stack without exiting, type `:s`
  * There should only be one variable left on the stack when the program is ended, if used correctly.
    * There is an error message being printed when the stack contains more than one item at the end of normal mode. In that case, the top element will still be returned as the answer, but the rest of the stack will be printed out as well.
* **REPL mode**
  * For each line of input, the stack will be printed. The program loops forever.



## Literals

All literals are simply pushed onto the stack.

* Integers, eg. `0` `1` `200`
* Floats, eg. `1.0` `200.0`
* Bools: `True` `False`
* Strings, delimited by double quotes `"` eg. `" hello world "`
* Lists, delimited by square brackets `[ ]` eg.
  * `[ 1 2 3 ]`
  * `[ " hello " " world " ]`
  * `[ 1 " hello " 2 " world " [ False True ] hello_symbol ]`

* Quotations (aka code blocks: `{ 1 2 + }`
  * Code block (aka just `block` or `quotation`) is a program sequence delimited by curly braces. For example `{ 1 + }` is a quotation that increments the current top element on the stack by 1.
    There is a function `exec` that picks a quotation from the top of the stack and executes it.
  * **Note**: in some of the functions quotation is expected, but, for a single instruction quotation the curly braces can be skipped.  So, for example `3 times { 10 }` is the same as `3 times 10` because the quotation contains ONLY one instruction. The notation without curly braces for single instruction quotations is more ergonomic. `times`, `map`, `foldl`, `each`, `if` should all work with both, quotations for multiple instructions and for single values (no curly braces needed). 
* Symbols: `a_symbol` Note: because there are no restrictions on symbols, anything that is not a reserved keyword in the language can become a valid symbol, and therefore, a function name.





## Operations

* Stack operations
  * `dup` ( x --> x x ) duplicates the top element on the stack
  * `swap` ( x y --> y x ) swaps the two top elements on the stack
  * `pop` ( x --> ) removes the top element from the stack
* Simple IO
  * We limit our language to TEXT only. All read/write operations operate on `String` types.
  * `print` ( x --> ) takes the top element from the stack and prints it to the standard output.
  * `read` ( --> x ) reads a line from standard input and puts it into the stack as string.
* String parsing

  * `parseInteger` ( s --> i ) takes a string from stack and converts it to Integer and puts it onto the stack
  * `parseFloat` ( s --> f ) same as above but for floats
  * `words` ( s --> list ) takes a string from the stack, splits it with Haskell `words` or Rust `split_whitespace` command, and puts a list of tokens onto the stack.
* Arithmetic operations

  * `+` ( x y --> x_plus_y ) - addition
  * `-` ( x y --> x_minus_y ) - subtraction
  * `*` ( x y --> mul ) - multiplication
  * `/` / `div` ( x y --> division ) - integer or floating point division
  * `<` ( x y --> bool) checks if `x < y`, and puts true or false on the stack
  * `>` ( x y --> bool) checks if `x > y`, and puts true or false on the stack
  * `==` ( x y --> bool ) checks if `x == y` and puts true or false on the stack
* Logical operations

  * `True` - literal
  * `False` - literal 
  * `&&` ( x y --> bool ) - logical AND
  * `||` ( x y --> bool ) - logical OR
  * `not` ( x --> bool ) - logical NOT. I've implemented it such that it also works like a negation on numbers, so, if you call it: `10 not` the program will put `-10` on top of the stack.

* List operations

    * `head` ( list --> item ) takes a list and returns its head
    * `tail` ( list --> tail ) takes a list and returns the tail
    * `empty` ( list --> bool ) takes a list and returns true if the list is empty 
    * `length` ( list --> len ) puts the length of a given list onto the stack
    * `append` ( item list --> list ) appends the item in front of the list
    * `cons` ( list1 list2 --> list3 ) concatenates both lists
    * `each quotation` ( list --> ) takes a list and a code block, and executes the code block on each of the elements of the list, e.g. `[ 1 2 3 ] each { print }` will print three lines with 1, 2, 3 respectively in each of the lines.
    * `map quotation` ( list --> newlist ) takes a list, and a block, and executes the block on each of the elements of the list, forming a new list that is put on the stack. E.g. `[ 1 2 3 ] map { 10 * }` will result in a list `[ 10 20 30 ]`
    * `foldl quotation` ( list initial_accumulator --> final_accumulator ) folds the list from left to right.  E.g. `[ 1 2 3 ] 0 foldl { + }` will result in `6` on top of the stack.

* Control flow

  * `if then_block else_block` ( bool ) `if` expression takes a boolean value from the stack, and executes the `then_code_block` if true, or `else_code_block` if false. The executed block operates in the context of the global stack.
    * The condition must be on the operand stack BEFORE `if` is called. Both blocks for `if` statement are needed, the THEN block and the ELSE block, but, one of the (or both) can be empty.  The code blocks are curly brace delimited.

  * `loop break block` execute the block until `break` becomes True. `break` and `block` are expected to be quotations. `break` evaluating to True or False does not leave that value on the stack (it is consumed by the `loop`)

  * `times block` ( num ) repeat the block `num` times


* Assignment to a symbol (variable)

  - Assignment `:=`
    - *variable_name* any value different from a symbol, e.g. number, bool, list or code_block `:=`

  - Function definition `fun`:
    - *function_name* quotation (code block) `fun`









## Examples







### Control flow

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



This is also valid code (white space is needed, indentation is not needed):

```
{
  if
  { " there was True on the stack " print }
  { " there was False on the stack " print }
}
```



This is a code block that you can assign a name and use in your program later on. This code block says nothing about the argument stack, so it can be applied in various contexts or assigned to a variable/function name.

Note, that assignment to variable expects the name to be deeper on the stack, and the value to be on top of the stack, this is why we had to do `swap` before `:=` or `fun`.

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





### Assignment to a symbol (variable)

* `sayhello { " hello " write } fun` defines a function `sayhello` that print `hello`.
* `age 10 :=` the symbol age now is of value `10`
  * can also be written as: `10 age swap :=`

To actually put a bound symbol onto the stack (without executing the function or without evaluating to a value in case of variable), we use `tick` operator, which is a single quote symbol `'`. Observe these two programs:

* `age 10 := age` -- produces 10 on top of the stack
* `age 10 := ' age` -- puts symbol age on top of the stack. The symbol `age` represents a variable, but the variable has not been evaluated to a value and the raw symbol is put onto the stack.  
* `age 10 := age 20 :=` -- this program is illegal, because assignment expects a symbol on the left-hand side, but instead, it gets 10.
* `age 10 := ' age 20 :=` -- this program defines a variable `age` and binds it to 10 first, then puts symbol `age` onto the stack, and re-binds it to value 20.
* Note, if the symbol has not been bound to a function or variable, using it will put it onto the stack raw. Because an unbound symbol evaluates to itself.

To evaluate bound symbol to a value, one can use `eval` function. Eval expects a symbol on the stack and returns the value of that symbol. In the case of variable it will be the value, and in the case of the function name it will be the quotation that is the function body. Observe:

* `age 10 := ' age eval` will bind age to value 10, then it will put age onto the stack as symbol, and then evaluate it to value 10. So, this program will end up with 10 on top of the stack.



## Error handling

The program should never panic. I have rather implemented that if an error occurs, i.e., the user tries to pop an empty stack, an appropriate error message will be printed out, but the program should not continue to run. In the case of any user-related error, the stack will not be changed.
