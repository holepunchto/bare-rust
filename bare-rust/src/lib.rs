use std::ffi::{c_void, CString};
use std::ptr;
use std::result;
use std::slice;
use std::string;

pub use bare_rust_ffi as ffi;

use ffi::*;

macro_rules! check_status {
    ($env:expr, $status:expr) => {
        if $status == JS_PENDING_EXCEPTION {
            return Err($env.pending_exception());
        } else if $status != 0 {
            panic!("Uncaught JavaScript exception");
        }
    };
}

type Result<T> = result::Result<T, Value>;

#[derive(Debug)]
pub struct Env {
    ptr: *mut js_env_t,
}

impl Env {
    pub fn is_exception_pending(&self) -> bool {
        let mut result = false;

        unsafe {
            js_is_exception_pending(self.ptr, &mut result);
        }

        result
    }

    pub fn pending_exception(&self) -> Value {
        let mut ptr: *mut js_value_t = ptr::null_mut();

        unsafe {
            js_get_and_clear_last_exception(self.ptr, &mut ptr);
        }

        Value { env: self.ptr, ptr }
    }
}

impl From<*mut js_env_t> for Env {
    fn from(ptr: *mut js_env_t) -> Self {
        return Self { ptr };
    }
}

#[derive(Debug)]
pub struct Scope {
    env: *mut js_env_t,
    ptr: *mut js_handle_scope_t,
}

impl Scope {
    pub fn new(env: &Env) -> Self {
        let mut ptr: *mut js_handle_scope_t = ptr::null_mut();

        unsafe {
            js_open_handle_scope(env.ptr, &mut ptr);
        }

        Self { env: env.ptr, ptr }
    }
}

impl Drop for Scope {
    fn drop(&mut self) {
        unsafe {
            js_close_handle_scope(self.env, self.ptr);
        }
    }
}

#[derive(Debug)]
pub struct EscapableScope {
    env: *mut js_env_t,
    ptr: *mut js_escapable_handle_scope_t,
}

impl EscapableScope {
    pub fn new(env: &Env) -> Self {
        let mut ptr: *mut js_escapable_handle_scope_t = ptr::null_mut();

        unsafe {
            js_open_escapable_handle_scope(env.ptr, &mut ptr);
        }

        Self { env: env.ptr, ptr }
    }

    pub fn escape<T>(self, escapee: T) -> Value
    where
        T: Into<*mut js_value_t>,
    {
        let mut ptr: *mut js_value_t = ptr::null_mut();

        unsafe {
            js_escape_handle(self.env, self.ptr, escapee.into(), &mut ptr);
        }

        Value { env: self.env, ptr }
    }
}

impl Drop for EscapableScope {
    fn drop(&mut self) {
        unsafe {
            js_close_escapable_handle_scope(self.env, self.ptr);
        }
    }
}

#[derive(Debug)]
pub struct Value {
    env: *mut js_env_t,
    ptr: *mut js_value_t,
}

impl From<Value> for *mut js_value_t {
    fn from(value: Value) -> Self {
        value.ptr
    }
}

macro_rules! value_conversions {
    ($type: ident) => {
        impl From<$type> for *mut js_value_t {
            fn from(value: $type) -> Self {
                value.0.ptr
            }
        }

        impl From<$type> for Value {
            fn from(value: $type) -> Self {
                value.0
            }
        }

        impl From<Value> for $type {
            fn from(value: Value) -> Self {
                Self(value)
            }
        }
    };
}

#[derive(Debug)]
pub struct Undefined(Value);

impl Undefined {
    pub fn new(env: &Env) -> Self {
        let mut ptr: *mut js_value_t = ptr::null_mut();

        unsafe {
            js_get_undefined(env.ptr, &mut ptr);
        }

        Self(Value { env: env.ptr, ptr })
    }
}

value_conversions!(Undefined);

#[derive(Debug)]
pub struct Null(Value);

