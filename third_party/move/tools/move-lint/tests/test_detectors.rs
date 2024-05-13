// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use std::{fs, path::PathBuf};

use clap::Parser;
use codespan_reporting::term::{emit, termcolor::Buffer, Config};
use move_lint::lint::Args;

#[test]
fn tesqt_modules() {
    let path = PathBuf::from("tests/cases/rewards_pool");
    let output_path = path.clone().join("output.exp");
    let args = Args::parse();
    let (diags, files) = move_lint::lint::main(args);
    // let mut writer = Buffer::no_color();
    // for diag in diags {
    //     let _ = emit(&mut writer, &Config::default(), &files, &diag);
    // }
    // let diag_buffer = writer.into_inner();
    // let rendered_diags = std::str::from_utf8(&diag_buffer).unwrap();
    // fs::write(output_path, rendered_diags);
}
