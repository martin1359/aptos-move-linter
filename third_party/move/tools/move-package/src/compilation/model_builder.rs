// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::{
    compilation::compiled_package::make_source_and_deps_for_compiler,
    resolution::resolution_graph::ResolvedGraph, ModelConfig,
};
use anyhow::Result;
use itertools::Itertools;
use move_compiler::shared::PackagePaths;
use move_model::{model::GlobalEnv, options::ModelBuilderOptions, run_model_builder_with_options};

#[derive(Debug, Clone)]
pub struct ModelBuilder {
    resolution_graph: ResolvedGraph,
    model_config: ModelConfig,
}

impl ModelBuilder {
    pub fn create(resolution_graph: ResolvedGraph, model_config: ModelConfig) -> Self {
        Self {
            resolution_graph,
            model_config,
        }
    }

    // NOTE: If there are now renamings, then the root package has the global resolution of all named
    // addresses in the package graph in scope. So we can simply grab all of the source files
    // across all packages and build the Move model from that.
    // TODO: In the future we will need a better way to do this to support renaming in packages
    // where we want to support building a Move model.
    pub fn build_model(&self) -> Result<GlobalEnv> {
        // Make sure no renamings have been performed
        if let Some(pkg_name) = self.resolution_graph.contains_renaming() {
            anyhow::bail!(
                "Found address renaming in package '{}' when \
                    building Move model -- this is currently not supported",
                pkg_name
            )
        }

        // Targets are all files in the root package
        let root_name = &self.resolution_graph.root_package.package.name;
        let root_package = self.resolution_graph.get_package(root_name).clone();
        let deps_source_info = self
            .resolution_graph
            .package_table
            .iter()
            .filter_map(|(nm, pkg)| {
                if nm == root_name {
                    return None;
                }
                let mut dep_source_paths = pkg
                    .get_sources(&self.resolution_graph.build_options)
                    .unwrap();
                let mut source_available = true;
                // If source is empty, search bytecode
                if dep_source_paths.is_empty() {
                    dep_source_paths = pkg.get_bytecodes().unwrap();
                    source_available = false;
                }
                Some(Ok((
                    *nm,
                    dep_source_paths,
                    &pkg.resolution_table,
                    source_available,
                )))
            })
            .collect::<Result<Vec<_>>>()?;
        let (target, deps) = make_source_and_deps_for_compiler(
            &self.resolution_graph,
            &root_package,
            deps_source_info,
        )?;
        let (all_targets, all_deps) = if self.model_config.all_files_as_targets {
            let mut targets = vec![target];
            targets.extend(deps.into_iter().map(|(p, _)| p).collect_vec());
            (targets, vec![])
        } else {
            (vec![target], deps)
        };
        let (all_targets, all_deps) = match &self.model_config.target_filter {
            Some(filter) => {
                let mut new_targets = vec![];
                let mut new_deps = all_deps.into_iter().map(|(p, _)| p).collect_vec();
                for PackagePaths {
                    name,
                    paths,
                    named_address_map,
                } in all_targets
                {
                    let (true_targets, false_targets): (Vec<_>, Vec<_>) =
                        paths.into_iter().partition(|t| t.contains(filter));
                    if !true_targets.is_empty() {
                        new_targets.push(PackagePaths {
                            name,
                            paths: true_targets,
                            named_address_map: named_address_map.clone(),
                        })
                    }
                    if !false_targets.is_empty() {
                        new_deps.push(PackagePaths {
                            name,
                            paths: false_targets,
                            named_address_map,
                        })
                    }
                }
                (new_targets, new_deps)
            },
            None => (
                all_targets,
                all_deps.into_iter().map(|(p, _)| p).collect_vec(),
            ),
        };

        let skip_attribute_checks = self
            .resolution_graph
            .build_options
            .compiler_config
            .skip_attribute_checks;
        let known_attributes = &self
            .resolution_graph
            .build_options
            .compiler_config
            .known_attributes;
        run_model_builder_with_options(
            all_targets,
            all_deps,
            ModelBuilderOptions::default(),
            skip_attribute_checks,
            known_attributes,
        )
    }

    //This is the same as build_model, but with compile_via_model set to true because we attached bytecode to the model for the purpose of linting, particularly for the unused_borrow_global_mut linter.
    pub fn build_model_v2(&self) -> Result<GlobalEnv> {
        // Make sure no renamings have been performed
        if let Some(pkg_name) = self.resolution_graph.contains_renaming() {
            anyhow::bail!(
                "Found address renaming in package '{}' when \
                    building Move model -- this is currently not supported",
                pkg_name
            )
        }

        // Targets are all files in the root package
        let root_name = &self.resolution_graph.root_package.package.name;
        let root_package = self.resolution_graph.get_package(root_name).clone();
        let deps_source_info = self
            .resolution_graph
            .package_table
            .iter()
            .filter_map(|(nm, pkg)| {
                if nm == root_name {
                    return None;
                }
                let mut dep_source_paths = pkg
                    .get_sources(&self.resolution_graph.build_options)
                    .unwrap();
                let mut source_available = true;
                // If source is empty, search bytecode
                if dep_source_paths.is_empty() {
                    dep_source_paths = pkg.get_bytecodes().unwrap();
                    source_available = false;
                }
                Some(Ok((
                    *nm,
                    dep_source_paths,
                    &pkg.resolution_table,
                    source_available,
                )))
            })
            .collect::<Result<Vec<_>>>()?;
        let (target, deps) = make_source_and_deps_for_compiler(
            &self.resolution_graph,
            &root_package,
            deps_source_info,
        )?;
        let (all_targets, all_deps) = if self.model_config.all_files_as_targets {
            let mut targets = vec![target];
            targets.extend(deps.into_iter().map(|(p, _)| p).collect_vec());
            (targets, vec![])
        } else {
            (vec![target], deps)
        };
        let (all_targets, all_deps) = match &self.model_config.target_filter {
            Some(filter) => {
                let mut new_targets = vec![];
                let mut new_deps = all_deps.into_iter().map(|(p, _)| p).collect_vec();
                for PackagePaths {
                    name,
                    paths,
                    named_address_map,
                } in all_targets
                {
                    let (true_targets, false_targets): (Vec<_>, Vec<_>) =
                        paths.into_iter().partition(|t| t.contains(filter));
                    if !true_targets.is_empty() {
                        new_targets.push(PackagePaths {
                            name,
                            paths: true_targets,
                            named_address_map: named_address_map.clone(),
                        })
                    }
                    if !false_targets.is_empty() {
                        new_deps.push(PackagePaths {
                            name,
                            paths: false_targets,
                            named_address_map,
                        })
                    }
                }
                (new_targets, new_deps)
            },
            None => (
                all_targets,
                all_deps.into_iter().map(|(p, _)| p).collect_vec(),
            ),
        };

        let skip_attribute_checks = self
            .resolution_graph
            .build_options
            .compiler_config
            .skip_attribute_checks;
        let known_attributes = &self
            .resolution_graph
            .build_options
            .compiler_config
            .known_attributes;
        run_model_builder_with_options(
            all_targets,
            all_deps,
            ModelBuilderOptions {
                compile_via_model: true,
                ..Default::default()
            },
            skip_attribute_checks,
            known_attributes,
        )
    }
}
