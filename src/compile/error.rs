// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2026 KyteKode

use std::process;

use colored::Colorize;

// Throws an error.
/*
# Codes
1: Terminal argument error
 */
pub(crate) fn throw_fatal_error(msg: impl Into<String>, code: i32) -> ! {
    eprintln!("{}", format!("{}{}", "Fatal Error: ".purple(), msg.into()));
    process::exit(code);
}

// Works like the expect() method of Result, but uses the custom throw_error() function.
pub(crate) fn expect<T, E>(res: Result<T, E>, msg: impl Into<String>, code: i32) -> T {
    match res {
        Ok(data) => data,
        Err(_) => throw_fatal_error(msg, code),
    }
}
