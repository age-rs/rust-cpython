#[allow(unused_imports)]
use libc::{c_char, c_short, c_int, c_uchar, c_void};

use crate::object::*;
use crate::pyport::Py_ssize_t;

#[cfg(Py_3_8)]
#[repr(C)]
pub struct _PyOpcache {
    _private: [u8; 0],
}

#[derive(Copy)]
#[repr(C)]
#[cfg(Py_3_11)]
// The field orderings have completely changed in 3.11,
// so we seperate it out into a different type declaration
// 
// the justification for the reordering was "optimization"
pub struct PyCodeObject {
    pub ob_base: PyVarObject,
    pub co_consts: *mut PyObject,
    pub co_names: *mut PyObject,
    pub co_exceptiontable: *mut PyObject,
    pub co_flags: c_int,
    #[cfg(not(Py_3_12))]
    pub co_warmup: c_short,
    pub co_argcount: c_int,
    pub co_posonlyargcount: c_int,
    pub co_kwonlyargcount: c_int,
    pub co_stacksize: c_int,
    pub co_firstlineno: c_int,
    pub co_nlocalsplus: c_int,
    #[cfg(Py_3_12)]
    pub co_framesize: c_int,
    pub co_nlocals: c_int,
    #[cfg(not(Py_3_12))]
    pub co_nplaincellvars: c_int,
    pub co_ncellvars: c_int,
    pub co_nfreevars: c_int,
    #[cfg(Py_3_12)]
    pub co_version: u32,
    pub co_localsplusnames: *mut PyObject,
    pub co_localspluskinds: *mut PyObject,
    pub co_filename: *mut PyObject,
    pub co_name: *mut PyObject,
    pub co_qualname: *mut PyObject,
    pub co_linetable: *mut PyObject,
    pub co_weakreflist: *mut PyObject,
    // Intently omitting some internal fields at the end of this structure
}

#[repr(C)]
#[derive(Copy)]
#[cfg(not(Py_3_11))] 
pub struct PyCodeObject {
    pub ob_base: PyObject,
    pub co_argcount: c_int,
    #[cfg(Py_3_8)]
    pub co_posonlyargcount: c_int,
    pub co_kwonlyargcount: c_int,
    pub co_nlocals: c_int,
    pub co_stacksize: c_int,
    pub co_flags: c_int,
    #[cfg(Py_3_6)]
    pub co_firstlineno: c_int,
    pub co_code: *mut PyObject,
    pub co_consts: *mut PyObject,
    pub co_names: *mut PyObject,
    pub co_varnames: *mut PyObject,
    pub co_freevars: *mut PyObject,
    pub co_cellvars: *mut PyObject,
    #[cfg(not(Py_3_7))]
    pub co_cell2arg: *mut c_uchar,
    #[cfg(Py_3_7)]
    pub co_cell2arg: *mut Py_ssize_t,
    pub co_filename: *mut PyObject,
    pub co_name: *mut PyObject,
    #[cfg(not(Py_3_6))]
    pub co_firstlineno: c_int,
    #[cfg(not(Py_3_10))]
    pub co_lnotab: *mut PyObject,
    #[cfg(Py_3_10)]
    pub co_linetable: *mut PyObject,
    pub co_zombieframe: *mut c_void,
    pub co_weakreflist: *mut PyObject,
    #[cfg(Py_3_6)]
    pub co_extra: *mut c_void,
    #[cfg(Py_3_8)]
    pub co_opcache_map: *mut c_uchar,
    #[cfg(Py_3_8)]
    pub co_opcache: *mut _PyOpcache,
    #[cfg(Py_3_8)]
    pub co_opcache_flag: c_int,
    #[cfg(Py_3_8)]
    pub co_opcache_size: c_uchar,
}
impl Clone for PyCodeObject {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for PyCodeObject {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

/* Masks for co_flags */
pub const CO_OPTIMIZED: c_int = 0x0001;
pub const CO_NEWLOCALS: c_int = 0x0002;
pub const CO_VARARGS: c_int = 0x0004;
pub const CO_VARKEYWORDS: c_int = 0x0008;
pub const CO_NESTED: c_int = 0x0010;
pub const CO_GENERATOR: c_int = 0x0020;
/* The CO_NOFREE flag is set if there are no free or cell variables.
   This information is redundant, but it allows a single flag test
   to determine whether there is any extra work to be done when the
   call frame it setup.
*/
pub const CO_NOFREE: c_int = 0x0040;
/* The CO_COROUTINE flag is set for coroutine functions (defined with
``async def`` keywords) */
#[cfg(Py_3_5)]
pub const CO_COROUTINE: c_int = 0x0080;
#[cfg(Py_3_5)]
pub const CO_ITERABLE_COROUTINE: c_int = 0x0100;
#[cfg(Py_3_6)]
pub const CO_ASYNC_GENERATOR: c_int = 0x0200;

pub const CO_FUTURE_DIVISION: c_int = 0x2000;
pub const CO_FUTURE_ABSOLUTE_IMPORT: c_int = 0x4000; /* do absolute imports by default */
pub const CO_FUTURE_WITH_STATEMENT: c_int = 0x8000;
pub const CO_FUTURE_PRINT_FUNCTION: c_int = 0x10000;
pub const CO_FUTURE_UNICODE_LITERALS: c_int = 0x20000;
pub const CO_FUTURE_BARRY_AS_BDFL: c_int = 0x40000;
#[cfg(Py_3_5)]
pub const CO_FUTURE_GENERATOR_STOP: c_int = 0x80000;
#[cfg(Py_3_7)]
pub const CO_FUTURE_ANNOTATIONS: c_int = 0x100000;

#[cfg(not(Py_3_7))]
pub const CO_CELL_NOT_AN_ARG: c_uchar = 255;
#[cfg(Py_3_7)]
pub const CO_CELL_NOT_AN_ARG: Py_ssize_t = -1;

pub const CO_MAXBLOCKS: usize = 20;

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub static mut PyCode_Type: PyTypeObject;

