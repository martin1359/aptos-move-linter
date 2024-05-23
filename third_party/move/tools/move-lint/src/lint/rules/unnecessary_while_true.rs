// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

//! This lint identifies and warns about `while(true)` loops in Move programs, suggesting the use of `loop` for clarity.
//! It enhances code readability by recommending a more idiomatic loop construct.
use crate::lint::{
    utils::{add_diagnostic_and_emit, LintConfig},
    visitor::ExpressionAnalysisVisitor,
};
use codespan::FileId;
use codespan_reporting::diagnostic::Diagnostic;
use move_model::{
    ast::{ExpData, Value},
    model::{FunctionEnv, GlobalEnv},
};
pub struct UnnecessaryWhileTrueVisitor;

impl Default for UnnecessaryWhileTrueVisitor {
    fn default() -> Self {
        Self::new()
    }
}

impl UnnecessaryWhileTrueVisitor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn visitor() -> Box<dyn ExpressionAnalysisVisitor> {
        Box::new(Self::new())
    }

    /// Checks for `while(true)` loops.
    fn check_unnecessary_while_true(
        &self,
        exp: &ExpData,
        env: &GlobalEnv,
        diags: &mut Vec<Diagnostic<FileId>>,
    ) {
        let ExpData::Loop(_, body) = exp else {
            return;
        };

        let ExpData::IfElse(_, cond, if_body, else_body) = body.as_ref() else {
            return;
        };
        let ExpData::Value(_, Value::Bool(true)) = cond.as_ref() else {
            return;
        };
        let ExpData::Sequence(_, vec_exp) = if_body.as_ref() else {
            return;
        };
        if vec_exp.len() >= 1 {
            let Some(for_loop_exp) = vec_exp.get(0) else {
                return;
            };
            let ExpData::IfElse(_, for_loop_var, _, _) = for_loop_exp.as_ref() else {
                return;
            };
            if !matches!(for_loop_var.as_ref(), ExpData::LocalVar(_, _)) {
                eprintln!("vec_exp {:?}", vec_exp);
                let message = "Unnecessary 'while(true)' detected. Consider using 'loop' instead.";
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

impl ExpressionAnalysisVisitor for UnnecessaryWhileTrueVisitor {
    fn post_visit_expression(
        &mut self,
        exp: &ExpData,
        _func_env: &FunctionEnv,
        env: &GlobalEnv,
        _: &LintConfig,
        diags: &mut Vec<Diagnostic<FileId>>,
    ) {
        self.check_unnecessary_while_true(exp, env, diags);
    }
}
