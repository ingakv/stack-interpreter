use crate::error_handling::Error::*;

#[derive(PartialEq)]
pub(crate) enum Error {
    StackEmpty,
    UnknownSymbol,
    ExpectedNumber,
    ExpectedBoolOrNumber,
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

    match err {

        StackEmpty => print!("\n\tError: Stack is empty!\n\n"),

        UnknownSymbol => print!("\n\tError: Unknown symbol!\n\n"),

        ExpectedNumber => print!("\n\tError: Expected a number!\n\n"),

        ExpectedBoolOrNumber => print!("\n\tError: Expected a bool or a number!\n\n"),

        ExpectedListOrString => print!("\n\tError: Expected an enum!\n\n"),

        ExpectedQuotation => print!("\n\tError: Expected a quotation!\n\n"),

        ExpectedList => print!("\n\tError: Expected a list!\n\n"),

        ExpectedVariable => print!("\n\tError: Expected a variable!\n\n"),

        DivisionByZero => print!("\n\tError: Can't divide by 0!\n\n"),

        ProgramFinishedWithMultipleValues => print!("\n\tError: Program finished with multiple values!\n\n"),

        NumberConversionError => print!("\n\tError: Number couldn't be converted!\n\n"),

        IncompleteString => print!("\n\tError: Incomplete string!\n\n"),

        IncompleteList => print!("\n\tError: Incomplete list!\n\n"),

        IncompleteQuotation => print!("\n\tError: Incomplete quotation!\n\n"),

        _ => print!("\n\tError: Syntax error, try again!\n\n"),
    }
}


