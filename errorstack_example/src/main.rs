use error_stack::{FutureExt, IntoReport, Report, Result, ResultExt};

use crate::errors::parse_error::EType;

mod errors;
/*
 other examples here:
 https://github.com/hashintel/hash/tree/main/libs/error-stack/examples
 https://github.com/letsgetrusty/error-stack-demo/blob/master/src/main.rs
*/
fn main() {
    match check_string("123") {
        Ok(val) => {
            println!("{:?}", val)
        }
        Err(e) => {
            dbg!(e);
        }
    };
}

// NOTE: this is error_stack's Result here!
fn check_string(value: &str) -> Result<String, EType> {
    match value {
        "" => Err(EType::EmptyStringError).into_report(),
        val if val.chars().all(|x| x.is_digit(10)) => Ok(val.to_owned()),
        val => Err(EType::InvalidStringError(val.to_owned())).into_report().attach_printable("All characters in the string must be digits"),
    }
}
