// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

//! Detect consecutive 'if' statements with identical conditions are usually redundant and can be
//! refactored to improve code readability and maintainability.
use crate::lint::{
    utils::{add_diagnostic_and_emit, get_var_info_from_func_param, LintConfig},
    visitor::ExpressionAnalysisVisitor,
};
use codespan::FileId;
use codespan_reporting::diagnostic::Diagnostic;
use move_model::{
    ast::{ExpData, Operation},
    model::{FunctionEnv, GlobalEnv},
};
pub struct IfsSameCondVisitor {
}

impl Default for IfsSameCondVisitor {
    fn default() -> Self {
        Self::new()
    }
}

impl IfsSameCondVisitor {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn visitor() -> Box<dyn ExpressionAnalysisVisitor> {
        Box::new(Self::new())
    }

    /// Checks if the current 'if' condition is a duplicate and sets the condition for future checks.
    fn wrapper_condition_string(
        &mut self,
        exp: &ExpData,
        func_env: &FunctionEnv,
        env: &GlobalEnv,
    ) -> Option<String> {
        let vars = &mut Vec::new();
        let opers = &mut Vec::new();
        self.get_condition_string(exp, env, func_env, vars, opers)

    }

    /// Constructs a string representation of the given condition for comparison purposes.
    fn get_condition_string(
        &mut self,
        exp: &ExpData,
        env: &GlobalEnv,
        func_env: &FunctionEnv,
        vars: &mut Vec<String>,
        opers: &mut Vec<Operation>,
    ) -> Option<String> {
        if let ExpData::Call(_, Operation::Exists(_), _) = exp {
            return None;
        }
        match exp {
            ExpData::Call(_, oper, vec_exp) => {
                opers.push(oper.clone());
                vec_exp.iter().for_each(|e| {
                    self.get_condition_string(e, env, func_env, vars, opers);
                });
            },
            ExpData::Value(_, value) => vars.push(env.display(value).to_string()),
            ExpData::LocalVar(_, symbol) => {
                vars.push(env.symbol_pool().string(*symbol).to_string());
            },
            ExpData::Temporary(_, usize) => {
                let parameters = func_env.get_parameters();
                let param = get_var_info_from_func_param(*usize, &parameters);
                if let Some(param) = param {
                    vars.push(env.symbol_pool().string(param.0).to_string());
                }
            },
            _ => (),
        }

        Some(format!("{:?} {:?}", vars, opers))
    }

}

impl ExpressionAnalysisVisitor for IfsSameCondVisitor {
    fn visit_function_custom(
        &mut self,
        func_env: &FunctionEnv,
        env: &GlobalEnv,
        _: &LintConfig,
        diags: &mut Vec<Diagnostic<FileId>>,
    ) {
        let func = func_env.get_def();
        if let Some(func) = func.as_ref() {
            func.visit_pre_post(
                &mut (|up: bool, exp: &ExpData| {
                    if !up {
                        if let ExpData::IfElse(_, cond, _, if_else) = exp {
                            if let ExpData::IfElse(_, if_else_cond, _, _) = if_else.as_ref() {
                                if self.wrapper_condition_string(cond.as_ref(), func_env, env).is_none() || self.wrapper_condition_string(if_else_cond.as_ref(), func_env, env).is_none(){
                                    return true;
                                };
                                if self.wrapper_condition_string(cond.as_ref(), func_env, env) == self.wrapper_condition_string(if_else_cond.as_ref(), func_env, env){
                                    let message =
                                        "Detected consecutive if conditions with the same expression. Consider refactoring to avoid redundancy.";
                                    add_diagnostic_and_emit(
                                        &env.get_node_loc(exp.node_id()),
                                        message,
                                        codespan_reporting::diagnostic::Severity::Warning,
                                        env,
                                        diags,
                                    );
                                }
 
                            }
                        }
                    }
                    true
                }),
            );
        }
    }
}
