// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2025 KyteKode

mod compile;

use std::env;

fn main() {
    compile::compile_using_args(env::args().collect());
}
