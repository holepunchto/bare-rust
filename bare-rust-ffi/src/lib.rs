#![allow(non_camel_case_types)]

use std::ffi::c_void;
use std::option::Option;
use std::os::raw::{c_char, c_double, c_int, c_uchar};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct js_env_t {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct js_handle_scope_t {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct js_escapable_handle_scope_t {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct js_context_t {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct js_value_t {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct js_ref_t {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct js_deferred_t {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct js_callback_info_t {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct js_arraybuffer_backing_store_t {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct js_property_descriptor_t {
    pub version: c_int,
    pub name: *mut js_value_t,
    pub data: *mut c_void,
    pub attributes: c_int,
    pub method: js_function_cb,
    pub getter: js_function_cb,
    pub setter: js_function_cb,
    pub value: *mut js_value_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct js_delegate_callbacks_t {
    pub version: c_int,
    pub get: js_delegate_get_cb,
    pub has: js_delegate_has_cb,
    pub set: js_delegate_set_cb,
    pub delete_property: js_delegate_delete_property_cb,
    pub own_keys: js_delegate_own_keys_cb,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct js_type_tag_t {
    pub lower: u64,
    pub upper: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct js_callback_signature_t {
    pub version: c_int,
    pub result: c_int,
    pub args_len: usize,
    pub args: *mut c_int,
}

pub type js_function_cb = Option<
    unsafe extern "C" fn(env: *mut js_env_t, info: *mut js_callback_info_t) -> *mut js_value_t,
>;

pub type js_finalize_cb =
    Option<unsafe extern "C" fn(env: *mut js_env_t, data: *mut c_void, finalize_hint: *mut c_void)>;

pub type js_delegate_get_cb = Option<
    unsafe extern "C" fn(
        env: *mut js_env_t,
        property: *mut js_value_t,
        data: *mut c_void,
    ) -> *mut js_value_t,
>;

pub type js_delegate_has_cb = Option<
    unsafe extern "C" fn(env: *mut js_env_t, property: *mut js_value_t, data: *mut c_void) -> bool,
>;

pub type js_delegate_set_cb = Option<
    unsafe extern "C" fn(
        env: *mut js_env_t,
        property: *mut js_value_t,
        value: *mut js_value_t,
        data: *mut c_void,
    ) -> bool,
>;

pub type js_delegate_delete_property_cb = Option<
    unsafe extern "C" fn(env: *mut js_env_t, property: *mut js_value_t, data: *mut c_void) -> bool,
>;

pub type js_delegate_own_keys_cb =
    Option<unsafe extern "C" fn(arg1: *mut js_env_t, data: *mut c_void) -> *mut js_value_t>;

pub const JS_PENDING_EXCEPTION: c_int = -1;
pub const JS_UNCAUGHT_EXCEPTION: c_int = -2;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum js_value_type_t {
    js_undefined = 0,
    js_null = 1,
    js_boolean = 2,
    js_number = 3,
    js_string = 4,
    js_symbol = 5,
    js_object = 6,
    js_function = 7,
    js_external = 8,
    js_bigint = 9,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum js_typedarray_type_t {
    js_int8array = 0,
    js_uint8array = 1,
    js_uint8clampedarray = 2,
    js_int16array = 3,
    js_uint16array = 4,
    js_int32array = 5,
    js_uint32array = 6,
    js_float16array = 11,
    js_float32array = 7,
    js_float64array = 8,
    js_bigint64array = 9,
    js_biguint64array = 10,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum js_promise_state_t {
    js_promise_pending = 0,
    js_promise_fulfilled = 1,
    js_promise_rejected = 2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum js_key_collection_mode_t {
    js_key_include_prototypes = 0,
    js_key_own_only = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum js_key_conversion_mode_t {
    js_key_convert_to_string = 0,
    js_key_keep_numbers = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum js_property_filter_t {
    js_property_all_properties = 0,
    js_property_only_writable = 1,
    js_property_only_enumerable = 1 << 1,
    js_property_only_configurable = 1 << 2,
    js_property_skip_strings = 1 << 3,
    js_property_skip_symbols = 1 << 4,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum js_index_filter_t {
    js_index_include_indices = 0,
    js_index_skip_indices = 1,
}

unsafe extern "C" {
    pub fn js_open_handle_scope(env: *mut js_env_t, result: *mut *mut js_handle_scope_t) -> c_int;

    pub fn js_close_handle_scope(env: *mut js_env_t, scope: *mut js_handle_scope_t) -> c_int;

    pub fn js_open_escapable_handle_scope(
        env: *mut js_env_t,
        result: *mut *mut js_escapable_handle_scope_t,
    ) -> c_int;

    pub fn js_close_escapable_handle_scope(
        env: *mut js_env_t,
        scope: *mut js_escapable_handle_scope_t,
    ) -> c_int;

    pub fn js_escape_handle(
        env: *mut js_env_t,
        scope: *mut js_escapable_handle_scope_t,
        escapee: *mut js_value_t,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_create_context(env: *mut js_env_t, result: *mut *mut js_context_t) -> c_int;

    pub fn js_destroy_context(env: *mut js_env_t, context: *mut js_context_t) -> c_int;

    pub fn js_enter_context(env: *mut js_env_t, context: *mut js_context_t) -> c_int;

    pub fn js_exit_context(env: *mut js_env_t, context: *mut js_context_t) -> c_int;

    pub fn js_get_bindings(env: *mut js_env_t, result: *mut *mut js_value_t) -> c_int;

    pub fn js_run_script(
        env: *mut js_env_t,
        file: *const c_char,
        len: usize,
        offset: c_int,
        source: *mut js_value_t,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_create_reference(
        env: *mut js_env_t,
        value: *mut js_value_t,
        count: u32,
        result: *mut *mut js_ref_t,
    ) -> c_int;

    pub fn js_delete_reference(env: *mut js_env_t, reference: *mut js_ref_t) -> c_int;

    pub fn js_reference_ref(
        env: *mut js_env_t,
        reference: *mut js_ref_t,
        result: *mut u32,
    ) -> c_int;

    pub fn js_reference_unref(
        env: *mut js_env_t,
        reference: *mut js_ref_t,
        result: *mut u32,
    ) -> c_int;

    pub fn js_get_reference_value(
        env: *mut js_env_t,
        reference: *mut js_ref_t,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_define_class(
        env: *mut js_env_t,
        name: *const c_char,
        len: usize,
        constructor: js_function_cb,
        data: *mut c_void,
        properties: *const js_property_descriptor_t,
        properties_len: usize,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_define_properties(
        env: *mut js_env_t,
        object: *mut js_value_t,
        properties: *const js_property_descriptor_t,
        properties_len: usize,
    ) -> c_int;

    pub fn js_wrap(
        env: *mut js_env_t,
        object: *mut js_value_t,
        data: *mut c_void,
        finalize_cb: js_finalize_cb,
        finalize_hint: *mut c_void,
        result: *mut *mut js_ref_t,
    ) -> c_int;

    pub fn js_unwrap(
        env: *mut js_env_t,
        object: *mut js_value_t,
        result: *mut *mut c_void,
    ) -> c_int;

    pub fn js_remove_wrap(
        env: *mut js_env_t,
        object: *mut js_value_t,
        result: *mut *mut c_void,
    ) -> c_int;

    pub fn js_create_delegate(
        env: *mut js_env_t,
        callbacks: *const js_delegate_callbacks_t,
        data: *mut c_void,
        finalize_cb: js_finalize_cb,
        finalize_hint: *mut c_void,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_add_finalizer(
        env: *mut js_env_t,
        object: *mut js_value_t,
        data: *mut c_void,
        finalize_cb: js_finalize_cb,
        finalize_hint: *mut c_void,
        result: *mut *mut js_ref_t,
    ) -> c_int;

    pub fn js_add_type_tag(
        env: *mut js_env_t,
        object: *mut js_value_t,
        tag: *const js_type_tag_t,
    ) -> c_int;

    pub fn js_check_type_tag(
        env: *mut js_env_t,
        object: *mut js_value_t,
        tag: *const js_type_tag_t,
        result: *mut bool,
    ) -> c_int;

    pub fn js_create_int32(env: *mut js_env_t, value: i32, result: *mut *mut js_value_t) -> c_int;

    pub fn js_create_uint32(env: *mut js_env_t, value: u32, result: *mut *mut js_value_t) -> c_int;

    pub fn js_create_int64(env: *mut js_env_t, value: i64, result: *mut *mut js_value_t) -> c_int;

    pub fn js_create_double(
        env: *mut js_env_t,
        value: c_double,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_create_bigint_int64(
        env: *mut js_env_t,
        value: i64,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_create_bigint_uint64(
        env: *mut js_env_t,
        value: u64,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_create_string_utf8(
        env: *mut js_env_t,
        string: *const c_uchar,
        len: usize,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_create_external_string_utf8(
        env: *mut js_env_t,
        string: *mut c_uchar,
        len: usize,
        finalize_cb: js_finalize_cb,
        finalize_hint: *mut c_void,
        result: *mut *mut js_value_t,
        copied: *mut bool,
    ) -> c_int;

    pub fn js_create_property_key_utf8(
        env: *mut js_env_t,
        string: *const c_uchar,
        len: usize,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_create_symbol(
        env: *mut js_env_t,
        description: *mut js_value_t,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_symbol_for(
        env: *mut js_env_t,
        description: *const c_char,
        len: usize,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_create_object(env: *mut js_env_t, result: *mut *mut js_value_t) -> c_int;

    pub fn js_create_function(
        env: *mut js_env_t,
        name: *const c_char,
        len: usize,
        cb: js_function_cb,
        data: *mut c_void,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_create_function_with_source(
        env: *mut js_env_t,
        name: *const c_char,
        name_len: usize,
        file: *const c_char,
        file_len: usize,
        args: *const *mut js_value_t,
        args_len: usize,
        offset: c_int,
        source: *mut js_value_t,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_create_typed_function(
        env: *mut js_env_t,
        name: *const c_char,
        len: usize,
        cb: js_function_cb,
        signature: *const js_callback_signature_t,
        address: *const c_void,
        data: *mut c_void,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_create_array(env: *mut js_env_t, result: *mut *mut js_value_t) -> c_int;

    pub fn js_create_array_with_length(
        env: *mut js_env_t,
        len: usize,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_create_external(
        env: *mut js_env_t,
        data: *mut c_void,
        finalize_cb: js_finalize_cb,
        finalize_hint: *mut c_void,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_create_error(
        env: *mut js_env_t,
        code: *mut js_value_t,
        message: *mut js_value_t,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_create_type_error(
        env: *mut js_env_t,
        code: *mut js_value_t,
        message: *mut js_value_t,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_create_range_error(
        env: *mut js_env_t,
        code: *mut js_value_t,
        message: *mut js_value_t,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_create_syntax_error(
        env: *mut js_env_t,
        code: *mut js_value_t,
        message: *mut js_value_t,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_create_reference_error(
        env: *mut js_env_t,
        code: *mut js_value_t,
        message: *mut js_value_t,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_create_promise(
        env: *mut js_env_t,
        deferred: *mut *mut js_deferred_t,
        promise: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_resolve_deferred(
        env: *mut js_env_t,
        deferred: *mut js_deferred_t,
        resolution: *mut js_value_t,
    ) -> c_int;

    pub fn js_reject_deferred(
        env: *mut js_env_t,
        deferred: *mut js_deferred_t,
        resolution: *mut js_value_t,
    ) -> c_int;

    pub fn js_get_promise_state(
        env: *mut js_env_t,
        promise: *mut js_value_t,
        result: *mut js_promise_state_t,
    ) -> c_int;

    pub fn js_get_promise_result(
        env: *mut js_env_t,
        promise: *mut js_value_t,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_create_arraybuffer(
        env: *mut js_env_t,
        len: usize,
        data: *mut *mut c_void,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_create_arraybuffer_with_backing_store(
        env: *mut js_env_t,
        backing_store: *mut js_arraybuffer_backing_store_t,
        data: *mut *mut c_void,
        len: *mut usize,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_create_unsafe_arraybuffer(
        env: *mut js_env_t,
        len: usize,
        data: *mut *mut c_void,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_create_external_arraybuffer(
        env: *mut js_env_t,
        data: *mut c_void,
        len: usize,
        finalize_cb: js_finalize_cb,
        finalize_hint: *mut c_void,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_detach_arraybuffer(env: *mut js_env_t, arraybuffer: *mut js_value_t) -> c_int;

    pub fn js_get_arraybuffer_backing_store(
        env: *mut js_env_t,
        arraybuffer: *mut js_value_t,
        result: *mut *mut js_arraybuffer_backing_store_t,
    ) -> c_int;

    pub fn js_create_sharedarraybuffer(
        env: *mut js_env_t,
        len: usize,
        data: *mut *mut c_void,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_create_sharedarraybuffer_with_backing_store(
        env: *mut js_env_t,
        backing_store: *mut js_arraybuffer_backing_store_t,
        data: *mut *mut c_void,
        len: *mut usize,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_create_unsafe_sharedarraybuffer(
        env: *mut js_env_t,
        len: usize,
        data: *mut *mut c_void,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_create_external_sharedarraybuffer(
        env: *mut js_env_t,
        data: *mut c_void,
        len: usize,
        finalize_cb: js_finalize_cb,
        finalize_hint: *mut c_void,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_get_sharedarraybuffer_backing_store(
        env: *mut js_env_t,
        sharedarraybuffer: *mut js_value_t,
        result: *mut *mut js_arraybuffer_backing_store_t,
    ) -> c_int;

    pub fn js_release_arraybuffer_backing_store(
        env: *mut js_env_t,
        backing_store: *mut js_arraybuffer_backing_store_t,
    ) -> c_int;

    pub fn js_create_typedarray(
        env: *mut js_env_t,
        variant: js_typedarray_type_t,
        len: usize,
        arraybuffer: *mut js_value_t,
        offset: usize,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_create_dataview(
        env: *mut js_env_t,
        len: usize,
        arraybuffer: *mut js_value_t,
        offset: usize,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_coerce_to_boolean(
        env: *mut js_env_t,
        value: *mut js_value_t,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_coerce_to_number(
        env: *mut js_env_t,
        value: *mut js_value_t,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_coerce_to_string(
        env: *mut js_env_t,
        value: *mut js_value_t,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_coerce_to_object(
        env: *mut js_env_t,
        value: *mut js_value_t,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_typeof(
        env: *mut js_env_t,
        value: *mut js_value_t,
        result: *mut js_value_type_t,
    ) -> c_int;

    pub fn js_instanceof(
        env: *mut js_env_t,
        object: *mut js_value_t,
        constructor: *mut js_value_t,
        result: *mut bool,
    ) -> c_int;

    pub fn js_is_undefined(env: *mut js_env_t, value: *mut js_value_t, result: *mut bool) -> c_int;

    pub fn js_is_null(env: *mut js_env_t, value: *mut js_value_t, result: *mut bool) -> c_int;

    pub fn js_is_boolean(env: *mut js_env_t, value: *mut js_value_t, result: *mut bool) -> c_int;

    pub fn js_is_number(env: *mut js_env_t, value: *mut js_value_t, result: *mut bool) -> c_int;

    pub fn js_is_int32(env: *mut js_env_t, value: *mut js_value_t, result: *mut bool) -> c_int;

    pub fn js_is_uint32(env: *mut js_env_t, value: *mut js_value_t, result: *mut bool) -> c_int;

    pub fn js_is_string(env: *mut js_env_t, value: *mut js_value_t, result: *mut bool) -> c_int;

    pub fn js_is_symbol(env: *mut js_env_t, value: *mut js_value_t, result: *mut bool) -> c_int;

    pub fn js_is_object(env: *mut js_env_t, value: *mut js_value_t, result: *mut bool) -> c_int;

    pub fn js_is_function(env: *mut js_env_t, value: *mut js_value_t, result: *mut bool) -> c_int;

    pub fn js_is_async_function(
        env: *mut js_env_t,
        value: *mut js_value_t,
        result: *mut bool,
    ) -> c_int;

    pub fn js_is_generator_function(
        env: *mut js_env_t,
        value: *mut js_value_t,
        result: *mut bool,
    ) -> c_int;

    pub fn js_is_generator(env: *mut js_env_t, value: *mut js_value_t, result: *mut bool) -> c_int;

    pub fn js_is_arguments(env: *mut js_env_t, value: *mut js_value_t, result: *mut bool) -> c_int;

    pub fn js_is_array(env: *mut js_env_t, value: *mut js_value_t, result: *mut bool) -> c_int;

    pub fn js_is_external(env: *mut js_env_t, value: *mut js_value_t, result: *mut bool) -> c_int;

    pub fn js_is_wrapped(env: *mut js_env_t, value: *mut js_value_t, result: *mut bool) -> c_int;

    pub fn js_is_delegate(env: *mut js_env_t, value: *mut js_value_t, result: *mut bool) -> c_int;

    pub fn js_is_bigint(env: *mut js_env_t, value: *mut js_value_t, result: *mut bool) -> c_int;

    pub fn js_is_date(env: *mut js_env_t, value: *mut js_value_t, result: *mut bool) -> c_int;

    pub fn js_is_regexp(env: *mut js_env_t, value: *mut js_value_t, result: *mut bool) -> c_int;

    pub fn js_is_error(env: *mut js_env_t, value: *mut js_value_t, result: *mut bool) -> c_int;

    pub fn js_is_promise(env: *mut js_env_t, value: *mut js_value_t, result: *mut bool) -> c_int;

    pub fn js_is_proxy(env: *mut js_env_t, value: *mut js_value_t, result: *mut bool) -> c_int;

    pub fn js_is_map(env: *mut js_env_t, value: *mut js_value_t, result: *mut bool) -> c_int;

    pub fn js_is_map_iterator(
        env: *mut js_env_t,
        value: *mut js_value_t,
        result: *mut bool,
    ) -> c_int;

    pub fn js_is_set(env: *mut js_env_t, value: *mut js_value_t, result: *mut bool) -> c_int;

    pub fn js_is_set_iterator(
        env: *mut js_env_t,
        value: *mut js_value_t,
        result: *mut bool,
    ) -> c_int;

    pub fn js_is_weak_map(env: *mut js_env_t, value: *mut js_value_t, result: *mut bool) -> c_int;

    pub fn js_is_weak_set(env: *mut js_env_t, value: *mut js_value_t, result: *mut bool) -> c_int;

    pub fn js_is_weak_ref(env: *mut js_env_t, value: *mut js_value_t, result: *mut bool) -> c_int;

    pub fn js_is_arraybuffer(
        env: *mut js_env_t,
        value: *mut js_value_t,
        result: *mut bool,
    ) -> c_int;

    pub fn js_is_detached_arraybuffer(
        env: *mut js_env_t,
        value: *mut js_value_t,
        result: *mut bool,
    ) -> c_int;

    pub fn js_is_sharedarraybuffer(
        env: *mut js_env_t,
        value: *mut js_value_t,
        result: *mut bool,
    ) -> c_int;

    pub fn js_is_typedarray(env: *mut js_env_t, value: *mut js_value_t, result: *mut bool)
        -> c_int;

    pub fn js_is_int8array(env: *mut js_env_t, value: *mut js_value_t, result: *mut bool) -> c_int;

    pub fn js_is_uint8array(env: *mut js_env_t, value: *mut js_value_t, result: *mut bool)
        -> c_int;

    pub fn js_is_uint8clampedarray(
        env: *mut js_env_t,
        value: *mut js_value_t,
        result: *mut bool,
    ) -> c_int;

    pub fn js_is_int16array(env: *mut js_env_t, value: *mut js_value_t, result: *mut bool)
        -> c_int;

    pub fn js_is_uint16array(
        env: *mut js_env_t,
        value: *mut js_value_t,
        result: *mut bool,
    ) -> c_int;

    pub fn js_is_int32array(env: *mut js_env_t, value: *mut js_value_t, result: *mut bool)
        -> c_int;

    pub fn js_is_uint32array(
        env: *mut js_env_t,
        value: *mut js_value_t,
        result: *mut bool,
    ) -> c_int;

    pub fn js_is_float16array(
        env: *mut js_env_t,
        value: *mut js_value_t,
        result: *mut bool,
    ) -> c_int;

    pub fn js_is_float32array(
        env: *mut js_env_t,
        value: *mut js_value_t,
        result: *mut bool,
    ) -> c_int;

    pub fn js_is_float64array(
        env: *mut js_env_t,
        value: *mut js_value_t,
        result: *mut bool,
    ) -> c_int;

    pub fn js_is_bigint64array(
        env: *mut js_env_t,
        value: *mut js_value_t,
        result: *mut bool,
    ) -> c_int;

    pub fn js_is_biguint64array(
        env: *mut js_env_t,
        value: *mut js_value_t,
        result: *mut bool,
    ) -> c_int;

    pub fn js_is_dataview(env: *mut js_env_t, value: *mut js_value_t, result: *mut bool) -> c_int;

    pub fn js_is_module_namespace(
        env: *mut js_env_t,
        value: *mut js_value_t,
        result: *mut bool,
    ) -> c_int;

    pub fn js_strict_equals(
        env: *mut js_env_t,
        a: *mut js_value_t,
        b: *mut js_value_t,
        result: *mut bool,
    ) -> c_int;

    pub fn js_get_global(env: *mut js_env_t, result: *mut *mut js_value_t) -> c_int;

    pub fn js_get_undefined(env: *mut js_env_t, result: *mut *mut js_value_t) -> c_int;

    pub fn js_get_null(env: *mut js_env_t, result: *mut *mut js_value_t) -> c_int;

    pub fn js_get_boolean(env: *mut js_env_t, value: bool, result: *mut *mut js_value_t) -> c_int;

    pub fn js_get_value_bool(
        env: *mut js_env_t,
        value: *mut js_value_t,
        result: *mut bool,
    ) -> c_int;

    pub fn js_get_value_int32(
        env: *mut js_env_t,
        value: *mut js_value_t,
        result: *mut i32,
    ) -> c_int;

    pub fn js_get_value_uint32(
        env: *mut js_env_t,
        value: *mut js_value_t,
        result: *mut u32,
    ) -> c_int;

    pub fn js_get_value_int64(
        env: *mut js_env_t,
        value: *mut js_value_t,
        result: *mut i64,
    ) -> c_int;

    pub fn js_get_value_double(
        env: *mut js_env_t,
        value: *mut js_value_t,
        result: *mut c_double,
    ) -> c_int;

    pub fn js_get_value_bigint_int64(
        env: *mut js_env_t,
        value: *mut js_value_t,
        result: *mut i64,
        lossless: *mut bool,
    ) -> c_int;

    pub fn js_get_value_bigint_uint64(
        env: *mut js_env_t,
        value: *mut js_value_t,
        result: *mut u64,
        lossless: *mut bool,
    ) -> c_int;

    pub fn js_get_value_string_utf8(
        env: *mut js_env_t,
        value: *mut js_value_t,
        string: *mut c_uchar,
        len: usize,
        result: *mut usize,
    ) -> c_int;

    pub fn js_get_value_external(
        env: *mut js_env_t,
        value: *mut js_value_t,
        result: *mut *mut c_void,
    ) -> c_int;

    pub fn js_get_value_date(
        env: *mut js_env_t,
        value: *mut js_value_t,
        result: *mut c_double,
    ) -> c_int;

    pub fn js_get_array_length(
        env: *mut js_env_t,
        array: *mut js_value_t,
        result: *mut u32,
    ) -> c_int;

    pub fn js_get_array_elements(
        env: *mut js_env_t,
        array: *mut js_value_t,
        elements: *mut *mut js_value_t,
        len: usize,
        offset: usize,
        result: *mut u32,
    ) -> c_int;

    pub fn js_set_array_elements(
        env: *mut js_env_t,
        array: *mut js_value_t,
        elements: *mut *const js_value_t,
        len: usize,
        offset: usize,
    ) -> c_int;

    pub fn js_get_prototype(
        env: *mut js_env_t,
        object: *mut js_value_t,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_get_property_names(
        env: *mut js_env_t,
        object: *mut js_value_t,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_get_filtered_property_names(
        env: *mut js_env_t,
        object: *mut js_value_t,
        mode: js_key_collection_mode_t,
        property_filter: js_property_filter_t,
        index_filter: js_index_filter_t,
        key_conversion: js_key_conversion_mode_t,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_get_property(
        env: *mut js_env_t,
        object: *mut js_value_t,
        key: *mut js_value_t,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_has_property(
        env: *mut js_env_t,
        object: *mut js_value_t,
        key: *mut js_value_t,
        result: *mut bool,
    ) -> c_int;

    pub fn js_has_own_property(
        env: *mut js_env_t,
        object: *mut js_value_t,
        key: *mut js_value_t,
        result: *mut bool,
    ) -> c_int;

    pub fn js_set_property(
        env: *mut js_env_t,
        object: *mut js_value_t,
        key: *mut js_value_t,
        value: *mut js_value_t,
    ) -> c_int;

    pub fn js_delete_property(
        env: *mut js_env_t,
        object: *mut js_value_t,
        key: *mut js_value_t,
        result: *mut bool,
    ) -> c_int;

    pub fn js_get_named_property(
        env: *mut js_env_t,
        object: *mut js_value_t,
        name: *const c_char,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_has_named_property(
        env: *mut js_env_t,
        object: *mut js_value_t,
        name: *const c_char,
        result: *mut bool,
    ) -> c_int;

    pub fn js_set_named_property(
        env: *mut js_env_t,
        object: *mut js_value_t,
        name: *const c_char,
        value: *mut js_value_t,
    ) -> c_int;

    pub fn js_delete_named_property(
        env: *mut js_env_t,
        object: *mut js_value_t,
        name: *const c_char,
        result: *mut bool,
    ) -> c_int;

    pub fn js_get_element(
        env: *mut js_env_t,
        object: *mut js_value_t,
        index: u32,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_has_element(
        env: *mut js_env_t,
        object: *mut js_value_t,
        index: u32,
        result: *mut bool,
    ) -> c_int;

    pub fn js_set_element(
        env: *mut js_env_t,
        object: *mut js_value_t,
        index: u32,
        value: *mut js_value_t,
    ) -> c_int;

    pub fn js_delete_element(
        env: *mut js_env_t,
        object: *mut js_value_t,
        index: u32,
        result: *mut bool,
    ) -> c_int;

    pub fn js_get_callback_info(
        env: *mut js_env_t,
        info: *const js_callback_info_t,
        argc: *mut usize,
        argv: *mut *mut js_value_t,
        receiver: *mut *mut js_value_t,
        data: *mut *mut c_void,
    ) -> c_int;

    pub fn js_get_new_target(
        env: *mut js_env_t,
        info: *const js_callback_info_t,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_get_arraybuffer_info(
        env: *mut js_env_t,
        arraybuffer: *mut js_value_t,
        data: *mut *mut c_void,
        len: *mut usize,
    ) -> c_int;

    pub fn js_get_sharedarraybuffer_info(
        env: *mut js_env_t,
        sharedarraybuffer: *mut js_value_t,
        data: *mut *mut c_void,
        len: *mut usize,
    ) -> c_int;

    pub fn js_get_typedarray_info(
        env: *mut js_env_t,
        typedarray: *mut js_value_t,
        type_: *mut js_typedarray_type_t,
        data: *mut *mut c_void,
        len: *mut usize,
        arraybuffer: *mut *mut js_value_t,
        offset: *mut usize,
    ) -> c_int;

    pub fn js_get_dataview_info(
        env: *mut js_env_t,
        dataview: *mut js_value_t,
        data: *mut *mut c_void,
        len: *mut usize,
        arraybuffer: *mut *mut js_value_t,
        offset: *mut usize,
    ) -> c_int;

    pub fn js_call_function(
        env: *mut js_env_t,
        receiver: *mut js_value_t,
        function: *mut js_value_t,
        argc: usize,
        argv: *const *mut js_value_t,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_call_function_with_checkpoint(
        env: *mut js_env_t,
        receiver: *mut js_value_t,
        function: *mut js_value_t,
        argc: usize,
        argv: *const *mut js_value_t,
        result: *mut *mut js_value_t,
    ) -> c_int;

    pub fn js_new_instance(
        env: *mut js_env_t,
        constructor: *mut js_value_t,
        argc: usize,
        argv: *const *mut js_value_t,
        result: *mut *mut js_value_t,
    ) -> c_int;
}
