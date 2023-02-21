use libc::c_char;
use libc::c_int;
use libc::c_void;
type VoidPtr = *mut c_void;

use std::ffi::CStr;

#[repr(C)]
struct Str {
    ptr: *const c_char,
    size: c_int,
}

pub const DEFAULT_MODEL: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"/usr/local/share/kytea/model.bin\0") };

#[link(name = "ckytea")]
extern "C" {
    fn new_kytea(model: *const c_char) -> VoidPtr;
    fn delete_kytea(void_kytea: VoidPtr);

    fn new_ostream() -> VoidPtr;
    fn new_istream(input: *const c_char) -> VoidPtr;
    fn delete_ostream(void_stream: VoidPtr);
    fn delete_istream(void_stream: VoidPtr);

    fn rewind_ostream(void_stream: VoidPtr);
    fn ostream_rust(void_stream: VoidPtr) -> Str;

    fn run_kytea_str_str(void_kytea: VoidPtr, input: VoidPtr, output: VoidPtr);
    fn run_kytea_file_str(void_kytea: VoidPtr, input: *const c_char, output: VoidPtr);
    fn run_kytea_str_file(void_kytea: VoidPtr, input: VoidPtr, output: *const c_char);
    fn run_kytea_file_file(void_kytea: VoidPtr, input: *const c_char, output: *const c_char);
}

pub struct Model {
    void_kytea: VoidPtr,
}

unsafe impl Send for Model {}
unsafe impl Sync for Model {}

impl Model {
    /// # Safety
    /// The provided slice **must** be nul-terminated and not contain any interior nul bytes.
    ///
    /// See [`CStr::from_bytes_with_nul_unchecked()`] for details.
    pub unsafe fn new_unchecked(model: &[u8]) -> Self {
        let model = CStr::from_bytes_with_nul_unchecked(model);
        Self::new(model)
    }

    pub fn new(model: &CStr) -> Self {
        Self {
            void_kytea: unsafe { new_kytea(model.as_ptr()) },
        }
    }

    #[inline]
    pub fn output() -> Ostream {
        Ostream::new()
    }

    pub fn tokenize_to_str(&mut self, input: &Istream, output: &mut Ostream) {
        unsafe {
            match input {
                Istream::File(input) => {
                    run_kytea_file_str(self.void_kytea, *input, output.void_stream)
                }
                Istream::Buf(input) => {
                    run_kytea_str_str(self.void_kytea, *input, output.void_stream)
                }
            };
        }
    }

    pub fn tokenize_to_file(&mut self, input: &Istream, output: &CStr) {
        unsafe {
            match input {
                Istream::File(input) => {
                    run_kytea_file_file(self.void_kytea, *input, output.as_ptr())
                }
                Istream::Buf(input) => run_kytea_str_file(self.void_kytea, *input, output.as_ptr()),
            };
        }
    }
}

impl Drop for Model {
    fn drop(&mut self) {
        unsafe {
            delete_kytea(self.void_kytea);
        }
    }
}

pub enum Istream {
    File(*const c_char),
    Buf(VoidPtr),
}

unsafe impl Send for Istream {}
unsafe impl Sync for Istream {}

impl Istream {
    /// # Safety
    /// The provided slice **must** be nul-terminated and not contain any interior nul bytes.
    ///
    /// See [`CStr::from_bytes_with_nul_unchecked()`] for details.
    pub unsafe fn from_file_unchecked(fname: &[u8]) -> Self {
        let fname = CStr::from_bytes_with_nul_unchecked(fname);
        Self::from_file(fname)
    }

    pub fn from_file(fname: &CStr) -> Self {
        Self::File(fname.as_ptr())
    }

    /// # Safety
    /// The provided slice **must** be nul-terminated and not contain any interior nul bytes.
    ///
    /// See [`CStr::from_bytes_with_nul_unchecked()`] for details.
    pub unsafe fn from_buffer_unchecked(bytes: &[u8]) -> Self {
        let bytes = CStr::from_bytes_with_nul_unchecked(bytes);
        Self::from_buffer(bytes)
    }

    /// # Safety
    /// The provided slice **must** be nul-terminated and not contain any interior nul bytes.
    ///
    /// See [`CStr::from_bytes_with_nul_unchecked()`] for details.
    pub fn from_buffer(bytes: &CStr) -> Self {
        unsafe { Self::Buf(new_istream(bytes.as_ptr())) }
    }
}

impl Drop for Istream {
    fn drop(&mut self) {
        if let &mut Self::Buf(void_stream) = self {
            unsafe {
                delete_istream(void_stream);
            }
        }
    }
}

pub struct Ostream {
    void_stream: VoidPtr,
}

unsafe impl Send for Ostream {}
unsafe impl Sync for Ostream {}

impl Default for Ostream {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Ostream {
    pub fn new() -> Self {
        unsafe {
            Self {
                void_stream: new_ostream(),
            }
        }
    }

    pub fn as_bytes<'a>(&self) -> &'a [u8] {
        let Str { ptr, size } = unsafe { ostream_rust(self.void_stream) };
        unsafe { std::slice::from_raw_parts(ptr as *const u8, size as usize) }
    }

    /// # Safety
    /// The input bytes ([`Istream`]) must be valid UTF-8.
    pub unsafe fn as_str_unchecked<'a>(&self) -> &'a str {
        std::str::from_utf8_unchecked(self.as_bytes())
    }

    pub fn rewind(&mut self) {
        unsafe {
            rewind_ostream(self.void_stream);
        }
    }
}

impl Drop for Ostream {
    fn drop(&mut self) {
        unsafe {
            delete_ostream(self.void_stream);
        }
    }
}
