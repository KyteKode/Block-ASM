// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2026 KyteKode

use std::process;
use std::path::PathBuf;

use colored::Colorize;

use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum BasmError {
    #[error(": Could not get working directory")]
    WorkingDirectoryNotFound,

    #[error(": Could not get canonicalize path `{path}`")]
    CannotCanoncializePath { path: PathBuf },

    #[error(": Cannot determine whether to output parsed or lexed data")]
    UndeterminedOutputType,

    #[error(": Unknown terminal argument `{path}`")]
    UnknownTerminalArgument { path: PathBuf },



    #[error("(line {line}): Found unclosed string literal")]
    UnclosedStringLiteral { line: u32 },

    #[error("(line {line}): Found unclosed target header")]
    UnclosedTargetHeader { line: u32 },

    #[error("(line {line}): Found unclosed monitor header")]
    UnclosedMonitorHeader { line: u32 },

    #[error("(line {line}): Could not parse unknown symbol `{data}`")]
    UnknownSymbol { line: u32, data: String }
}

pub(crate) fn throw_errors(errors: Vec<BasmError>) -> ! {
    for e in errors {
        eprintln!("{}{}", "Error".red(), e);
    }
    process::exit(1);
}

// Works like the expect() method of Result, but uses the custom throw_error() function.
pub(crate) fn expect<T, E>(res: Result<T, E>, msg: impl Into<String>, code: i32) -> T {
    match res {
        Ok(data) => data,
        Err(_) => {eprintln!("{}{}", "Error: ".red(), msg.into()); process::exit(1)}
    }
}
