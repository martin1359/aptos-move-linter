// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use move_lint::lint;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(version, about = "An Aptos Move Linter")]
struct Args {
    #[clap(value_parser)]
    input_file: PathBuf,
}

fn main() {
    let args = Args::parse();
    lint::main(args.input_file);
}
