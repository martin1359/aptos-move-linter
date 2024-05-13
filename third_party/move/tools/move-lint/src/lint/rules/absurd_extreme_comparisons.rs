// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

//! Lint to check for structs with event attribute but does not have drop and store ability.
use std::collections::BTreeMap;

use crate::lint::{
    utils::{add_diagnostic_and_emit, LintConfig},
    visitor::ExpressionAnalysisVisitor,
};
use codespan::FileId;
use codespan_reporting::diagnostic::Diagnostic;
use move_model::{
    ast::{Exp, ExpData, Operation, Pattern, Value},
    model::{FunctionEnv, GlobalEnv, NodeId, Parameter},
    symbol::Symbol,
    ty::Type,
};
use num::{integer, BigInt, ToPrimitive};

pub struct LikelyComparisonMistake {
    max_var_list: BTreeMap<Symbol, BigInt>,
    min_var_list: BTreeMap<Symbol, BigInt>,
}

impl Default for LikelyComparisonMistake {
    fn default() -> Self {
        Self::new()
    }
}

impl LikelyComparisonMistake {
    pub fn new() -> Self {
        Self {
            max_var_list: BTreeMap::new(),
            min_var_list: BTreeMap::new(),
        }
    }

    pub fn visitor() -> Box<dyn ExpressionAnalysisVisitor> {
        Box::new(Self::new())
    }

    fn collect_variable_types(
        &mut self,
        exp: &ExpData,
        env: &GlobalEnv,
        diags: &mut Vec<Diagnostic<FileId>>,
    ) {
        match exp {
            ExpData::Block(_, Pattern::Var(_, sym), Some(bind_exp), _) => {
                if let ExpData::Value(_, Value::Number(num)) = bind_exp.as_ref() {
                    self.classify_variable(sym, num.clone())
                }
            },

            ExpData::Assign(_, Pattern::Var(_, sym), bind_exp) => {
                if let ExpData::Value(_, Value::Number(num)) = bind_exp.as_ref() {
                    self.classify_variable(sym, num.clone())
                }
            },
            ExpData::IfElse(node_id, cond, _, _) => {
                if let ExpData::Call(_, op, exp_vec) = cond.as_ref() {
                    self.check_comparison_operations(op, exp_vec, node_id, env, diags);
                }
            },
            _ => {},
        }
    }

    fn classify_variable(&mut self, sym: &Symbol, value: BigInt) {
        if let Some(integer_value) = value.to_u128() {
            let u128_max = u128::MAX as u128;
            let u128_min = u128::MIN as u128;

            if integer_value == u128_max {
                self.max_var_list.insert(*sym, value);
            } else if integer_value == u128_min {
                self.min_var_list.insert(*sym, value);
            }
        }
    }

    fn check_comparison_operations(
        &mut self,
        op: &Operation,
        exp_vec: &Vec<Exp>,
        node_id: &NodeId,
        env: &GlobalEnv,
        diags: &mut Vec<Diagnostic<FileId>>,
    ) {
        let lhs = exp_vec.get(0);
        let rhs = exp_vec.get(1);
        if lhs.is_none() || rhs.is_none() {
            return;
        }
        match (op, lhs.unwrap().as_ref()) {
            (Operation::Lt, ExpData::Value(_, Value::Number(value))) => {
                if value == &BigInt::from(u128::MAX) {
                    let message = "Cannot compare parameter with max value";
                    add_diagnostic_and_emit(
                        &env.get_node_loc(*node_id),
                        message,
                        codespan_reporting::diagnostic::Severity::Warning,
                        env,
                        diags,
                    );
                }
            },
            (Operation::Gt, ExpData::Value(_, Value::Number(value))) => {
                if value == &BigInt::from(u128::MIN) {
                    let message = "Cannot compare parameter with min value";
                    add_diagnostic_and_emit(
                        &env.get_node_loc(*node_id),
                        message,
                        codespan_reporting::diagnostic::Severity::Warning,
                        env,
                        diags,
                    );
                }
            },
            _ => {},
        }

        match (op, rhs.unwrap().as_ref()) {
            (Operation::Gt, ExpData::Value(_, Value::Number(value))) => {
                if value == &BigInt::from(u128::MAX) {
                    let message = "Cannot compare parameter with max value";
                    add_diagnostic_and_emit(
                        &env.get_node_loc(*node_id),
                        message,
                        codespan_reporting::diagnostic::Severity::Warning,
                        env,
                        diags,
                    );
                }
            },
            (Operation::Lt, ExpData::Value(_, Value::Number(value))) => {
                if value == &BigInt::from(u128::MIN) {
                    let message = "Cannot compare parameter with min value";
                    add_diagnostic_and_emit(
                        &env.get_node_loc(*node_id),
                        message,
                        codespan_reporting::diagnostic::Severity::Warning,
                        env,
                        diags,
                    );
                }
            },
            _ => {},
        }
    }
}

impl ExpressionAnalysisVisitor for LikelyComparisonMistake {
    fn post_visit_expression(
        &mut self,
        exp: &ExpData,
        func_env: &FunctionEnv,
        env: &GlobalEnv,
        _: &LintConfig,
        diags: &mut Vec<Diagnostic<FileId>>,
    ) {
        self.collect_variable_types(exp, env, diags)
    }
}
