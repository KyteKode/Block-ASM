// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2026 KyteKode

use std::path::PathBuf;
use std::process;

use colored::Colorize;

use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub(crate) enum BasmError {
    #[error(": Could not get working directory")]
    WorkingDirectoryNotFound,

    #[error(": Could not get canonicalize path `{path}`")]
    CannotCanoncializePath { path: PathBuf },

    #[error(": Could not read source at path `{path}`")]
    CannotReadSource { path: PathBuf },

    #[error(": Cannot determine whether to output parsed or lexed data")]
    UndeterminedOutputType,

    #[error(": Unknown terminal argument `{arg}`")]
    UnknownTerminalArgument { arg: String },



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