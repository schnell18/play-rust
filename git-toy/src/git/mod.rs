mod raw;


use std;
use libc;
use std::error;
use std::fmt;
use std::result;
use std::os::raw::{c_int, c_char};
use std::io::Write;
use std::ffi::CStr;
use std::ffi::CString;
use std::path::Path;
use std::ptr::null_mut;
use std::mem::uninitialized;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Error {
    code: i32,
    message: String,
    class: i32
}

pub struct Repository {
    // This must always be a pointer to a live `git_repository`
    // structure. No other `Repository` may point to it.
    raw: *mut raw::git_repository
}

pub struct Commit<'repo> {
    // This must always be a pointer to a usable `git_commit` structure
    raw: *mut raw::git_commit,
    _marker: PhantomData<&'repo Repository>
}

/// The identifier of some sort of object stored in the Git object
/// database: a commit, tree, blob, tag, etc. This is a wide hash
/// of the object's contents.
pub struct Oid {
    pub raw: raw::git_oid
}

pub struct Signature<'text> {
    raw: *const raw::git_signature,
    _marker: PhantomData<&'text str>
}

impl Repository {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Repository> {
        ensure_initialized();

        let path = path_to_cstring(path.as_ref())?;
        let mut repo = null_mut();
        unsafe {
            check(raw::git_repository_open(&mut repo, path.as_ptr()))?;
        }
        Ok(Repository { raw: repo })
    }

    pub fn reference_name_to_id(&self, name: &str) -> Result<Oid> {
        let name = CString::new(name)?;
        unsafe {
            let mut oid = uninitialized();
            check(
                raw::git_reference_name_to_id(
                    &mut oid,
                    self.raw,
                    name.as_ptr() as *const c_char
                )
            )?;
            Ok(Oid { raw: oid })
        }
    }

    pub fn find_commit(&self, oid: &Oid) -> Result<Commit> {
        let mut commit = null_mut();
        unsafe {
            check(raw::git_commit_lookup(&mut commit, self.raw, &oid.raw))?;
        }
        Ok(Commit { raw: commit, _marker: PhantomData })
    }
}

impl<'repo> Commit<'repo> {
    pub fn author(&self) -> Signature {
        unsafe {
            Signature {
                raw: raw::git_commit_author(self.raw),
                _marker: PhantomData
            }
        }
    }

    pub fn message(&self) -> Option<&str> {
        unsafe {
            let message = raw::git_commit_message(self.raw);
            char_ptr_to_str(self, message)
        }
    }
}

impl<'text> Signature<'text> {
    /// Return the author's name as a `&str`,
    /// or `None` if it is not well-formed UTF-8.
    pub fn name(&self) -> Option<&str> {
        unsafe {
            char_ptr_to_str(self, (*self.raw).name)
        }
    }

    /// Return the author's email as a `&str`,
    /// or `None` if it is not well-formed UTF-8.
    pub fn email(&self) -> Option<&str> {
        unsafe {
            char_ptr_to_str(self, (*self.raw).email)
        }
    }
}

impl Drop for Repository {
    fn drop(&mut self) {
        unsafe {
            raw::git_repository_free(self.raw);
        }
    }
}

impl<'repo> Drop for Commit<'repo> {
    fn drop(&mut self) {
        unsafe {
            raw::git_commit_free(self.raw);
        }
    }
}

fn ensure_initialized() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        unsafe {
            check(raw::git_libgit2_init())
                .expect("initializing libgit2 failed");
            assert_eq!(libc::atexit(shutdown), 0);

        }
    });
}

extern fn shutdown() {
    unsafe {
        if let Err(e) = check(raw::git_libgit2_shutdown()) {
            let _ = writeln!(
                std::io::stderr(),
                "shutting down libgit2 failed: {}",
                e
            );
            std::process::abort();
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        // Displaying an `Error` simply displays
        // the message from libgit2
        self.message.fmt(f)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }
}

pub type Result<T> = result::Result<T, Error>;

fn check(code: c_int) -> Result<c_int> {
    if code >= 0 {
        return Ok(code);
    }
    unsafe {
        let error = raw::giterr_last();
        // libgit2 ensures that (*error).message is always non-null
        // and null terminated, so this call is safe.
        let message = CStr::from_ptr((*error).message)
            .to_string_lossy()
            .into_owned();
        Err(Error {
            code: code as i32,
            message,
            class: (*error).klass as i32
        })
    }
}

#[cfg(unix)]
fn path_to_cstring(path: &Path) -> Result<CString> {
    // The `as_bytes` method exists only on Unix-like systems.
    use std::os::unix::ffi::OsStrExt;
    Ok(CString::new(path.as_os_str().as_bytes())?)
}

#[cfg(windows)]
fn path_to_cstring(path: &Path) -> Result<CString> {
    // Try to convert to UTF-8.
    // If this failes, libgit2 can't handle the path anyway.
    match path.to_str() {
        Some(s) => Ok(CString::new(s)?),
        None => {
            let message = format!(
                "Couldn't convert path '{}' to UTF-8",
                path.display()
            );
            Err(message.into())
        }
    }
}

impl From<String> for Error {
    fn from(message: String) -> Error {
        Error { code: -1, message, class: 0}
    }
}

impl From<std::ffi::NulError> for Error {
    fn from(e: std::ffi::NulError) -> Error {
        Error { code: -1, message: e.to_string(), class: 0}
    }
}

/// Try to borrow a `&str`, given that `ptr` may be null or
/// refer to ill-formed UTF-8. Give the result a lifetime as if it were
/// borrowed from `_owner`.
///
/// Safety: if `ptr` is non-null, it must point to a null-terminated C
/// string that is safe to access.
unsafe fn char_ptr_to_str<T>(_owner: &T, ptr: *const c_char) -> Option<&str> {
    if ptr.is_null() {
        return None;
    }
    else {
        CStr::from_ptr(ptr).to_str().ok()
    }
}
