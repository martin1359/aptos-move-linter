// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use std::{fs, path::PathBuf};

use codespan_reporting::term::{emit, termcolor::Buffer, Config};

#[test]
fn test_modules() {
    let path = PathBuf::from(
        "/Users/dmr/Projects/rust/aptos-move-linter/aptos-move/framework/aptos-framework",
    );
    let output_path = path.clone().join("output.exp");

    let (diags, files) = move_lint::lint::main(path);
    let mut writer = Buffer::no_color();
    for diag in diags {
        let _ = emit(&mut writer, &Config::default(), &files, &diag);
    }
    let diag_buffer = writer.into_inner();
    let rendered_diags = std::str::from_utf8(&diag_buffer).unwrap();
    eprintln!("{}", rendered_diags);
    fs::write(output_path, rendered_diags);
}
