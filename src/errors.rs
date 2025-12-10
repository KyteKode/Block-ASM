// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025 KyteKode

use std::process;

use colored::Colorize;

pub fn error(msg: String) -> ! {
    eprintln!(
        "{}",
        format!("Error: {}", msg).red()
    );
    process::exit(1);
}

pub fn warn(msg: &str) {
    eprintln!(
        "{}",
        format!("Warning: {}", msg).yellow()
    );
}