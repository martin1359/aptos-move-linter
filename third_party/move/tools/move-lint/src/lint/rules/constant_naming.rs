// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

//! `ConstantNamingVisitor` enforces a naming convention for constants in Move programs,
//! requiring them to follow an ALL_CAPS_SNAKE_CASE format. This lint checks each constant's name
//! within a module against this convention.
use crate::lint::{utils::add_diagnostic_and_emit, visitor::ExpressionAnalysisVisitor};
use codespan::FileId;
use codespan_reporting::diagnostic::Diagnostic;
use move_model::model::{GlobalEnv, ModuleEnv, NamedConstantEnv};
pub struct ConstantNamingVisitor;

impl Default for ConstantNamingVisitor {
    fn default() -> Self {
        Self::new()
    }
}

impl ConstantNamingVisitor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn visitor() -> Box<dyn ExpressionAnalysisVisitor> {
        Box::new(Self::new())
    }

    /// Checks if a constant name follows the all caps and snake case convention.
    fn check_constant_naming(
        &self,
        constant_env: &NamedConstantEnv,
        global_env: &GlobalEnv,
        diags: &mut Vec<Diagnostic<FileId>>,
    ) {
        let name = constant_env.get_name();
        let name_str = global_env.symbol_pool().string(name).to_string();

        if !is_all_caps_snake_case(&name_str) {
            let message = "Constant names should be in all caps and snake case.";
            add_diagnostic_and_emit(
                &constant_env.get_loc(),
                message,
                codespan_reporting::diagnostic::Severity::Warning,
                global_env,
                diags,
            );
        }
    }
}

impl ExpressionAnalysisVisitor for ConstantNamingVisitor {
    fn visit_module(
        &mut self,
        module: &ModuleEnv,
        _env: &GlobalEnv,
        diags: &mut Vec<Diagnostic<FileId>>,
    ) {
        let constants = module.get_named_constants();
        constants.for_each(|c| {
            self.check_constant_naming(&c, _env, diags);
        });
    }
}

/// Checks if the given string is in all caps, snake case, and possibly includes digits.
fn is_all_caps_snake_case(s: &str) -> bool {
    let is_upper_snake_case = s
        .chars()
        .all(|c| c.is_uppercase() || c == '_' || c.is_numeric());
    let has_letters = s.chars().any(char::is_alphabetic);
    is_upper_snake_case && has_letters
}
