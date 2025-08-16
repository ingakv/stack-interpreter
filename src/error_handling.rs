use crate::error_handling::Error::*;

#[allow(dead_code)]
pub(crate) enum Error {
    StackEmpty,
    ExpectedNumberStringOrList,
    ExpectedNumber,
    ExpectedString,
    ExpectedListOrString,
    ExpectedQuotation,
    ExpectedList,
    ExpectedVariable,
    DivisionByZero,
    ProgramFinishedWithMultipleValues,
    NotEnoughValues,
    ExpectedBoolean,
    IncompleteString,
    IncompleteList,
    IncompleteQuotation,
}


pub(crate) fn print_error(err: Error) {

    let msg = match err {

        StackEmpty => "Stack is empty",

        ExpectedNumberStringOrList => "Expected a number, a string or a list",

        ExpectedNumber => "Expected a number",

        ExpectedString => "Expected a string",

        ExpectedListOrString => "Expected a list or a string",

        ExpectedQuotation => "Expected a quotation",

        ExpectedList => "Expected a list",

        ExpectedVariable => "Expected a variable",

        DivisionByZero => "Can't divide by 0",

        ProgramFinishedWithMultipleValues => "Program finished with multiple values",
        
        NotEnoughValues => "Not enough values on stack to perform operation",

        ExpectedBoolean => "Expected a boolean",

        IncompleteString => "Incomplete string",

        IncompleteList => "Incomplete list",

        IncompleteQuotation => "Incomplete quotation",
    };
    print!("\n\tError: {msg}!\n\n")
}
