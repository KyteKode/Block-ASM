// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2025 KyteKode
#![allow(unused)]

pub(crate) mod errors;

mod lexer;
mod parser;

pub use lexer::lex;
pub use parser::parse;

use std::collections::HashSet;

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
    TopLevel
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputType {
    #[default]
    SB3, 
    Parsed,
    Lexed
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

pub fn compile(data: &CompilationData) {
    todo!("Add compile functionality");
}