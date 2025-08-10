use crate::error_handling::Error::*;

#[derive(PartialEq)]
pub(crate) enum Error {
    StackEmpty,
    ExpectedBool,
    ExpectedNumber,
    ExpectedString,
    ExpectedListOrString,
    ExpectedQuotation,
    ExpectedList,
    ExpectedVariable,
    DivisionByZero,
    ProgramFinishedWithMultipleValues,
    NumberConversionError,
    IncompleteString,
    IncompleteList,
    IncompleteQuotation,
}


pub(crate) fn print_error(err: Error) {

    let msg = match err {

        StackEmpty => "Stack is empty",

        ExpectedBool => "Expected a bool",

        ExpectedNumber => "Expected a number",

        ExpectedString => "Expected a string",

        ExpectedListOrString => "Expected a list or a string",

        ExpectedQuotation => "Expected a quotation",

        ExpectedList => "Expected a list",

        ExpectedVariable => "Expected a variable",

        DivisionByZero => "Can't divide by 0",

        ProgramFinishedWithMultipleValues => "Program finished with multiple values",

        NumberConversionError => "String couldnt be converted to a number",

        IncompleteString => "Incomplete string",

        IncompleteList => "Incomplete list",

        IncompleteQuotation => "Incomplete quotation",
    };
    print!("\n\tError: {msg}!\n\n")
}
