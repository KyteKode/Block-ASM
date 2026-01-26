// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2026 KyteKode

mod error;
use error::{throw_errors, BasmError};

use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputType {
    #[default]
    SB3,
    Lexed,
    Parsed,
}

#[derive(Debug, Clone, Default)]
pub struct CompileData {
    pub output_name: String,
    pub source_path: PathBuf,

    pub version_flag: bool,
    pub verbose_flag: bool,

    pub output_type: OutputType,
}

// Parses command line arguments
fn handle_args(args: Vec<String>) -> CompileData {
    let mut data = CompileData::default();

    let mut errors = Vec::new();

    // Tracks if the current argument is meant to be the output filename
    let mut is_output_name = false;

    for (idx, arg) in args.iter().enumerate() {
        // Skips the first argument because it's always the program name
        if idx == 0 {
            continue;
        }

        // Handles source path, which is always second argument
        if idx == 1 {
            let working_dir = env::current_dir();
            if working_dir.is_err() {
                errors.push(BasmError::WorkingDirectoryNotFound);
                continue;
            }

            let path = working_dir.unwrap().join(arg);
            let canonical = path.canonicalize();
            if canonical.is_err() {
                errors.push(BasmError::CannotCanoncializePath { path });
                continue;
            }

            data.source_path = canonical.unwrap();
            continue;
        }

        // Checks if -o came before it
        // If it did, set the output filename to the argument
        if is_output_name {
            data.output_name = arg.clone();
            continue;
        }

        match arg.as_str() {
            "-o" => is_output_name = true,
            "--version" => data.version_flag = true,
            "-v" | "--verbose" => data.verbose_flag = true,
            "-L" => {
                if data.output_type == OutputType::Parsed {
                    errors.push(BasmError::UndeterminedOutputType);
                }
            }
            "-P" => {
                if data.output_type == OutputType::Parsed {
                    errors.push(BasmError::UndeterminedOutputType);
                }
            }
            _ => errors.push(BasmError::UnknownTerminalArgument { arg: arg.clone() })
        }

        is_output_name = false;
    }
    
    if !errors.is_empty() {
        throw_errors(errors);
    }

    data
}

/*
# Flags:
-o: Sets output filename. Argument immediately after is the name.
--version: Block-ASM version and target Scratch version.
-v or --verbose: Prints more data during compilation.
-L: Returns lexed output.
-P: Returns parsed output.
 */
