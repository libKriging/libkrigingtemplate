mod linear_regression;

use extendr_api::prelude::*;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod libkrigingtemplate;
    fn hello_world;
}

#[cfg(test)]
mod tests {
    #[test]
    fn rs_compile() {}
}
