// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2025 KyteKode
#![allow(unused)]

pub(crate) mod errors;

mod lexer;
mod parser;

use errors::{throw_error, throw_warning};
pub use lexer::lex;
pub use parser::parse;

use std::collections::HashSet;
use std::{env, fs};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WarningType {
    Op,
    Uid,
    Parent,
    Next,
    In,
    Field,
    Mut,
    Shadow,
    TopLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputType {
    #[default]
    SB3,
    Parsed,
    Lexed,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CompilationData {
    pub out_name: String,
    pub source: String,

    pub version: bool,
    pub verbose: bool,
    pub log: bool,
    pub stdout: bool,

    pub out_type: OutputType,

    pub reverse: bool,

    pub warn: HashSet<WarningType>,
    pub no_warn: HashSet<WarningType>,
    pub wall: bool,
    pub werror: bool,
}

fn use_bit(bitfield: &mut u32, pos: u32) {
    *bitfield |= 1u32 << pos;
}

fn find_el(vector: &Vec<String>, bitfield: &mut u32, target: &str) -> (bool, usize) {
    let (found, pos) = vector
        .iter()
        .position(|x| x.as_str() == target)
        .map_or((false, 0), |pos| (true, pos));
    if found {
        use_bit(bitfield, pos as u32);
    }
    (found, pos)
}
pub fn compile(data: &CompilationData) {
    todo!("Add compile functionality");
}

pub fn compile_using_args(args: Vec<String>) {
    let mut data = CompilationData::default();

    let mut used: u32 = 0b11; // checks which terminal arguments are used

    let (found, pos) = find_el(&args, &mut used, "-o");
    if found {
        use_bit(&mut used, pos as u32);

        if args.len() == pos + 1 {
            eprintln!("Error: Could not find output filename after -o");
        } else {
            use_bit(&mut used, (pos + 1) as u32);
            data.out_name = args[pos + 1].clone();
        }
    }

    let (found, pos) = find_el(&args, &mut used, "-Otoken");
    if found {
        use_bit(&mut used, pos as u32);
        data.out_type = OutputType::Lexed;
    }

    let (found, pos) = find_el(&args, &mut used, "-Onode");
    if found {
        use_bit(&mut used, pos as u32);
        if data.out_type == OutputType::SB3 {
            data.out_type = OutputType::Parsed;
        } else {
            throw_error("Cannot use both -Otoken and -Onode".to_string())
        }
    }

    let warnings;
    {
        use crate::compile::WarningType::*;

        warnings = [
            (Op, "-Wop", "-Wno-op"),
            (Uid, "-Wuid", "-Wuid-op"),
            (Parent, "-Wparent", "-Wno-parent"),
            (Next, "-Wnext", "-Wno-next"),
            (In, "-Win", "-Wno-in"),
            (Field, "-Wfield", "-Wno-field"),
            (Mut, "-Wmut", "-Wno-mut"),
            (Shadow, "-Wshadow", "-Wno-shadow"),
            (TopLevel, "-Wtop", "Wno-top"),
        ];
    }

    for (warning, name, no_name) in warnings {
        let (found, _) = find_el(&args, &mut used, name);
        if found {
            data.warn.insert(warning);
        }

        let (found, _) = find_el(&args, &mut used, no_name);
        if found {
            data.no_warn.insert(warning);
        }
    }

    let (found, pos) = find_el(&args, &mut used, "--log");
    if found {
        use_bit(&mut used, pos as u32);
        data.log = true;
    }

    let (reverse_found, _) = find_el(&args, &mut used, "--reverse");
    if data.verbose {
        for (warning, name, no_name) in warnings {
            if data.warn.contains(&warning) {
                if reverse_found {
                    throw_warning(&format!("{} is ignored when using --reverse", name));
                }
                if data.wall {
                    throw_warning(&format!("{} is redundant when used with -Wall", name));
                }
                if data.no_warn.contains(&warning) {
                    throw_warning(&format!(
                        "{} and {} are redundant when used together",
                        name, no_name
                    ));
                }
            }
        }
    }

    if used & (1u32 << (args.len() - 1)) == 0 {
        let filename = args.last().unwrap();
        match fs::read_to_string(filename) {
            Ok(value) => {
                data.source = value;
                data.out_name = (*filename).clone();
            }
            Err(e) => {
                throw_error(format!("File {}, {}", filename, e));
            }
        };
    }

    compile(&data);
}

/*
   Flags:
   `-W[no-]uid`: Checks if the uids of blocks are unique.
   `-W[no-]op`: Checks if the opcodes of blocks are valid.
   `-W[no-]parent`: Checks if the uids the parent properties of blocks point to are valid.
   `-W[no-]next`: Checks if the uids the next properties of blocks point to are valid.
   `-W[no-]in`: Checks if the inputs of blocks are valid for their opcode.
   `-W[no-]field`: Checks if the fields of blocks are valid for their opcode.
   `-W[no-]mut`: Checks if the mutations of blocks are valid for their opcode.
   `-W[no-]shadow`: Checks if the shadow property of blocks are valid for their opcode.
   `-W[no-]top`: Checks if the top_level property of blocks are valid for their opcode.
   `-Wall`: Enables all validation warnings.
   `-Werror`: Treats warnings as errors.

   `-Otoken`: Only lexes the source.
   `-Onode`: Lexes the source (if not already lexed) and parses the source.

   `-o`: The argument given after will be the name of the compiled SB3. If not included, the name of the SB3 will be the name of the source file.
   `--stdout`: Prints the output SB3 to stdout instead of a source file. Not compatible with `-o`.
   `--verbose`: Outputs to stdout a summary of what Block-ASM is doing and some info about the results of each step in compilation.
   `--log`: Logs everything that Block-ASM does with timestamps

   `--reverse`: Reverses an SB3 back into source code. Not compatible with any of the flags above except for `-o`, `--stdout`, `--verbose`, and `--log`.

   `--version`: Outputs the current Block-ASM version and the Scratch version it generates SB3s for. Not compatible with any of the flags above
*/