impl Null {
    pub fn new(env: &Env) -> Self {
        let mut ptr: *mut js_value_t = ptr::null_mut();

        unsafe {
            js_get_null(env.ptr, &mut ptr);
        }

        Self(Value { env: env.ptr, ptr })
    }
}

value_conversions!(Null);

#[derive(Debug)]
pub struct Boolean(Value);

impl Boolean {
    pub fn new(env: &Env, value: bool) -> Self {
        let mut ptr: *mut js_value_t = ptr::null_mut();

        unsafe {
            js_get_boolean(env.ptr, value, &mut ptr);
        }

        Self(Value { env: env.ptr, ptr })
    }
}

value_conversions!(Boolean);

impl From<Boolean> for bool {
    fn from(boolean: Boolean) -> Self {
        let mut value = false;

        unsafe {
            js_get_value_bool(boolean.0.env, boolean.0.ptr, &mut value);
        }

        value
    }
}

#[derive(Debug)]
pub struct Number(Value);

impl Number {
    pub fn with_i32(env: &Env, value: i32) -> Self {
        let mut ptr: *mut js_value_t = ptr::null_mut();

        unsafe {
            js_create_int32(env.ptr, value, &mut ptr);
        }

        Self(Value { env: env.ptr, ptr })
    }

    pub fn with_u32(env: &Env, value: u32) -> Self {
        let mut ptr: *mut js_value_t = ptr::null_mut();

        unsafe {
            js_create_uint32(env.ptr, value, &mut ptr);
        }

        Self(Value { env: env.ptr, ptr })
    }

    pub fn with_i64(env: &Env, value: i64) -> Self {
        let mut ptr: *mut js_value_t = ptr::null_mut();

        unsafe {
            js_create_int64(env.ptr, value, &mut ptr);
        }

        Self(Value { env: env.ptr, ptr })
    }

    pub fn with_f64(env: &Env, value: f64) -> Self {
        let mut ptr: *mut js_value_t = ptr::null_mut();

        unsafe {
            js_create_double(env.ptr, value, &mut ptr);
        }

        Self(Value { env: env.ptr, ptr })
    }
}

value_conversions!(Number);

impl From<Number> for i32 {
    fn from(number: Number) -> Self {
        let mut value = 0;

        unsafe {
            js_get_value_int32(number.0.env, number.0.ptr, &mut value);
        }

        value
    }
}

impl From<Number> for u32 {
    fn from(number: Number) -> Self {
        let mut value = 0;

        unsafe {
            js_get_value_uint32(number.0.env, number.0.ptr, &mut value);
        }

        value
    }
}

impl From<Number> for i64 {
    fn from(number: Number) -> Self {
        let mut value = 0;

        unsafe {
            js_get_value_int64(number.0.env, number.0.ptr, &mut value);
        }

        value
    }
}

impl From<Number> for f64 {
    fn from(number: Number) -> Self {
        let mut value = 0.0;

        unsafe {
            js_get_value_double(number.0.env, number.0.ptr, &mut value);
        }

        value
    }
}

#[derive(Debug)]
pub struct BigInt(Value);

impl BigInt {
    pub fn with_i64(env: &Env, value: i64) -> Self {
        let mut ptr: *mut js_value_t = ptr::null_mut();

        unsafe {
            js_create_bigint_int64(env.ptr, value, &mut ptr);
        }

        Self(Value { env: env.ptr, ptr })
    }

    pub fn with_u64(env: &Env, value: u64) -> Self {
        let mut ptr: *mut js_value_t = ptr::null_mut();

        unsafe {
            js_create_bigint_uint64(env.ptr, value, &mut ptr);
        }

        Self(Value { env: env.ptr, ptr })
    }
}

value_conversions!(BigInt);

impl From<BigInt> for i64 {
    fn from(bigint: BigInt) -> Self {
        let mut value = 0;

        unsafe {
            js_get_value_bigint_int64(bigint.0.env, bigint.0.ptr, &mut value, ptr::null_mut());
        }

        value
    }
}

