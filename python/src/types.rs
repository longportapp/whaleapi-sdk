use longportwhale_python_macros::PyEnum;
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[allow(non_camel_case_types)]
#[py(remote = "longportwhale::Language")]
pub(crate) enum Language {
    /// zh-CN
    ZH_CN,
    /// zh-HK
    ZH_HK,
    /// en
    EN,
}