    pub fn PyCode_New(
        argcount: c_int,
        kwonlyargcount: c_int,
        nlocals: c_int,
        stacksize: c_int,
        flags: c_int,
        code: *mut PyObject,
        consts: *mut PyObject,
        names: *mut PyObject,
        varnames: *mut PyObject,
        freevars: *mut PyObject,
        cellvars: *mut PyObject,
        filename: *mut PyObject,
        name: *mut PyObject,
        #[cfg(Py_3_11)]
        qualname: *mut PyObject,
        firstlineno: c_int,
        lnotab: *mut PyObject,
        #[cfg(Py_3_11)]
        exceptiontable: *mut PyObject,
    ) -> *mut PyCodeObject;

    #[cfg(Py_3_8)]
    pub fn PyCode_NewWithPosOnlyArgs(
        argcount: c_int,
        posonlyargcount: c_int,
        kwonlyargcount: c_int,
        nlocals: c_int,
        stacksize: c_int,
        flags: c_int,
        code: *mut PyObject,
        consts: *mut PyObject,
        names: *mut PyObject,
        varnames: *mut PyObject,
        freevars: *mut PyObject,
        cellvars: *mut PyObject,
        filename: *mut PyObject,
        name: *mut PyObject,
        #[cfg(Py_3_11)]
        qualname: *mut PyObject,
        firstlineno: c_int,
        lnotab: *mut PyObject,
        #[cfg(Py_3_11)]
        exceptiontable: *mut PyObject,
    ) -> *mut PyCodeObject;

    pub fn PyCode_NewEmpty(
        filename: *const c_char,
        funcname: *const c_char,
        firstlineno: c_int,
    ) -> *mut PyCodeObject;
    pub fn PyCode_Addr2Line(arg1: *mut PyCodeObject, arg2: c_int) -> c_int;
    #[cfg(Py_3_11)]
    pub fn PyCode_Addr2Location(
        co: *mut PyCodeObject,
        byte_offset: c_int,
        start_line: *mut c_int,
        start_column: *mut c_int,
        end_line: *mut c_int,
        end_column: *mut c_int
    ) -> c_int;
    #[cfg(Py_3_11)]
    pub fn PyCode_GetCode(co: *mut PyCodeObject) -> *mut PyObject;
    pub fn PyCode_Optimize(
        code: *mut PyObject,
        consts: *mut PyObject,
        names: *mut PyObject,
        lnotab: *mut PyObject,
    ) -> *mut PyObject;
}

#[inline]
pub unsafe fn PyCode_Check(op: *mut PyObject) -> c_int {
    (Py_TYPE(op) == &mut PyCode_Type) as c_int
}

#[inline]
#[cfg(Py_3_11)]
pub unsafe fn PyCode_GetNumFree(op: *mut PyCodeObject) -> Py_ssize_t {
    (*op).co_nfreevars as Py_ssize_t
}

#[inline]
#[cfg(not(Py_3_11))]
pub unsafe fn PyCode_GetNumFree(op: *mut PyCodeObject) -> Py_ssize_t {
    crate::tupleobject::PyTuple_GET_SIZE((*op).co_freevars)
}
