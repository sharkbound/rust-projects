use error_stack::Context;
use error_stack_derive::ErrorStack;

#[derive(ErrorStack, Debug)]
#[error_message("String input cannot be empty")]
pub struct EmptyStringError {}

#[derive(ErrorStack, Debug)]
#[error_message("Invalid string value: {0}")]
pub struct InvalidStringError(String);

#[derive(ErrorStack, Debug)]
pub enum EType {
    #[error_message("String input cannot be empty")]
    EmptyStringError,

    #[error_message("Invalid string value")]
    InvalidStringError(String),
}