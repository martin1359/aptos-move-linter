// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

//! Lint to check for functions that take mutable references but don't actually mutate anything.
use crate::lint::{
    utils::{add_diagnostic_and_emit, get_var_info_from_func_param},
    visitor::ExpressionAnalysisVisitor,
};
use codespan::FileId;
use codespan_reporting::diagnostic::Diagnostic;
use move_model::{
    ast::{Exp, ExpData, Operation},
    model::{FunctionEnv, GlobalEnv, Parameter},
};

#[derive(Debug)]
pub struct UnmodifiedMutableArgumentLint;

impl Default for UnmodifiedMutableArgumentLint {
    fn default() -> Self {
        Self::new()
    }
}

impl UnmodifiedMutableArgumentLint {
    pub fn new() -> Self {
        Self {}
    }

    pub fn visitor() -> Box<dyn ExpressionAnalysisVisitor> {
        Box::new(Self::new())
    }

    /// Main function to check mutable parameters which are never modified.
    fn check_unmodified_mut_arguments(
        &self,
        func_env: &FunctionEnv,
        env: &GlobalEnv,
        diags: &mut Vec<Diagnostic<FileId>>,
    ) {
        for param in func_env.get_parameters().iter() {
            if param.1.is_mutable_reference() && !self.is_argument_modified(param, func_env) {
                let message = format!(
                    "Mutable parameter `{}` is never modified in function `{}`.",
                    param.0.display(func_env.symbol_pool()),
                    func_env.get_name().display(func_env.symbol_pool())
                );
                add_diagnostic_and_emit(
                    &func_env.get_loc(),
                    &message,
                    codespan_reporting::diagnostic::Severity::Warning,
                    env,
                    diags,
                );
            }
        }
    }

    /// Checks if a mutable parameter is modified within the function body.
    fn is_argument_modified(&self, param: &Parameter, func_env: &FunctionEnv) -> bool {
        if let Some(func_body) = func_env.get_def().as_ref() {
            let param_name = param.0.display(func_env.symbol_pool()).to_string();
            let mut is_modified = false;

            func_body.visit_pre_post(
                &mut (|up: bool, exp: &ExpData| {
                    if !up && !is_modified {
                        self.update_usage_status(exp, &param_name, func_env, &mut is_modified);
                    };
                    true
                }),
            );

            is_modified
        } else {
            false
        }
    }

    /// Helper function to update the usage status of a parameter based on the expression.
    fn update_usage_status(
        &self,
        exp: &ExpData,
        param_name: &str,
        func_env: &FunctionEnv,
        is_modified: &mut bool,
    ) {
        match exp {
            ExpData::Mutate(_, lhs, _) => {
                if let ExpData::Call(_, _, vec_exp) = lhs.as_ref() {
                    self.check_exp_vector(vec_exp, param_name, func_env, is_modified)
                }
            },
            ExpData::Call(_, Operation::MoveFunction(_, _), exp_vec) => {
                self.check_exp_vector(exp_vec, param_name, func_env, is_modified)
            },
            ExpData::Call(_, Operation::Select(_, _, _), exp_vec) => {
                self.check_exp_vector(exp_vec, param_name, func_env, is_modified);
            },
            _ => (),
        }
    }

    /// Checks expressions in vectors like arguments to function calls or mutates.
    fn check_exp_vector(
        &self,
        exp_vec: &Vec<Exp>,
        param_name: &str,
        func_env: &FunctionEnv,
        is_modified: &mut bool,
    ) {
        for exp in exp_vec {
            match exp.as_ref() {
                ExpData::Temporary(_, index) => {
                    if let Some(param) =
                        get_var_info_from_func_param(*index, &func_env.get_parameters())
                    {
                        if param.0.display(func_env.symbol_pool()).to_string() == param_name {
                            *is_modified = true;
                        }
                    }
                },
                ExpData::Call(_, _, exp_vec) => {
                    self.check_exp_vector(exp_vec, param_name, func_env, is_modified)
                },
                _ => (),
            }
        }
    }
}

impl ExpressionAnalysisVisitor for UnmodifiedMutableArgumentLint {
    fn visit_module(
        &mut self,
        _module: &move_model::model::ModuleEnv,
        _env: &GlobalEnv,
        diags: &mut Vec<Diagnostic<FileId>>,
    ) {
        for func_env in _module.get_functions() {
            if func_env.is_native() {
                return;
            };
            self.check_unmodified_mut_arguments(&func_env, _env, diags);
        }
    }
}
