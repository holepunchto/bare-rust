use std::ffi::{c_void, CString};
use std::ptr;
use std::slice;

pub use bare_rust_ffi as ffi;

use ffi::*;

pub type Result<T> = std::result::Result<T, i32>;

pub struct Env {
    ptr: *mut js_env_t,
}

impl From<*mut js_env_t> for Env {
    fn from(ptr: *mut js_env_t) -> Self {
        return Self { ptr };
    }
}

pub struct Value {
    env: *mut js_env_t,
    ptr: *mut js_value_t,
}

impl From<Value> for *mut js_value_t {
    fn from(value: Value) -> Self {
        value.ptr
    }
}

pub struct Undefined(pub Value);

impl Undefined {
    pub fn new(env: &Env) -> Result<Self> {
        let mut ptr: *mut js_value_t = ptr::null_mut();

        let status = unsafe { js_get_undefined(env.ptr, &mut ptr) };

        if status != 0 {
            Err(status)
        } else {
            Ok(Self(Value { env: env.ptr, ptr }))
        }
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

pub struct Null(pub Value);

impl Null {
    pub fn new(env: &Env) -> Result<Self> {
        let mut ptr: *mut js_value_t = ptr::null_mut();

        let status = unsafe { js_get_null(env.ptr, &mut ptr) };

        if status != 0 {
            Err(status)
        } else {
            Ok(Self(Value { env: env.ptr, ptr }))
        }
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

pub struct Boolean(pub Value);

impl Boolean {
    pub fn new(env: &Env, value: bool) -> Result<Self> {
        let mut ptr: *mut js_value_t = ptr::null_mut();

        let status = unsafe { js_get_boolean(env.ptr, value, &mut ptr) };

        if status != 0 {
            Err(status)
        } else {
            Ok(Self(Value { env: env.ptr, ptr }))
        }
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

pub struct String(pub Value);

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

        if status != 0 {
            Err(status)
        } else {
            Ok(Self(Value { env: env.ptr, ptr }))
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut len = 0;

        unsafe {
            js_get_value_string_utf8(self.0.env, self.0.ptr, ptr::null_mut(), 0, &mut len);
        }

        len += 1;

        let mut result = Vec::new();

        result.resize(len, 0);

        unsafe {
            js_get_value_string_utf8(self.0.env, self.0.ptr, result.as_mut_ptr(), len, &mut len);
        }

        result
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

pub struct Object(pub Value);

impl Object {
    pub fn new(env: &Env) -> Result<Self> {
        let mut ptr: *mut js_value_t = ptr::null_mut();

        let status = unsafe { js_create_object(env.ptr, &mut ptr) };

        if status != 0 {
            Err(status)
        } else {
            Ok(Self(Value { env: env.ptr, ptr }))
        }
    }

    pub fn get_named_property<T>(&self, name: &str) -> Result<T>
    where
        T: From<Value>,
    {
        let key = CString::new(name).unwrap();

        let mut ptr: *mut js_value_t = ptr::null_mut();

        let status =
            unsafe { js_get_named_property(self.0.env, self.0.ptr, key.as_ptr(), &mut ptr) };

        if status != 0 {
            Err(status)
        } else {
            Ok(T::from(Value {
                env: self.0.env,
                ptr,
            }))
        }
    }

    pub fn has_named_property<T>(&self, name: &str) -> Result<bool> {
        let key = CString::new(name).unwrap();

        let mut result = false;

        let status =
            unsafe { js_has_named_property(self.0.env, self.0.ptr, key.as_ptr(), &mut result) };

        if status != 0 {
            Err(status)
        } else {
            Ok(result)
        }
    }

    pub fn set_named_property<T>(&mut self, name: &str, value: T) -> Result<()>
    where
        T: Into<*mut js_value_t>,
    {
        let key = CString::new(name).unwrap();

        let status =
            unsafe { js_set_named_property(self.0.env, self.0.ptr, key.as_ptr(), T::into(value)) };

        if status != 0 {
            Err(status)
        } else {
            Ok(())
        }
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

pub struct Function(Value);

impl Function {
    pub fn new<F, R>(env: &Env, function: F) -> Result<Self>
    where
        F: FnMut(&Env, &Callback) -> R,
        R: Into<*mut js_value_t>,
    {
        let mut function = function;

        let closure: Box<dyn FnMut(&Env, &Callback) -> *mut js_value_t> =
            Box::new(move |env, info| function(env, info).into());

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

        if status != 0 {
            return Err(status);
        }

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

pub struct ArrayBuffer(Value);

impl ArrayBuffer {
    pub fn new(env: &Env, len: usize) -> Result<Self> {
        let mut ptr: *mut js_value_t = ptr::null_mut();

        let status = unsafe { js_create_arraybuffer(env.ptr, len, ptr::null_mut(), &mut ptr) };

        if status != 0 {
            Err(status)
        } else {
            Ok(Self(Value { env: env.ptr, ptr }))
        }
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

pub struct Uint8Array(Value);

impl Uint8Array {
    pub fn new(env: &Env, len: usize) -> Result<Self> {
        let arraybuffer = ArrayBuffer::new(env, len)?;

        let mut ptr: *mut js_value_t = ptr::null_mut();

        let status = unsafe {
            js_create_typedarray(
                env.ptr,
                js_typedarray_type_t::js_uint8array,
                len,
                arraybuffer.0.ptr,
                0,
                &mut ptr,
            )
        };

        if status != 0 {
            Err(status)
        } else {
            Ok(Self(Value { env: env.ptr, ptr }))
        }
    }
}

impl TypedArray<u8> for Uint8Array {
    fn as_slice(&self) -> &[u8] {
        self.as_mut_slice()
    }

    fn as_mut_slice(&self) -> &mut [u8] {
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

        unsafe { slice::from_raw_parts_mut(data as *mut u8, len) }
    }
}

impl From<Uint8Array> for *mut js_value_t {
    fn from(uint8array: Uint8Array) -> Self {
        uint8array.0.ptr
    }
}

impl From<Value> for Uint8Array {
    fn from(value: Value) -> Self {
        Self(value)
    }
}
