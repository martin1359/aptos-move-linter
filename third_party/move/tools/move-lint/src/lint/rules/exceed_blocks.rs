// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

//! The lint identifies and warns about modules that exceed the allowed limit of dependencies.
//! This lint is useful for identifying modules that may be overly complex and difficult to maintain.
use crate::lint::{
    utils::{add_diagnostic_and_emit, LintConfig},
    visitor::ExpressionAnalysisVisitor,
};
use codespan::FileId;
use codespan_reporting::diagnostic::Diagnostic;
use move_bytecode_verifier::VerifierConfig;
use move_model::{
    ast::ExpData,
    model::{GlobalEnv, ModuleEnv},
};
#[derive(Debug)]
pub struct ExceedBlocksVisitor;

impl Default for ExceedBlocksVisitor {
    fn default() -> Self {
        Self::new()
    }
}

impl ExceedBlocksVisitor {
    fn new() -> Self {
        Self {}
    }

    pub fn visitor() -> Box<dyn ExpressionAnalysisVisitor> {
        Box::new(Self::new())
    }
    fn check_exceed_blocks(&self, module_env: &ModuleEnv, diags: &mut Vec<Diagnostic<FileId>>) {
        let mut total_blocks = 0;
        let config = VerifierConfig::production();
        for func_env in module_env.get_functions() {
            if let Some(func) = func_env.get_def().as_ref() {
                func.visit_pre_post(&mut |is_pre_visit, exp: &ExpData| {
                    if is_pre_visit {
                        if let ExpData::Block(_, _, _, _) = exp {
                            total_blocks += 1;
                            if let Some(max_blocks_count) = config.max_basic_blocks_in_script {
                                if total_blocks > max_blocks_count {
                                    let message = format!(
                                        "Script exceeds the allowed limit of {} blocks.",
                                        max_blocks_count
                                    );
                                    add_diagnostic_and_emit(
                                        &module_env.get_loc(),
                                        &message,
                                        codespan_reporting::diagnostic::Severity::Warning,
                                        module_env.env,
                                        diags,
                                    );
                                }
                            };
                        };
                    };
                    true
                });
            }
        }
    }
}

impl ExpressionAnalysisVisitor for ExceedBlocksVisitor {
    fn visit_module(
        &mut self,
        module: &ModuleEnv,
        _env: &GlobalEnv,
        diags: &mut Vec<Diagnostic<FileId>>,
    ) {
        self.check_exceed_blocks(module, diags);
    }
}
