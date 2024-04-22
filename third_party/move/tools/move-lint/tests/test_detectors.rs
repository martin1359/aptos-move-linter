// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;

#[test]
fn test_modules() {
    let path = PathBuf::from("tests").join("cases").join("exceed_params");
    move_lint::lint::main(path);
}
