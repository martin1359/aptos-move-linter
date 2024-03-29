# Move Linter

A linter for the Move programming language used within the Aptos ecosystem. This linter helps you maintain code quality, enforce best practices, and catch potential errors early in the development cycle.

## Installation

**Prerequisites**

* Rust toolchain ([https://rustup.rs](https://rustup.rs))

**Installation via Cargo**

```bash
```

## Usage

Once installed, run the linter from your command line:

```bash
move-linter <input_move_file>
```

**Options**

* `--version`:  Displays the version of the Move Linter.
* `--help`:  Displays a help message with usage information.

## Lints

The Move Linter currently detects the following issues:

**Style and Formatting**

* **bool_comparison.rs:** Checks for redundant boolean comparisons or expressions.
* **combinable_bool_conditions.rs:** Identifies boolean conditions that can be combined or simplified. 
* **constant_naming.rs:** Enforces consistent naming conventions for constants.
* **deep_nesting.rs:** Warns about deeply nested code structures that might be difficult to read.
* **sorted_imports.rs:**  Ensures imports are sorted for better organization.

**Type Safety**

* **needless_bool.rs:** Detects unnecessary boolean expressions or operations.
* **out_of_bounds_array_indexing.rs:** Checks for potential array index out-of-bounds errors. 
* **unnecessary_type_conversion.rs:** Finds redundant or unnecessary type conversions.

**Security**

* **infinite_loop_detector.rs:** Detects loops that might run infinitely. 
* **overflow_multiplication_detector.rs:** Warns about multiplication operations that could lead to integer overflow.
* **shift_overflow.rs:** Checks for shift operations that might cause overflow.

**Best Practices**

* **complex_inline_function.rs:** Suggests refactoring complex inline functions for readability.
* **empty_loop.rs:** Detects empty loops that might have unintended side effects.
* **exceed_fields.rs:** Warns about exceeding a recommended number of struct fields.
* **exceed_params.rs:** Warns about exceeding a recommended number of function parameters.
* **explicit_self_assignments.rs:** Checks for unnecessary assignments to `self`.
* **getter_method_field_match.rs:** Suggests aligning getter method names with corresponding field names.
* **ifs_same_cond.rs:** Identifies if-statements with the same condition, suggesting consolidation.
* **meaningless_math_operations.rs:** Detects mathematical operations that have no effect.
* **multiplication_before_division.rs:** Recommends using multiplication before division for potential performance optimization.
* **redundant_deref_ref.rs:** Finds redundant reference-dereference patterns.
* **redundant_ref_deref.rs:** Finds redundant dereference-reference patterns.
* **return_at_end_of_block.rs:** Suggests avoiding unnecessary returns at the end of blocks. 
* **unconditional_exit_loop.rs:** Detects loops with unconditional exit conditions.
* **unmodified_mutable_argument_lint.rs:** Checks for mutable function arguments that are not modified.
* **unnecessary_mutable_reference.rs:** Identifies unnecessary use of mutable references.
* **unnecessary_while_true.rs:** Detects `while true` loops that could be simplified.
* **unused_borrow_global_mut.rs:** Finds unused mutable borrows of globals.
* **use_mul_div.rs:**  Suggests using the `*=` or `\=` operators where applicable.

## Contributing

This project is open source! Contributions are welcome. Please check out our contribution guidelines and code of conduct before getting started.

## Uninstall

To uninstall the Move Linter:

```bash
```