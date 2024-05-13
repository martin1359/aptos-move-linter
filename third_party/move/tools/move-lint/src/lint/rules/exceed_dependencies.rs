// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

//! The lint identifies and warns about modules that exceed the allowed limit of dependencies.
//! This lint is useful for identifying modules that may be overly complex and difficult to maintain.
use crate::lint::{utils::add_diagnostic_and_emit, visitor::ExpressionAnalysisVisitor};
use codespan::FileId;
use codespan_reporting::diagnostic::Diagnostic;
use move_bytecode_verifier::VerifierConfig;
use move_model::model::{GlobalEnv, ModuleEnv};
#[derive(Debug)]
pub struct ExceedDepsVisitor;

impl Default for ExceedDepsVisitor {
    fn default() -> Self {
        Self::new()
    }
}

impl ExceedDepsVisitor {
    fn new() -> Self {
        Self {}
    }

    pub fn visitor() -> Box<dyn ExpressionAnalysisVisitor> {
        Box::new(Self::new())
    }

    fn check_exceed_dependencies(
        &self,
        module_env: &ModuleEnv,
        diags: &mut Vec<Diagnostic<FileId>>,
    ) {
        let config = VerifierConfig::production();
        let import_count = module_env.get_use_decls().len();
        if let Some(max_deps_count) = config.max_basic_blocks_in_script {
            if import_count > max_deps_count {
                let message = format!(
                    "Module `{}` exceeds the allowed limit of {} dependencies.",
                    module_env.get_name().display(module_env.env),
                    max_deps_count
                );
                add_diagnostic_and_emit(
                    &module_env.get_loc(),
                    &message,
                    codespan_reporting::diagnostic::Severity::Warning,
                    module_env.env,
                    diags,
                );
            }
        }
    }
}

impl ExpressionAnalysisVisitor for ExceedDepsVisitor {
    fn visit_module(
        &mut self,
        module: &ModuleEnv,
        _env: &GlobalEnv,
        diags: &mut Vec<Diagnostic<FileId>>,
    ) {
        self.check_exceed_dependencies(module, diags);
    }
}
