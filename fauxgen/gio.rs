// This file is part of Grust, GObject introspection bindings for Rust
//
// Copyright (C) 2013, 2014  Mikhail Zabaluev <mikhail.zabaluev@gmail.com>
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA

#![crate_name = "grust-Gio-2_0"]

#![crate_type = "lib"]

#![allow(unstable)]

#[macro_use]
extern crate grust;

extern crate "grust-GLib-2_0" as glib;
extern crate "grust-GObject-2_0" as gobject;

extern crate libc;

use grust::error;
use grust::gstr;
use grust::gstr::IntoUtf8;
use grust::gtype::GType;
use grust::marker;
use grust::object;
use grust::quark;
use grust::refcount;
use grust::types::{gint, gpointer};
use grust::wrap;

use std::fmt;
use std::num::FromPrimitive;
use std::mem;

#[repr(C)]
pub struct AsyncResult {
    raw: raw::GAsyncResult,
    _marker: marker::ObjectMarker
}

unsafe impl wrap::Wrapper for AsyncResult {
    type Raw = raw::GAsyncResult;
}

#[repr(C)]
pub struct File {
    raw: raw::GFile,
    _marker: marker::ObjectMarker
}

unsafe impl wrap::Wrapper for File {
    type Raw = raw::GFile;
}

#[repr(C)]
pub struct Cancellable {
    raw: raw::GCancellable,
    _marker: marker::SyncObjectMarker
}

unsafe impl wrap::Wrapper for Cancellable {
    type Raw = raw::GCancellable;
}

#[repr(C)]
pub struct InputStream {
    raw: raw::GInputStream,
    _marker: marker::ObjectMarker
}

unsafe impl wrap::Wrapper for InputStream {
    type Raw = raw::GInputStream;
}

#[repr(C)]
pub struct FileInputStream {
    raw: raw::GFileInputStream,
    _marker: marker::ObjectMarker
}

unsafe impl wrap::Wrapper for FileInputStream {
    type Raw = raw::GFileInputStream;
}

#[derive(Copy, PartialEq, Eq, FromPrimitive)]
#[repr(C)]
pub enum IOErrorEnum {
    Failed = 0,
    NotFound = 1,
    Exists = 2,
    // ...
}

impl fmt::Show for IOErrorEnum {
    fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
        let s: &'static str = match *self {
            IOErrorEnum::Failed   => "failed",
            IOErrorEnum::NotFound => "not-found",
            IOErrorEnum::Exists   => "exists",
            // ...
        };
        s.fmt(format)
    }
}

impl IOErrorEnum {

    pub fn error_domain() -> quark::Quark {
        g_static_quark!(b"g-io-error-quark\0")
    }

    pub fn from_error(err: &error::Error) -> error::Match<IOErrorEnum> {
        let (domain, code) = err.key();
        if domain != IOErrorEnum::error_domain() {
            return error::Match::NotInDomain;
        }
        if let Some(v) = FromPrimitive::from_i64(code as i64) {
            error::Match::Known(v)
        } else {
            error::Match::Unknown(code)
        }
    }
}

#[allow(missing_copy_implementations)]
pub mod raw {
    use grust::types::{gchar, gint, gpointer};
    use grust::gtype::raw::GType;
    use grust::error::raw::GError;
    use gobject;
    use libc;

    #[repr(C)]
    pub struct GAsyncResult;

    #[repr(C)]
    pub struct GFile;

    #[repr(C)]
    pub struct GCancellable {
        pub parent_instance: gobject::raw::GObject,
        _priv: gpointer
    }

    #[repr(C)]
    pub struct GInputStream {
        pub parent_instance: gobject::raw::GObject,
        _priv: gpointer
    }

    #[repr(C)]
    pub struct GFileInputStream {
        pub parent_instance: GInputStream,
        _priv: gpointer
    }

    pub type GAsyncReadyCallback = extern "C" fn(source_object: *mut gobject::raw::GObject,
                                                 res: *mut GAsyncResult,
                                                 user_data: gpointer);  

