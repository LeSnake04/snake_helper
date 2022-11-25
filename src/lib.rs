/*!
A crate providing helper macros
*/
#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(rustdoc::all)]
mod miette;
mod unwrap_or_print;

pub use unwrap_or::{unwrap_err_or, unwrap_none_or, unwrap_ok_or, unwrap_some_or};

#[macro_export]
/**
# Into None If
Turns `T` into an Option when condition privided
## Arguments
1. condition to check
2. object to wrap in Some if condition not met
*/
macro_rules! into_none_if(
    ($cond: expr, $obj: ident ) => (if $cond {None}else{Some($obj)})
);
