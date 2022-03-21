use crate::ffi::object::*;
#[cfg(not(Py_LIMITED_API))]
use crate::ffi::pyarena::PyArena;
use crate::ffi::pystate::PyThreadState;
use libc::{wchar_t, FILE};
use std::os::raw::{c_char, c_int};
use std::ptr;

// TODO: PyCF_MASK etc. constants

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    // TODO: these moved to pylifecycle.h
    pub fn Py_SetProgramName(arg1: *mut wchar_t) -> ();
    #[cfg_attr(PyPy, link_name = "PyPy_GetProgramName")]
    pub fn Py_GetProgramName() -> *mut wchar_t;
    pub fn Py_SetPythonHome(arg1: *mut wchar_t) -> ();
    pub fn Py_GetPythonHome() -> *mut wchar_t;
    pub fn Py_Initialize() -> ();
    pub fn Py_InitializeEx(arg1: c_int) -> ();
    pub fn Py_Finalize() -> ();
    #[cfg_attr(PyPy, link_name = "PyPy_IsInitialized")]
    pub fn Py_IsInitialized() -> c_int;
    pub fn Py_NewInterpreter() -> *mut PyThreadState;
    pub fn Py_EndInterpreter(arg1: *mut PyThreadState) -> ();
}

#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(not(Py_LIMITED_API))]
pub struct PyCompilerFlags {
    pub cf_flags: c_int,
}

#[cfg(not(Py_LIMITED_API))]
pub enum _mod {}

#[cfg(not(Py_LIMITED_API))]
#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub fn PyRun_SimpleStringFlags(arg1: *const c_char, arg2: *mut PyCompilerFlags) -> c_int;
    pub fn PyRun_AnyFileFlags(
        arg1: *mut FILE,
        arg2: *const c_char,
        arg3: *mut PyCompilerFlags,
    ) -> c_int;
    pub fn PyRun_AnyFileExFlags(
        fp: *mut FILE,
        filename: *const c_char,
        closeit: c_int,
        flags: *mut PyCompilerFlags,
    ) -> c_int;
    pub fn PyRun_SimpleFileExFlags(
        fp: *mut FILE,
        filename: *const c_char,
        closeit: c_int,
        flags: *mut PyCompilerFlags,
    ) -> c_int;
    pub fn PyRun_InteractiveOneFlags(
        fp: *mut FILE,
        filename: *const c_char,
        flags: *mut PyCompilerFlags,
    ) -> c_int;
    pub fn PyRun_InteractiveOneObject(
        fp: *mut FILE,
        filename: *mut PyObject,
        flags: *mut PyCompilerFlags,
    ) -> c_int;
    pub fn PyRun_InteractiveLoopFlags(
        fp: *mut FILE,
        filename: *const c_char,
        flags: *mut PyCompilerFlags,
    ) -> c_int;
    pub fn PyParser_ASTFromString(
        s: *const c_char,
        filename: *const c_char,
        start: c_int,
        flags: *mut PyCompilerFlags,
        arena: *mut PyArena,
    ) -> *mut _mod;
    pub fn PyParser_ASTFromStringObject(
        s: *const c_char,
        filename: *mut PyObject,
        start: c_int,
        flags: *mut PyCompilerFlags,
        arena: *mut PyArena,
    ) -> *mut _mod;
    pub fn PyParser_ASTFromFile(
        fp: *mut FILE,
        filename: *const c_char,
        enc: *const c_char,
        start: c_int,
        ps1: *const c_char,
        ps2: *const c_char,
        flags: *mut PyCompilerFlags,
        errcode: *mut c_int,
        arena: *mut PyArena,
    ) -> *mut _mod;
    pub fn PyParser_ASTFromFileObject(
        fp: *mut FILE,
        filename: *mut PyObject,
        enc: *const c_char,
        start: c_int,
        ps1: *const c_char,
        ps2: *const c_char,
        flags: *mut PyCompilerFlags,
        errcode: *mut c_int,
        arena: *mut PyArena,
    ) -> *mut _mod;
}

pub enum symtable {}
pub enum _node {}

#[inline]
pub unsafe fn PyParser_SimpleParseString(s: *const c_char, b: c_int) -> *mut _node {
    PyParser_SimpleParseStringFlags(s, b, 0)
}