impl From<BigInt> for u64 {
    fn from(bigint: BigInt) -> Self {
        let mut value = 0;

        unsafe {
            js_get_value_bigint_uint64(bigint.0.env, bigint.0.ptr, &mut value, ptr::null_mut());
        }

        value
    }
}

#[derive(Debug)]
pub struct Name(Value);

value_conversions!(Name);

impl From<String> for Name {
    fn from(string: String) -> Self {
        Name(string.0)
    }
}

impl From<Symbol> for Name {
    fn from(symbol: Symbol) -> Self {
        Name(symbol.0)
    }
}

#[derive(Debug)]
pub struct Symbol(Value);

impl Symbol {
    pub fn new(env: &Env, description: &str) -> Result<Self> {
        let description = String::new(env, description)?;

        let mut ptr: *mut js_value_t = ptr::null_mut();

        let status = unsafe { js_create_symbol(env.ptr, description.0.ptr, &mut ptr) };

        check_status!(env, status);

        Ok(Self(Value { env: env.ptr, ptr }))
    }
}

value_conversions!(Symbol);

#[derive(Debug)]
pub struct String(Value);

impl String {
    pub fn new(env: &Env, value: &str) -> Result<Self> {
        let mut ptr: *mut js_value_t = ptr::null_mut();

        let status = unsafe {
            js_create_string_utf8(
                env.ptr,
                value.as_ptr().cast(),
                value.len() as usize,
                &mut ptr,
            )
        };

        check_status!(env, status);

        Ok(Self(Value { env: env.ptr, ptr }))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut len = 0;

        unsafe {
            js_get_value_string_utf8(self.0.env, self.0.ptr, ptr::null_mut(), 0, &mut len);
        }

        let mut result = Vec::new();

        result.resize(len, 0);

        unsafe {
            js_get_value_string_utf8(self.0.env, self.0.ptr, result.as_mut_ptr(), len, &mut len);
        }

        result
    }
}

value_conversions!(String);

impl From<String> for string::String {
    fn from(string: String) -> Self {
        return string::String::from_utf8(string.to_bytes()).unwrap();
    }
}

#[derive(Debug)]
pub struct Object(Value);

impl Object {
    pub fn new(env: &Env) -> Result<Self> {
        let mut ptr: *mut js_value_t = ptr::null_mut();

        let status = unsafe { js_create_object(env.ptr, &mut ptr) };

        check_status!(env, status);

        Ok(Self(Value { env: env.ptr, ptr }))
    }

    pub fn get_property<N, T>(&self, name: N) -> Result<T>
    where
        N: Into<Name>,
        T: From<Value>,
    {
        let env = Env::from(self.0.env);

        let mut ptr: *mut js_value_t = ptr::null_mut();

        let status =
            unsafe { js_get_property(self.0.env, self.0.ptr, name.into().0.ptr, &mut ptr) };

        check_status!(env, status);

        Ok(Value { env: env.ptr, ptr }.into())
    }

    pub fn get_named_property<T>(&self, name: &str) -> Result<T>
    where
        T: From<Value>,
    {
        let env = Env::from(self.0.env);

        let key = CString::new(name).unwrap();

        let mut ptr: *mut js_value_t = ptr::null_mut();

        let status =
            unsafe { js_get_named_property(self.0.env, self.0.ptr, key.as_ptr(), &mut ptr) };

        check_status!(env, status);

        Ok(Value { env: env.ptr, ptr }.into())
    }

    pub fn get_element<T>(&self, index: u32) -> Result<T>
    where
        T: From<Value>,
    {
        let env = Env::from(self.0.env);

        let mut ptr: *mut js_value_t = ptr::null_mut();

        let status = unsafe { js_get_element(self.0.env, self.0.ptr, index, &mut ptr) };

        check_status!(env, status);

        Ok(Value { env: env.ptr, ptr }.into())
    }

