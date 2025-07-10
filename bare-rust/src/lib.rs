use std::ffi::{c_void, CString};
use std::ptr;
use std::slice;
use std::string;

pub use bare_rust_ffi as ffi;

use ffi::*;

macro_rules! check_status {
    ($env:expr, $status:expr) => {
        if $status == JS_PENDING_EXCEPTION {
            return Err($env.pending_exception().unwrap());
        } else if $status != 0 {
            panic!("Uncaught JavaScript exception");
        }
    };
}

pub type Result<T> = std::result::Result<T, Value>;

#[derive(Debug)]
pub struct Env {
    ptr: *mut js_env_t,
}

impl Env {
    pub fn pending_exception(&self) -> Option<Value> {
        let mut ptr: *mut js_value_t = ptr::null_mut();

        unsafe {
            js_get_and_clear_last_exception(self.ptr, &mut ptr);
        }

        if ptr.is_null() {
            None
        } else {
            Some(Value { env: self.ptr, ptr })
        }
    }
}

impl From<*mut js_env_t> for Env {
    fn from(ptr: *mut js_env_t) -> Self {
        return Self { ptr };
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

impl From<Undefined> for *mut js_value_t {
    fn from(undefined: Undefined) -> Self {
        undefined.0.ptr
    }
}

impl From<Value> for Undefined {
    fn from(value: Value) -> Self {
        Self(value)
    }
}

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

impl From<Null> for *mut js_value_t {
    fn from(null: Null) -> Self {
        null.0.ptr
    }
}

impl From<Value> for Null {
    fn from(value: Value) -> Self {
        Self(value)
    }
}

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

impl From<Boolean> for bool {
    fn from(boolean: Boolean) -> Self {
        let mut value = false;

        unsafe {
            js_get_value_bool(boolean.0.env, boolean.0.ptr, &mut value);
        }

        value
    }
}

impl From<Boolean> for *mut js_value_t {
    fn from(boolean: Boolean) -> Self {
        boolean.0.ptr
    }
}

impl From<Value> for Boolean {
    fn from(value: Value) -> Self {
        Self(value)
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

impl From<Number> for *mut js_value_t {
    fn from(number: Number) -> Self {
        number.0.ptr
    }
}

impl From<Value> for Number {
    fn from(value: Value) -> Self {
        Self(value)
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

impl From<BigInt> for *mut js_value_t {
    fn from(bigint: BigInt) -> Self {
        bigint.0.ptr
    }
}

impl From<Value> for BigInt {
    fn from(value: Value) -> Self {
        Self(value)
    }
}

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

impl From<String> for string::String {
    fn from(string: String) -> Self {
        return string::String::from_utf8(string.to_bytes()).unwrap();
    }
}

impl From<String> for *mut js_value_t {
    fn from(string: String) -> Self {
        string.0.ptr
    }
}

impl From<Value> for String {
    fn from(value: Value) -> Self {
        Self(value)
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

        Ok(T::from(Value { env: env.ptr, ptr }))
    }

    pub fn has_named_property<T>(&self, name: &str) -> Result<bool> {
        let env = Env::from(self.0.env);

        let key = CString::new(name).unwrap();

        let mut result = false;

        let status =
            unsafe { js_has_named_property(self.0.env, self.0.ptr, key.as_ptr(), &mut result) };

        check_status!(env, status);

        Ok(result)
    }

    pub fn set_named_property<T>(&mut self, name: &str, value: T) -> Result<()>
    where
        T: Into<*mut js_value_t>,
    {
        let env = Env::from(self.0.env);

        let key = CString::new(name).unwrap();

        let status =
            unsafe { js_set_named_property(self.0.env, self.0.ptr, key.as_ptr(), T::into(value)) };

        check_status!(env, status);

        Ok(())
    }
}

impl From<Object> for *mut js_value_t {
    fn from(object: Object) -> Self {
        object.0.ptr
    }
}

impl From<Value> for Object {
    fn from(value: Value) -> Self {
        Self(value)
    }
}

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
            Some(T::from(Value {
                env: self.env,
                ptr: self.args[i],
            }))
        } else {
            None
        }
    }

    pub fn receiver<T>(&self) -> T
    where
        T: From<Value>,
    {
        T::from(Value {
            env: self.env,
            ptr: self.receiver,
        })
    }
}

#[derive(Debug)]
pub struct Function(Value);

impl Function {
    pub fn new<F, R>(env: &Env, function: F) -> Result<Self>
    where
        F: FnMut(&Env, &Callback) -> Result<R>,
        R: Into<*mut js_value_t>,
    {
        let mut function = function;

        let closure: Box<dyn FnMut(&Env, &Callback) -> *mut js_value_t> =
            Box::new(move |env, info| match function(env, info) {
                Ok(result) => result.into(),
                Err(error) => {
                    unsafe {
                        js_throw(env.ptr, error.ptr);
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

        let closure: &mut Box<dyn FnMut(&Env, &Callback) -> *mut js_value_t> =
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
        let _: Box<Box<dyn FnMut(&Env, &Callback)>> = unsafe { Box::from_raw(data as *mut _) };
    }
}

impl From<Function> for *mut js_value_t {
    fn from(function: Function) -> Self {
        function.0.ptr
    }
}

impl From<Value> for Function {
    fn from(value: Value) -> Self {
        Self(value)
    }
}

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

impl From<ArrayBuffer> for *mut js_value_t {
    fn from(arraybuffer: ArrayBuffer) -> Self {
        arraybuffer.0.ptr
    }
}

impl From<Value> for ArrayBuffer {
    fn from(value: Value) -> Self {
        Self(value)
    }
}

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

        impl From<$name> for *mut js_value_t {
            fn from(typedarray: $name) -> Self {
                typedarray.0.ptr
            }
        }

        impl From<Value> for $name {
            fn from(value: Value) -> Self {
                Self(value)
            }
        }
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

        impl From<$name> for *mut js_value_t {
            fn from(error: $name) -> Self {
                error.0.ptr
            }
        }

        impl From<Value> for $name {
            fn from(value: Value) -> Self {
                Self(value)
            }
        }
    };
}

define_error!(Error, js_create_error);
define_error!(TypeError, js_create_type_error);
define_error!(RangeError, js_create_range_error);
define_error!(SyntaxError, js_create_syntax_error);
define_error!(ReferenceError, js_create_reference_error);
