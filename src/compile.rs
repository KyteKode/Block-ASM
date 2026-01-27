// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2026 KyteKode

mod error;
use error::{throw_errors, BasmError};

mod lexer;

use std::env;
use std::fs::read_to_string;

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
    pub source: String,

    pub version_flag: bool,
    pub verbose_flag: bool,

    pub output_type: OutputType,
}

// Parses command line arguments
pub fn handle_args(args: Vec<String>) -> CompileData {
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
            // Gets working directory and queues an error if it cannot be read
            let working_dir = env::current_dir();
            if working_dir.is_err() {
                errors.push(BasmError::WorkingDirectoryNotFound);
                continue;
            }

            // Canonicalizes a path.
            // For example, /hello/world/../myFolder becomes /hello/myFolder
            // If it cannot be canonicalized, it queues an error.
            let path = working_dir.unwrap().join(arg);
            let canonical = path.canonicalize();
            if canonical.is_err() {
                errors.push(BasmError::CannotCanoncializePath { path });
                continue;
            }

            // Reads the source file, and queues an error if it cannot be read.
            let file_data = read_to_string(canonical.unwrap());
            if file_data.is_err() {
                errors.push(BasmError::CannotReadSource { path });
                continue;
            }

            data.source = file_data.unwrap();
            continue;
        }

        // Checks if -o came before it
        // If it did, set the output filename to the argument
        if is_output_name {
            data.output_name = arg.clone();
            continue;
        }

        is_output_name = false;

        match arg.as_str() {
            "-o" => is_output_name = true,
            "--version" => data.version_flag = true,
            "-v" => data.verbose_flag = true,
            "-L" => {
                if data.output_type == OutputType::Parsed {
                    errors.push(BasmError::UndeterminedOutputType);
                }
                data.output_type = OutputType::Lexed;
            }
            "-P" => {
                if data.output_type == OutputType::Lexed {
                    errors.push(BasmError::UndeterminedOutputType);
                }
                data.output_type = OutputType::Parsed;
            }
            _ => errors.push(BasmError::UnknownTerminalArgument { arg: arg.clone() })
        }
    }

    // Throws all queued errors
    if !errors.is_empty() {
        throw_errors(errors);
    }

    data
}

// Currently unfinished.
// Only returns unit type because I'm not sure what the return type should be yet.
// Eventually, I will add the parsing, semantic analysis, etc to this function.
pub fn compile_with_data(data: CompileData) {
    lexer::lex(data.source);
    todo!("Finish compilation")
}

// Is ! type for the same reason `compile_with_data` is () type.
pub fn compile(args: Vec<String>) -> ! {
    compile_with_data(handle_args(args));
    todo!("Replace ! type with actual return type when changing compile_with_data return type")
}

/*
# Flags:
-o: Sets output filename. Argument immediately after is the name.
--version: Block-ASM version and target Scratch version.
-v: Prints more data during compilation.
-L: Returns lexed output.
-P: Returns parsed output.
 */

#[cfg(test)]
mod tests {
    use super::*;

    // MUST RUN IN Block-ASM/ DIRECTORY
    #[test]
    fn handle_source_arg() {
        let names = ["tests/abcde.txt", "tests/subdirectory/../hello.txt", "tests/subdirectory/../subdirectory/test.txt"].iter();

        let mut files_data = vec!["abcdefghijklmnopqrstuvwxyz test"];
        if cfg!(windows) {
            files_data.push("Hello, world!\r\nThis is a test!");
            files_data.push("testing\r\nTesting\r\n123");
        } else {
            files_data.push("Hello, world!\nThis is a test!");
            files_data.push("testing\nTesting\n123");
        }

        for (name, file_data) in names.zip(files_data.iter()) {
            let data = handle_args(vec![
                "basm".to_string(),
                name.to_string(),
            ]);

            assert_eq!(data.source, file_data.to_string());
        }
    }

    #[test]
    fn handle_output_name() {
        let names = ["test.sb3", "myProject123.sb3", "Unnamed-2019.sb3"].iter();

        for name in names {
            let data = handle_args(vec![
                "basm".to_string(),
                "tests/abcde.txt".to_string(),
                "-o".to_string(),
                name.to_string()
            ]);

            assert_eq!(data.output_name, name.to_string());
        }
    }

    #[test]
    fn handle_output_type() {
        let data = handle_args(vec![
            "basm".to_string(),
            "tests/abcde.txt".to_string(),
            "-L".to_string()
        ]);

        assert_eq!(data.output_type,OutputType::Lexed);

        let data = handle_args(vec![
            "basm".to_string(),
            "tests/abcde.txt".to_string(),
            "-P".to_string()
        ]);

        assert_eq!(data.output_type, OutputType::Parsed);
    }

    #[test]
    fn handle_verbose_flag() {
        let values = [true, false];
        for is_verbose in values.iter() {
            let mut args = vec![
                "basm".to_string(),
                "tests/abcde.txt".to_string()
            ];
            if *is_verbose {
                args.push("-v".to_string());
            }
            let data = handle_args(args);

            assert_eq!(data.verbose_flag, *is_verbose);
        }
    }

    #[test]
    fn handle_version_flag() {
        let values = [true, false];
        for show_version in values.iter() {
            let mut args = vec![
                "basm".to_string(),
                "tests/abcde.txt".to_string()
            ];
            if *show_version {
                args.push("--version".to_string());
            }
            let data = handle_args(args);

            assert_eq!(data.version_flag, *show_version);
        }
    }
}
