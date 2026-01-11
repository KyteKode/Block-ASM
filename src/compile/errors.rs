// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2025 KyteKode

use std::process;

use colored::Colorize;

#[macro_export]
macro_rules! maybe_unreachable {
    () => {
        unreachable!(
            "Supposedly unreachable code at {}:{} is reachable",
            file!(),
            line!()
        )
    };
}
pub(crate) use maybe_unreachable;

pub(crate) fn throw_error(msg: impl Into<String>) -> ! {
    eprintln!("{}", format!("Error: {}", msg.into()).red());
    process::exit(1);
}

pub(crate) fn throw_warning(msg: &str) {
    eprintln!("{}", format!("Warning: {}", msg).yellow());
}
