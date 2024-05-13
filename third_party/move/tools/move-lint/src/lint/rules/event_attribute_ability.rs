// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

//! Lint to check for structs with event attribute but does not have drop and store ability.
use crate::lint::{
    utils::{add_diagnostic_and_emit, LintConfig},
    visitor::ExpressionAnalysisVisitor,
};
use codespan::FileId;
use codespan_reporting::diagnostic::Diagnostic;
use move_model::{
    ast::{ExpData, Operation, Pattern},
    model::{FunId, FunctionEnv, GlobalEnv, ModuleEnv, Visibility},
};
pub struct EventAttributeAbility;

impl Default for EventAttributeAbility {
    fn default() -> Self {
        Self::new()
    }
}

impl EventAttributeAbility {
    pub fn new() -> Self {
        Self {}
    }

    pub fn visitor() -> Box<dyn ExpressionAnalysisVisitor> {
        Box::new(Self::new())
    }

    /// Checks if a struct has event attribute but does not have drop and store ability.
    fn check_struct_ability(
        &self,
        module_env: &ModuleEnv,
        env: &GlobalEnv,
        diags: &mut Vec<Diagnostic<FileId>>,
    ) {
        module_env.get_structs().for_each(|struct_env| {
            struct_env.get_attributes().iter().for_each(|attr| {
                let attr_name = module_env.symbol_pool().string(attr.name()).to_string();
                if attr_name == "event" {
                    if !struct_env.get_abilities().has_drop()
                        || !struct_env.get_abilities().has_store()
                    {
                        let message =
                            "Struct has event attribute but does not have drop and store ability.";
                        add_diagnostic_and_emit(
                            &struct_env.get_loc(),
                            message,
                            codespan_reporting::diagnostic::Severity::Warning,
                            env,
                            diags,
                        );
                    }
                }
            });
        });
    }
}

impl ExpressionAnalysisVisitor for EventAttributeAbility {
    fn visit_module(
        &mut self,
        module: &ModuleEnv,
        env: &GlobalEnv,
        diags: &mut Vec<Diagnostic<FileId>>,
    ) {
        self.check_struct_ability(module, env, diags);
    }
}
