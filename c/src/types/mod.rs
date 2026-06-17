#[allow(dead_code)]
mod array;
mod cow;
mod datetime;
mod decimal;
mod language;
mod option;
mod string;

pub(crate) use cow::CCow;
pub(crate) use decimal::CDecimal;
pub(crate) use language::CLanguage;
pub(crate) use string::CString;

pub(crate) trait ToFFI {
    type FFIType;

    fn to_ffi_type(&self) -> Self::FFIType;
}

impl ToFFI for f64 {
    type FFIType = f64;

    #[inline]
    fn to_ffi_type(&self) -> Self::FFIType {
        *self
    }
}

impl ToFFI for i64 {
    type FFIType = i64;

    #[inline]
    fn to_ffi_type(&self) -> Self::FFIType {
        *self
    }
}

impl<T> ToFFI for *const *const T {
    type FFIType = *const T;

    #[inline]
    fn to_ffi_type(&self) -> Self::FFIType {
        if self.is_null() {
            std::ptr::null()
        } else {
            unsafe { *(*self) }
        }
    }
}