#[cfg(not(Py_LIMITED_API))]
#[inline]
pub unsafe fn PyParser_SimpleParseFile(fp: *mut FILE, s: *const c_char, b: c_int) -> *mut _node {
    PyParser_SimpleParseFileFlags(fp, s, b, 0)
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub fn PyParser_SimpleParseStringFlags(
        arg1: *const c_char,
        arg2: c_int,
        arg3: c_int,
    ) -> *mut _node;
    pub fn PyParser_SimpleParseStringFlagsFilename(
        arg1: *const c_char,
        arg2: *const c_char,
        arg3: c_int,
        arg4: c_int,
    ) -> *mut _node;
    #[cfg(not(Py_LIMITED_API))]
    pub fn PyParser_SimpleParseFileFlags(
        arg1: *mut FILE,
        arg2: *const c_char,
        arg3: c_int,
        arg4: c_int,
    ) -> *mut _node;
    #[cfg(not(Py_LIMITED_API))]
    #[cfg_attr(PyPy, link_name = "PyPyRun_StringFlags")]
    pub fn PyRun_StringFlags(
        arg1: *const c_char,
        arg2: c_int,
        arg3: *mut PyObject,
        arg4: *mut PyObject,
        arg5: *mut PyCompilerFlags,
    ) -> *mut PyObject;
    #[cfg(not(Py_LIMITED_API))]
    pub fn PyRun_FileExFlags(
        fp: *mut FILE,
        filename: *const c_char,
        start: c_int,
        globals: *mut PyObject,
        locals: *mut PyObject,
        closeit: c_int,
        flags: *mut PyCompilerFlags,
    ) -> *mut PyObject;
    #[cfg(Py_LIMITED_API)]
    #[cfg(not(PyPy))]
    pub fn Py_CompileString(string: *const c_char, p: *const c_char, s: c_int) -> *mut PyObject;
    #[cfg(PyPy)]
    #[cfg(not(Py_LIMITED_API))]
    #[cfg_attr(PyPy, link_name = "PyPy_CompileStringFlags")]
    pub fn Py_CompileStringFlags(
        string: *const c_char,
        p: *const c_char,
        s: c_int,
        f: *mut PyCompilerFlags,
    ) -> *mut PyObject;
}
#[cfg(not(Py_LIMITED_API))]
#[inline]
#[cfg(not(PyPy))]
pub unsafe fn Py_CompileString(string: *const c_char, p: *const c_char, s: c_int) -> *mut PyObject {
    Py_CompileStringExFlags(string, p, s, ptr::null_mut(), -1)
}

#[inline]
#[cfg(PyPy)]
pub unsafe fn Py_CompileString(string: *const c_char, p: *const c_char, s: c_int) -> *mut PyObject {
    Py_CompileStringFlags(string, p, s, ptr::null_mut())
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    #[cfg(not(Py_LIMITED_API))]
    #[cfg(not(PyPy))]
    pub fn Py_CompileStringExFlags(
        str: *const c_char,
        filename: *const c_char,
        start: c_int,
        flags: *mut PyCompilerFlags,
        optimize: c_int,
    ) -> *mut PyObject;
    #[cfg(not(Py_LIMITED_API))]
    pub fn Py_CompileStringObject(
        str: *const c_char,
        filename: *mut PyObject,
        start: c_int,
        flags: *mut PyCompilerFlags,
        optimize: c_int,
    ) -> *mut PyObject;
    pub fn Py_SymtableString(
        str: *const c_char,
        filename: *const c_char,
        start: c_int,
    ) -> *mut symtable;
    #[cfg(not(Py_LIMITED_API))]
    pub fn Py_SymtableStringObject(
        str: *const c_char,
        filename: *mut PyObject,
        start: c_int,
    ) -> *mut symtable;

    #[cfg_attr(PyPy, link_name = "PyPyErr_Print")]
    pub fn PyErr_Print() -> ();
    #[cfg_attr(PyPy, link_name = "PyPyErr_PrintEx")]
    pub fn PyErr_PrintEx(arg1: c_int) -> ();
    #[cfg_attr(PyPy, link_name = "PyPyErr_Display")]
    pub fn PyErr_Display(arg1: *mut PyObject, arg2: *mut PyObject, arg3: *mut PyObject) -> ();

    // TODO: these moved to pylifecycle.h
    #[cfg_attr(PyPy, link_name = "PyPy_AtExit")]
    pub fn Py_AtExit(func: Option<extern "C" fn() -> ()>) -> c_int;
    pub fn Py_Exit(arg1: c_int) -> ();
    pub fn Py_Main(argc: c_int, argv: *mut *mut wchar_t) -> c_int;
    pub fn Py_GetProgramFullPath() -> *mut wchar_t;
    pub fn Py_GetPrefix() -> *mut wchar_t;
    pub fn Py_GetExecPrefix() -> *mut wchar_t;
    pub fn Py_GetPath() -> *mut wchar_t;
    pub fn Py_SetPath(arg1: *const wchar_t) -> ();
    #[cfg_attr(PyPy, link_name = "PyPy_GetVersion")]
    pub fn Py_GetVersion() -> *const c_char;
    pub fn Py_GetPlatform() -> *const c_char;
    pub fn Py_GetCopyright() -> *const c_char;
    pub fn Py_GetCompiler() -> *const c_char;
    pub fn Py_GetBuildInfo() -> *const c_char;
}