    #[link(name="gio-2.0")]
    extern {
        pub fn g_async_result_get_type() -> GType;
        pub fn g_file_get_type() -> GType;
        pub fn g_file_new_for_path(path: *const gchar) -> *mut GFile;
        pub fn g_file_get_path(file: *mut GFile) -> *mut libc::c_char;
        pub fn g_file_read_async(file: *mut GFile,
                                 io_priority: gint,
                                 cancellable: *mut GCancellable,
                                 callback: GAsyncReadyCallback,
                                 user_data: gpointer);
        pub fn g_file_read_finish(file: *mut GFile,
                                  res: *mut GAsyncResult,
                                  error: *mut *mut GError)
                                  -> *mut GFileInputStream;
        pub fn g_file_input_stream_get_type() -> GType;
    }
}

mod async {
    use super::raw;
    use gobject;

    use grust::types::gpointer;
    use grust::wrap;
    use std::mem;

    pub extern "C" fn async_ready_callback<F>(source_object: *mut gobject::raw::GObject,
                                              res: *mut raw::GAsyncResult,
                                              user_data: gpointer)
        where F: FnOnce(&mut gobject::Object, &mut super::AsyncResult)
    {
        let cb: Box<F> = unsafe { mem::transmute(user_data) };
        let arg1 = unsafe {
            wrap::from_raw_mut::<gobject::Object, _>(source_object,
                                                     &source_object)
        };
        let arg2 = unsafe {
            wrap::from_raw_mut::<super::AsyncResult, _>(res, &res)
        };
        cb(arg1, arg2);
    }
}

pub mod cast {
    use grust::object;

    use gobject;

    pub trait AsAsyncResult {
        fn as_gio_async_result(&self) -> &super::AsyncResult;
        fn as_mut_gio_async_result(&mut self) -> &mut super::AsyncResult;
    }

    impl<T> AsAsyncResult for T where T: object::Upcast<super::AsyncResult> {

        #[inline]
        fn as_gio_async_result(&self) -> &super::AsyncResult {
            self.upcast()
        }

        #[inline]
        fn as_mut_gio_async_result(&mut self) -> &mut super::AsyncResult {
            self.upcast_mut()
        }
    }

    pub trait AsCancellable : gobject::cast::AsObject {
        fn as_gio_cancellable(&self) -> &super::Cancellable;
        fn as_mut_gio_cancellable(&mut self) -> &mut super::Cancellable;
    }

    impl<T> AsCancellable for T
        where T: object::Upcast<super::Cancellable>,
              T: object::Upcast<gobject::Object>
    {
        #[inline]
        fn as_gio_cancellable(&self) -> &super::Cancellable {
            self.upcast()
        }

        #[inline]
        fn as_mut_gio_cancellable(&mut self) -> &mut super::Cancellable {
            self.upcast_mut()
        }
    }

    pub trait AsInputStream : gobject::cast::AsObject {
        fn as_gio_input_stream(&self) -> &super::InputStream;
        fn as_mut_gio_input_stream(&mut self) -> &mut super::InputStream;
    }

    impl<T> AsInputStream for T
        where T: object::Upcast<super::InputStream>,
              T: object::Upcast<gobject::Object>
    {
        #[inline]
        fn as_gio_input_stream(&self) -> &super::InputStream {
            self.upcast()
        }

        #[inline]
        fn as_mut_gio_input_stream(&mut self) -> &mut super::InputStream {
            self.upcast_mut()
        }
    }

    pub trait AsFileInputStream : AsInputStream {
        fn as_gio_file_input_stream(&self) -> &super::FileInputStream;
        fn as_mut_gio_file_input_stream(&mut self) -> &mut super::FileInputStream;
    }

    impl<T> AsFileInputStream for T
        where T: object::Upcast<super::FileInputStream>,
              T: object::Upcast<super::InputStream>,
              T: object::Upcast<gobject::Object>
    {
        #[inline]
        fn as_gio_file_input_stream(&self) -> &super::FileInputStream {
            self.upcast()
        }

        #[inline]
        fn as_mut_gio_file_input_stream(&mut self) -> &mut super::FileInputStream {
            self.upcast_mut()
        }
    }

    pub trait AsFile {
        fn as_gio_file(&self) -> &super::File;
        fn as_mut_gio_file(&mut self) -> &mut super::File;
    }

    impl<T> AsFile for T where T: object::Upcast<super::File> {

        #[inline]
        fn as_gio_file(&self) -> &super::File {
            self.upcast()
        }