    pub fn has_property<N>(&self, name: N) -> Result<bool>
    where
        N: Into<Name>,
    {
        let env = Env::from(self.0.env);

        let mut result = false;

        let status =
            unsafe { js_has_property(self.0.env, self.0.ptr, name.into().0.ptr, &mut result) };

        check_status!(env, status);

        Ok(result)
    }

    pub fn has_own_property<N>(&self, name: N) -> Result<bool>
    where
        N: Into<Name>,
    {
        let env = Env::from(self.0.env);

        let mut result = false;

        let status =
            unsafe { js_has_own_property(self.0.env, self.0.ptr, name.into().0.ptr, &mut result) };

        check_status!(env, status);

        Ok(result)
    }

    pub fn has_named_property(&self, name: &str) -> Result<bool> {
        let env = Env::from(self.0.env);

        let key = CString::new(name).unwrap();

        let mut result = false;

        let status =
            unsafe { js_has_named_property(self.0.env, self.0.ptr, key.as_ptr(), &mut result) };

        check_status!(env, status);

        Ok(result)
    }

    pub fn has_element(&self, index: u32) -> Result<bool> {
        let env = Env::from(self.0.env);

        let mut result = false;

        let status = unsafe { js_has_element(self.0.env, self.0.ptr, index, &mut result) };

        check_status!(env, status);

        Ok(result)
    }

    pub fn set_property<N, T>(&mut self, name: N, value: T) -> Result<()>
    where
        N: Into<Name>,
        T: Into<*mut js_value_t>,
    {
        let env = Env::from(self.0.env);

        let status =
            unsafe { js_set_property(self.0.env, self.0.ptr, name.into().0.ptr, value.into()) };

        check_status!(env, status);

        Ok(())
    }

    pub fn set_named_property<T>(&mut self, name: &str, value: T) -> Result<()>
    where
        T: Into<*mut js_value_t>,
    {
        let env = Env::from(self.0.env);

        let key = CString::new(name).unwrap();

        let status =
            unsafe { js_set_named_property(self.0.env, self.0.ptr, key.as_ptr(), value.into()) };

        check_status!(env, status);

        Ok(())
    }

    pub fn set_element<T>(&mut self, index: u32, value: T) -> Result<()>
    where
        T: Into<*mut js_value_t>,
    {
        let env = Env::from(self.0.env);

        let status = unsafe { js_set_element(self.0.env, self.0.ptr, index, value.into()) };

        check_status!(env, status);

        Ok(())
    }

    pub fn delete_property<N>(&self, name: N) -> Result<bool>
    where
        N: Into<Name>,
    {
        let env = Env::from(self.0.env);

        let mut result = false;

        let status =
            unsafe { js_delete_property(self.0.env, self.0.ptr, name.into().0.ptr, &mut result) };

        check_status!(env, status);

        Ok(result)
    }

    pub fn delete_named_property(&self, name: &str) -> Result<bool> {
        let env = Env::from(self.0.env);

        let key = CString::new(name).unwrap();

        let mut result = false;

        let status =
            unsafe { js_delete_named_property(self.0.env, self.0.ptr, key.as_ptr(), &mut result) };

        check_status!(env, status);

        Ok(result)
    }

    pub fn delete_element(&self, index: u32) -> Result<bool> {
        let env = Env::from(self.0.env);

        let mut result = false;

        let status = unsafe { js_delete_element(self.0.env, self.0.ptr, index, &mut result) };

        check_status!(env, status);

        Ok(result)
    }
}

value_conversions!(Object);

#[derive(Debug)]
pub struct Array(Value);

impl Array {
    pub fn new(env: &Env, len: usize) -> Result<Self> {
        let mut ptr: *mut js_value_t = ptr::null_mut();

        let status = unsafe { js_create_array_with_length(env.ptr, len, &mut ptr) };

        check_status!(env, status);

        Ok(Self(Value { env: env.ptr, ptr }))
    }

    pub fn len(&self) -> u32 {
        let mut len = 0;

        unsafe {
            js_get_array_length(self.0.env, self.0.ptr, &mut len);
        }

        len
    }

