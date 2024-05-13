// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

//! Lint to check for public entry functions in the `randomness` module.
use crate::lint::{
    utils::{add_diagnostic_and_emit, LintConfig},
    visitor::ExpressionAnalysisVisitor,
};
use codespan::FileId;
use codespan_reporting::diagnostic::Diagnostic;
use move_model::{
    ast::{ExpData, Operation},
    model::{FunctionEnv, GlobalEnv, Visibility},
};
pub struct RandomnessPublicEntry;

impl Default for RandomnessPublicEntry {
    fn default() -> Self {
        Self::new()
    }
}

impl RandomnessPublicEntry {
    pub fn new() -> Self {
        Self {}
    }

    pub fn visitor() -> Box<dyn ExpressionAnalysisVisitor> {
        Box::new(Self::new())
    }

    /// Checks for randomness public entry function.
    fn check_randomness_public_entry(
        &self,
        func_env: &FunctionEnv,
        exp: &ExpData,
        env: &GlobalEnv,
        diags: &mut Vec<Diagnostic<FileId>>,
    ) {
        if let ExpData::Call(_, Operation::MoveFunction(module_id, _), _) = exp {
            let module_name = env
                .symbol_pool()
                .string(env.get_module(*module_id).get_name().name())
                .to_string();

            if &env.get_stdlib_address() == env.get_module(*module_id).self_address()
                && module_name == "randomness"
                && func_env.is_entry()
                && func_env.visibility() == Visibility::Public
            {
                let message = "Function `randomness` should not be used in public entry function.";
                add_diagnostic_and_emit(
                    &func_env.get_loc(),
                    message,
                    codespan_reporting::diagnostic::Severity::Warning,
                    env,
                    diags,
                );
            }
        };
    }
}

impl ExpressionAnalysisVisitor for RandomnessPublicEntry {
    fn post_visit_expression(
        &mut self,
        exp: &ExpData,
        func_env: &FunctionEnv,
        env: &GlobalEnv,
        _: &LintConfig,
        diags: &mut Vec<Diagnostic<FileId>>,
    ) {
        self.check_randomness_public_entry(func_env, exp, env, diags);
    }
}
