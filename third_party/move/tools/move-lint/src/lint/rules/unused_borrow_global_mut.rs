// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

//! Detect borrow_global_mut variables that are not actually used to modify any data.
use crate::lint::{utils::add_diagnostic_and_emit, visitor::ExpressionAnalysisVisitor};
use codespan::FileId;
use codespan_reporting::diagnostic::Diagnostic;
use move_model::model::{FunctionEnv, GlobalEnv};
use move_stackless_bytecode::{
    function_target::FunctionTarget,
    stackless_bytecode::{AssignKind, AttrId, Bytecode, Operation},
    stackless_bytecode_generator::StacklessBytecodeGenerator,
};
use std::collections::{BTreeMap, HashSet};
// Struct representing the visitor for detecting unused mutable variables.
#[derive(Debug)]
pub struct UnusedBorrowGlobalMutVisitor {}

impl Default for UnusedBorrowGlobalMutVisitor {
    fn default() -> Self {
        Self::new()
    }
}

impl UnusedBorrowGlobalMutVisitor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn visitor() -> Box<dyn ExpressionAnalysisVisitor> {
        Box::new(Self::new())
    }

    fn process_bytecode(
        &mut self,
        bytecode: &Bytecode,
        func_target: &FunctionTarget,
        all_borrow_mut_refs: &mut BTreeMap<usize, AttrId>,
        assigned_refs: &mut BTreeMap<usize, usize>,
        borrow_fields: &mut BTreeMap<usize, usize>,
        modified: &mut HashSet<usize>,
    ) {
        match bytecode {
            Bytecode::Call(att_id, _, Operation::BorrowGlobal(_, _, _), _, _) => {
                let (_, mut_mods) = bytecode.modifies(func_target);
                for (idx, is_mutable_reference) in mut_mods {
                    if is_mutable_reference {
                        all_borrow_mut_refs.insert(idx, *att_id);
                    }
                }
            },
            Bytecode::Assign(_, target, src, AssignKind::Store) => {
                if let Some(attr_id) = all_borrow_mut_refs.get(src) {
                    let attr_id_clone = attr_id.clone();
                    all_borrow_mut_refs.remove_entry(src);
                    all_borrow_mut_refs.insert(*target, attr_id_clone);
                }
            },
            Bytecode::Assign(_, target, src, AssignKind::Move | AssignKind::Copy) => {
                assigned_refs.insert(*src, *target);
            },
            Bytecode::Call(_, dest, Operation::BorrowField(_, _, _, _), srcs, _) => {
                for (des, src) in dest.iter().zip(srcs.iter()) {
                    borrow_fields.insert(*src, *des);
                }
            },
            Bytecode::Call(_, _, Operation::WriteRef | Operation::Function(_, _, _), srcs, _) => {
                let (_, mut_mods) = bytecode.modifies(func_target);
                for (idx, _) in mut_mods {
                    modified.insert(idx);
                }
                for src in srcs {
                    modified.insert(*src);
                }
                self.find_unused_borrow_mut_refs(
                    all_borrow_mut_refs,
                    &assigned_refs,
                    &borrow_fields,
                    &modified,
                );
            },
            _ => {},
        }
    }

    fn find_unused_borrow_mut_refs(
        &self,
        all_borrow_mut_refs: &mut BTreeMap<usize, AttrId>,
        all_assigned_refs: &BTreeMap<usize, usize>,
        borrow_fields: &BTreeMap<usize, usize>,
        modified: &HashSet<usize>,
    ) {
        all_borrow_mut_refs.clone().iter().for_each(|(&ref_id, _)| {
            if let Some(&assigned_id) = all_assigned_refs.get(&ref_id) {
                let is_modified = if let Some(&mapped_ref) = borrow_fields.get(&assigned_id) {
                    modified.contains(&mapped_ref)
                } else {
                    modified.contains(&ref_id)
                };

                if is_modified {
                    all_borrow_mut_refs.remove_entry(&ref_id);
                }
            } else if modified.contains(&ref_id) {
                all_borrow_mut_refs.remove_entry(&ref_id);
            } else if let Some(&mapped_ref) = borrow_fields.get(&ref_id) {
                if modified.contains(&mapped_ref) {
                    all_borrow_mut_refs.remove_entry(&ref_id);
                }
            }
        });
    }
}

impl ExpressionAnalysisVisitor for UnusedBorrowGlobalMutVisitor {
    fn requires_bytecode_inspection(&self) -> bool {
        true
    }

    fn visit_function_with_bytecode(
        &mut self,
        func_env: &FunctionEnv,
        env: &GlobalEnv,
        diags: &mut Vec<Diagnostic<FileId>>,
    ) {
        if func_env.is_inline() {
            return;
        }
        let data = StacklessBytecodeGenerator::new(func_env).generate_function();
        // Handle to work with stackless functions -- function targets.
        let target = FunctionTarget::new(&func_env, &data);
        let byte_codes = target.get_bytecode();

        let mut all_borrow_mut_refs = BTreeMap::new();
        let mut all_assigned_refs = BTreeMap::new();
        let mut borrow_fields_refs = BTreeMap::new();
        let mut modified = HashSet::new();
        for bytecode in byte_codes {
            self.process_bytecode(
                bytecode,
                &target,
                &mut all_borrow_mut_refs,
                &mut all_assigned_refs,
                &mut borrow_fields_refs,
                &mut modified,
            );
        }

        for (_, attr_id) in all_borrow_mut_refs {
            let message = "Unused borrowed mutable variable. Consider normal borrow (borrow_global, vector::borrow, etc.) instead";
            add_diagnostic_and_emit(
                &target.get_bytecode_loc(attr_id),
                message,
                codespan_reporting::diagnostic::Severity::Warning,
                env,
                diags,
            );
        }
    }
}