    pub fn get<T>(&self, index: u32) -> Result<T>
    where
        T: From<Value>,
    {
        let env = Env::from(self.0.env);

        let mut ptr: *mut js_value_t = ptr::null_mut();

        let status = unsafe { js_get_element(self.0.env, self.0.ptr, index, &mut ptr) };

        check_status!(env, status);

        Ok(Value { env: env.ptr, ptr }.into())
    }

    pub fn set<T>(&mut self, index: u32, value: T) -> Result<()>
    where
        T: Into<*mut js_value_t>,
    {
        let env = Env::from(self.0.env);

        let status = unsafe { js_set_element(self.0.env, self.0.ptr, index, value.into()) };

        check_status!(env, status);

        Ok(())
    }
}

value_conversions!(Array);

#[derive(Debug)]
pub struct Callback {
    env: *mut js_env_t,
    args: Vec<*mut js_value_t>,
    receiver: *mut js_value_t,
}

impl Callback {
    pub fn arg<T>(&self, i: usize) -> Option<T>
    where
        T: From<Value>,
    {
        if i < self.args.len() {
            Some(
                Value {
                    env: self.env,
                    ptr: self.args[i],
                }
                .into(),
            )
        } else {
            None
        }
    }

    pub fn receiver<T>(&self) -> T
    where
        T: From<Value>,
    {
        Value {
            env: self.env,
            ptr: self.receiver,
        }
        .into()
    }
}

#[derive(Debug)]
pub struct Function(Value);

impl Function {
    pub fn new<F>(env: &Env, function: F) -> Result<Self>
    where
        F: FnMut(&Env, &Callback) -> Result<Value>,
    {
        let mut function = function;

        let closure: Box<dyn FnMut(&Env, &Callback) -> *mut js_value_t> =
            Box::new(move |env, info| match function(env, info) {
                Ok(result) => result.into(),
                Err(error) => {
                    unsafe {
                        js_throw(env.ptr, error.into());
                    }

                    ptr::null_mut()
                }
            });

        let data = Box::into_raw(Box::new(closure)) as *mut _;

        let mut ptr: *mut js_value_t = ptr::null_mut();

        let status = unsafe {
            js_create_function(
                env.ptr,
                ptr::null_mut(),
                0,
                Some(Function::call),
                data,
                &mut ptr,
            )
        };

        check_status!(env, status);

        unsafe {
            js_add_finalizer(
                env.ptr,
                ptr,
                data,
                Some(Function::drop),
                ptr::null_mut(),
                ptr::null_mut(),
            );
        }

        Ok(Self(Value { env: env.ptr, ptr }))
    }

    extern "C" fn call(env: *mut js_env_t, info: *mut js_callback_info_t) -> *mut js_value_t {
        let mut len: usize = 0;
        let mut receiver: *mut js_value_t = ptr::null_mut();
        let mut data: *mut c_void = ptr::null_mut();

        unsafe {
            js_get_callback_info(
                env,
                info,
                &mut len,
                ptr::null_mut(),
                &mut receiver,
                &mut data,
            );
        }

        let mut args = Vec::new();

        args.resize(len, ptr::null_mut());

        if len > 0 {
            unsafe {
                js_get_callback_info(
                    env,
                    info,
                    &mut len,
                    args.as_mut_ptr(),
                    ptr::null_mut(),
                    ptr::null_mut(),
                );
            }
        }

        let closure =
            unsafe { &mut *(data as *mut Box<dyn FnMut(&Env, &Callback) -> *mut js_value_t>) };

        return closure(
            &Env::from(env),
            &Callback {
                env,
                args,
                receiver,
            },
        );
    }

    extern "C" fn drop(_: *mut js_env_t, data: *mut c_void, _: *mut c_void) -> () {
        unsafe {
            drop(Box::from_raw(data));
        }
    }
}

value_conversions!(Function);

#[derive(Debug)]
pub struct ArrayBuffer(Value);