        #[inline]
        fn as_mut_gio_file(&mut self) -> &mut super::File {
            self.upcast_mut()
        }
    }
}

impl File {

    // TODO: need a macro for static UTF8In literals
    // to make the argument &UTF8In without having to put tedious code
    // into existing tests
    pub fn new_for_path(path: &str) -> refcount::Ref<File> {
        let p = path.into_utf8().unwrap();
        unsafe {
            let ret = raw::g_file_new_for_path(p.as_ptr());
            refcount::Ref::from_raw(ret)
        }
    }

    pub fn get_path<'a>(&mut self) -> gstr::GStr {
        unsafe {
            let ret = raw::g_file_get_path(&mut self.raw);
            gstr::GStr::from_raw_buf(ret)
        }
    }

    pub fn read_async<F>(&mut self,
                         io_priority: gint,
                         cancellable: Option<&mut Cancellable>,
                         callback: F)
        where F : FnOnce(&mut gobject::Object, &mut AsyncResult)
    {
        unsafe {
            let cancellable = {
                use grust::wrap::Wrapper;
                match cancellable {
                    Some(c) => c.as_mut_ptr(),
                    None    => std::ptr::null_mut()
                }
            };
            let callback: gpointer = mem::transmute(Box::new(callback));

            raw::g_file_read_async(&mut self.raw,
                                   io_priority as libc::c_int,
                                   cancellable,
                                   async::async_ready_callback::<F>,
                                   callback);
        }
    }

    pub fn read_finish(&mut self, res: &mut AsyncResult)
                      -> std::result::Result<refcount::Ref<FileInputStream>,
                                             grust::error::Error> {
        use grust::wrap::Wrapper;
        unsafe {
            let mut err: grust::error::Error = grust::error::none();
            let ret = raw::g_file_read_finish(&mut self.raw,
                                              res.as_mut_ptr(),
                                              err.slot_ptr());
            if err.is_set() {
                Err(err)
            } else {
                Ok(refcount::Ref::from_raw(ret))
            }
        }
    }
}

unsafe impl object::ObjectType for AsyncResult {
    fn get_type(_: Option<&Self>) -> GType {
        unsafe {
            GType::new(raw::g_async_result_get_type())
        }
    }
}

unsafe impl object::ObjectType for File {
    fn get_type(_: Option<&Self>) -> GType {
        unsafe {
            GType::new(raw::g_file_get_type())
        }
    }
}

unsafe impl object::ObjectType for FileInputStream {
    fn get_type(_: Option<&Self>) -> GType {
        unsafe {
            GType::new(raw::g_file_input_stream_get_type())
        }
    }
}

impl object::Upcast<gobject::Object> for Cancellable {

    #[inline]
    fn upcast(&self) -> &gobject::Object {
        unsafe {
            wrap::from_raw(&self.raw.parent_instance, self)
        }
    }

    #[inline]
    fn upcast_mut(&mut self) -> &mut gobject::Object {
        unsafe {
            wrap::from_raw_mut(&mut self.raw.parent_instance, self)
        }
    }
}

impl object::Upcast<gobject::Object> for InputStream {

    #[inline]
    fn upcast(&self) -> &gobject::Object {
        unsafe {
            wrap::from_raw(&self.raw.parent_instance, self)
        }
    }

    #[inline]
    fn upcast_mut(&mut self) -> &mut gobject::Object {
        unsafe {
            wrap::from_raw_mut(&mut self.raw.parent_instance, self)
        }
    }
}

impl object::Upcast<InputStream> for FileInputStream {

    #[inline]
    fn upcast(&self) -> &InputStream {
        unsafe {
            wrap::from_raw(&self.raw.parent_instance, self)
        }
    }

    #[inline]
    fn upcast_mut(&mut self) -> &mut InputStream {
        unsafe {
            wrap::from_raw_mut(&mut self.raw.parent_instance, self)
        }
    }
}

impl object::Upcast<gobject::Object> for FileInputStream {

    #[inline]
    fn upcast(&self) -> &gobject::Object {
        use cast::AsInputStream;
        self.as_gio_input_stream().upcast()
    }

    #[inline]
    fn upcast_mut(&mut self) -> &mut gobject::Object {
        use cast::AsInputStream;
        self.as_mut_gio_input_stream().upcast_mut()
    }
}
