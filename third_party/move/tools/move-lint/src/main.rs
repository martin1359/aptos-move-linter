// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use move_lint::lint::{self, Args};

fn main() {
    let args = Args::parse();
    lint::main(args);
}
