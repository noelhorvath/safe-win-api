//! A library that offers an ergonomic interface for interacting with Windows API.

#![warn(
    clippy::await_holding_lock,
    clippy::borrow_as_ptr,
    clippy::char_lit_as_u8,
    clippy::checked_conversions,
    clippy::cognitive_complexity,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::doc_link_with_quotes,
    clippy::doc_markdown,
    clippy::empty_enum,
    clippy::empty_line_after_outer_attr,
    clippy::empty_structs_with_brackets,
    clippy::enum_glob_use,
    clippy::exit,
    clippy::expl_impl_clone_on_copy,
    clippy::explicit_deref_methods,
    clippy::explicit_into_iter_loop,
    clippy::fallible_impl_from,
    clippy::filter_map_next,
    clippy::flat_map_option,
    clippy::float_cmp,
    clippy::float_cmp_const,
    clippy::float_equality_without_abs,
    clippy::fn_params_excessive_bools,
    clippy::from_iter_instead_of_collect,
    clippy::if_let_mutex,
    clippy::implicit_clone,
    clippy::imprecise_flops,
    clippy::inefficient_to_string,
    clippy::invalid_upcast_comparisons,
    clippy::large_digit_groups,
    clippy::large_stack_arrays,
    clippy::large_types_passed_by_value,
    clippy::let_unit_value,
    clippy::linkedlist,
    clippy::lossy_float_literal,
    clippy::macro_use_imports,
    clippy::manual_ok_or,
    clippy::map_err_ignore,
    clippy::map_flatten,
    clippy::map_unwrap_or,
    clippy::match_on_vec_items,
    clippy::match_same_arms,
    clippy::match_wild_err_arm,
    clippy::match_wildcard_for_single_variants,
    clippy::mem_forget,
    clippy::mismatched_target_os,
    clippy::missing_const_for_fn,
    clippy::missing_docs_in_private_items,
    clippy::missing_enforced_import_renames,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc,
    clippy::mod_module_files,
    clippy::mut_mut,
    clippy::mutex_integer,
    clippy::needless_borrow,
    clippy::needless_continue,
    clippy::needless_for_each,
    clippy::option_if_let_else,
    clippy::option_option,
    clippy::panicking_unwrap,
    clippy::path_buf_push_overwrite,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::ptr_as_ptr,
    clippy::rc_mutex,
    clippy::ref_option_ref,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_functions_in_if_condition,
    clippy::semicolon_if_nothing_returned,
    clippy::shadow_unrelated,
    clippy::similar_names,
    clippy::single_match_else,
    clippy::string_add_assign,
    clippy::string_add,
    clippy::string_lit_as_bytes,
    clippy::string_to_string,
    clippy::suspicious_operation_groupings,
    clippy::todo,
    clippy::trait_duplication_in_bounds,
    clippy::undocumented_unsafe_blocks,
    clippy::unimplemented,
    clippy::unnested_or_patterns,
    clippy::unseparated_literal_suffix,
    clippy::unused_self,
    clippy::use_debug,
    clippy::use_self,
    clippy::used_underscore_binding,
    clippy::useless_let_if_seq,
    clippy::useless_transmute,
    clippy::verbose_file_reads,
    clippy::wildcard_dependencies,
    clippy::wildcard_imports,
    clippy::zero_sized_map_values,
    non_ascii_idents,
    noop_method_call,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    future_incompatible,
    keyword_idents,
    missing_debug_implementations,
    nonstandard_style
)]

/*
mod common;
mod diagnostics;
pub mod kernel;
pub mod messaging;
pub mod process;
pub mod registry;
pub mod shell;
pub mod thread;
pub mod window;
*/

/* const EPOCH_INTERVAL_DIFFERENCE: u64 = 116_444_736_000_000_000;
pub fn file_time_to_duration(file_time: FILETIME) -> SystemTime {
    // Safety: `FILETIME` has the same size as `u64`
    let intervals = unsafe { core::mem::transmute::<FILETIME, u64>(file_time) };
    if intervals < EPOCH_INTERVAL_DIFFERENCE {
        return SystemTime::UNIX_EPOCH;
    }

    SystemTime::UNIX_EPOCH + Duration::from_secs((intervals - EPOCH_INTERVAL_DIFFERENCE) / 10_000_000)
}
*/