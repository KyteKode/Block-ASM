// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025 KyteKode

use std::process;

use colored::Colorize;

pub fn throw_error(msg: String) -> ! {
    eprintln!(
        "{}",
        format!("Error: {}", msg).red()
    );
    process::exit(1);
}

pub fn throw_warning(msg: &str) {
    eprintln!(
        "{}",
        format!("Warning: {}", msg).yellow()
    );
}