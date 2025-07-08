use std::ffi::CString;
use std::ptr;

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

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut len = 0;

        let status = unsafe {
            js_get_value_string_utf8(self.0.env, self.0.ptr, ptr::null_mut(), 0, &mut len)
        };

        if status != 0 {
            return Err(status);
        }

        len += 1;

        let mut result = Vec::with_capacity(len);

        let status = unsafe {
            js_get_value_string_utf8(self.0.env, self.0.ptr, result.as_mut_ptr(), len, &mut len)
        };

        if status != 0 {
            Err(status)
        } else {
            Ok(result)
        }
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