impl ArrayBuffer {
    pub fn new(env: &Env, len: usize) -> Result<Self> {
        let mut ptr: *mut js_value_t = ptr::null_mut();

        let status = unsafe { js_create_arraybuffer(env.ptr, len, ptr::null_mut(), &mut ptr) };

        check_status!(env, status);

        Ok(Self(Value { env: env.ptr, ptr }))
    }

    pub fn as_slice(&self) -> &[u8] {
        self.as_mut_slice()
    }

    pub fn as_mut_slice(&self) -> &mut [u8] {
        let mut len: usize = 0;
        let mut data: *mut c_void = ptr::null_mut();

        unsafe {
            js_get_arraybuffer_info(self.0.env, self.0.ptr, &mut data, &mut len);
        }

        unsafe { slice::from_raw_parts_mut(data as *mut u8, len) }
    }
}

value_conversions!(ArrayBuffer);

pub trait TypedArray<T> {
    fn as_slice(&self) -> &[T];
    fn as_mut_slice(&self) -> &mut [T];
}

macro_rules! define_typedarray {
    ($name:ident, $type:ident, $kind:ident) => {
        #[derive(Debug)]
        pub struct $name(Value);

        impl $name {
            pub fn new(env: &Env, len: usize) -> Result<Self> {
                let arraybuffer = ArrayBuffer::new(env, len * size_of::<$type>())?;

                let mut ptr: *mut js_value_t = ptr::null_mut();

                let status = unsafe {
                    js_create_typedarray(
                        env.ptr,
                        js_typedarray_type_t::$kind,
                        len,
                        arraybuffer.0.ptr,
                        0,
                        &mut ptr,
                    )
                };

                check_status!(env, status);

                Ok(Self(Value { env: env.ptr, ptr }))
            }
        }

        impl TypedArray<$type> for $name {
            fn as_slice(&self) -> &[$type] {
                self.as_mut_slice()
            }

            fn as_mut_slice(&self) -> &mut [$type] {
                let mut len: usize = 0;
                let mut data: *mut c_void = ptr::null_mut();

                unsafe {
                    js_get_typedarray_info(
                        self.0.env,
                        self.0.ptr,
                        ptr::null_mut(),
                        &mut data,
                        &mut len,
                        ptr::null_mut(),
                        ptr::null_mut(),
                    );
                }

                unsafe { slice::from_raw_parts_mut(data as *mut $type, len) }
            }
        }

        value_conversions!($name);
    };
}

define_typedarray!(Int8Array, i8, js_int8array);
define_typedarray!(Uint8Array, u8, js_uint8array);
define_typedarray!(Uint8ClampedArray, u8, js_uint8clampedarray);
define_typedarray!(Int16Array, i16, js_int16array);
define_typedarray!(Uint16Array, u16, js_uint16array);
define_typedarray!(Int32Array, i32, js_int32array);
define_typedarray!(Uint32Array, u32, js_uint32array);
define_typedarray!(Float32Array, f32, js_float32array);
define_typedarray!(Float64Array, f64, js_float64array);
define_typedarray!(BigInt64Array, i32, js_bigint64array);
define_typedarray!(BigUint64Array, u32, js_biguint64array);

macro_rules! define_error {
    ($name:ident, $create:ident) => {
        #[derive(Debug)]
        pub struct $name(Value);

        impl $name {
            pub fn new(env: &Env, message: &str) -> Self {
                let message = String::new(env, message).unwrap();

                let mut ptr: *mut js_value_t = std::ptr::null_mut();

                unsafe {
                    $create(env.ptr, std::ptr::null_mut(), message.0.ptr, &mut ptr);
                }

                Self(Value { env: env.ptr, ptr })
            }
        }

        value_conversions!($name);
    };
}

define_error!(Error, js_create_error);
define_error!(TypeError, js_create_type_error);
define_error!(RangeError, js_create_range_error);
define_error!(SyntaxError, js_create_syntax_error);
define_error!(ReferenceError, js_create_reference_error);
