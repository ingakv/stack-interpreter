# BPROG - Stack Interpreter

This is an interpreter that can do simple math, io, parsing, and simple code blocks. I wrote this in Rust.

Currently I have Ints, Floats, Bools, Strings, Lists, and Code blocks as separate file types. It also has the ability to execute code for a list with the each function.

However, this version does not pass the majority of the tests. This is due to an issue concerning the order of which the variables or operations are executed.

The previous version with less functionality is commit 2d53a09e53b97eb99820a3f1148bf8b8e7d9efd9


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
I.e., if the stack is `[1, "hello", 2, +]` the program will extract 1 and 2, and push the result to the back, resulting in the stack now being `["hello", 3]`

The print function does not remove the element being printed from the list. All other functions do if I remember correctly.

### Functional Requirements

* The program can run in two different modes:
  * **NORMAL mode**
    * Write the input in postfix line for line. When you want to see the result, type in `:q`.
    * To see the stack without exiting, type `:s`
    * There should only be 1 variable left on the stack when the program is ended, if used correctly
  * **REPL mode**
    * For each line of input, the stack will be printed. The program loops forever.
  
* Literals are pushed onto the stack (integers, floats, bools, strings, lists)

* The program should be able to perform operations

  * Stack operations (swap, dup, pop)
  * Simple IO (print, read)
  * String parsing (parseInteger, parseFloat, words)
  * Arithmetic operations (+ ,  - ,  * , / , div , < , > , ==)
  * Logical operations (&& , || , not)

  * List operations (head, tail, empty, length, cons, append, each quotation, map quotation, foldl quotation)

  * Control flow (if, then, else, loop, times, break, block)
  * Assigning variables

* The program should handle errors and print them to help user understand what went wrong
* The program should be able to parse input into custom types.
  * Strings, bools, operators, lists

### Non-functional Requirements

* The program should be written in RUST

* The code should be sufficiently commented

* The program should run efficiently

* The program should be tested on all aspects of functionality

* It should be easy to add more functionality



**Implemented**

- NORMAL mode
- REPL mode
- Literals
- Stack operations
- Simple IO operations
- String parsing
- Arithmetic operations
- Logical operations
- List operations
- Parsing initial input
- Error handling that explains what the user did wrong
  - The program should never panic. I have rather implemented that if an error occurs, i.e., the user tries to pop an empty stack, an appropriate error message will be printed out, but the program should not continue to run. In the case of any user-related error, the stack will not be changed.




**Not Implemented**

- Control flow ( if, then, else, loop, times, break, block)
- Assigning variables





## Self Assessment



I have implemented a lot of the features except for those related to code blocks. I also have not yet had time to implement error handling. For now, if no changes were made on the stack during one input line, a basic error message will be printed. There is also an error message being printed when the stack contains more than one item at the end of normal mode. In that case, the top element will still be returned as the answer, but the rest of the stack will be printed out as well.

This iteration of the program passes 58 of the 104 tests that were a part of the original repository. All the tests that are failed contain code blocks, which I have not yet implemented. So all the tests related to features I have implemented passed.

REPL mode works as specified, with the stack being updated and printed for each time the user presses `enter`. Normal mode also works as specified, if I understood it correctly. I interpreted it as the user writes several input lines, but unlike REPL mode, the stack is not printed each time `enter` is pressed. Instead, when the user writes `:q`, the stack is executed and returns the answer. The reason for not executing the stack when `ctrl + d` is pressed, is because it was easier to debug it using another input. I am also under the assumption that if there were more than one element left in the stack when `:q` is pressed, that the program was supposed to panic and display an error message. I rather made my program work the same way regardless of how many items are on the stack. With the exception of printing a message to the user that there are still items in the stack, as well as printing out said items, in the case of the stack having more than one item. TLDR: the answer is still printed, since it's annoying for the user to type all of it again :cowboy_hat_face:
