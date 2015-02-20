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

#![feature(core)]

#[macro_use]
extern crate grust;

#[macro_use]
#[no_link]
extern crate bitflags;

extern crate "gio-2_0-sys" as ffi;
extern crate "glib-2_0-sys" as glib_ffi;
extern crate "gobject-2_0-sys" as gobject_ffi;
extern crate "grust-GLib-2_0" as glib;
extern crate "grust-GObject-2_0" as gobject;

use grust::enumeration;
use grust::enumeration::IntrospectedEnum as _grust_IntrospectedEnumTrait;
use grust::error;
use grust::gstr;
use grust::gtype::GType;
use grust::marker;
use grust::object;
use grust::quark;
use grust::refcount;
use grust::types::{gint, gpointer};
use grust::wrap;

use std::fmt;
use std::mem;
use std::num;
use std::ptr;
use std::result;

#[repr(C)]
pub struct AsyncResult {
    raw: ffi::GAsyncResult,
    _marker: marker::ObjectMarker
}

unsafe impl wrap::Wrapper for AsyncResult {
    type Raw = ffi::GAsyncResult;
}

#[repr(C)]
pub struct File {
    raw: ffi::GFile,
    _marker: marker::ObjectMarker
}

unsafe impl wrap::Wrapper for File {
    type Raw = ffi::GFile;
}

#[repr(C)]
pub struct Cancellable {
    raw: ffi::GCancellable,
    _marker: marker::ObjectMarker
}

unsafe impl Send for Cancellable { }
unsafe impl Sync for Cancellable { }
unsafe impl wrap::Wrapper for Cancellable {
    type Raw = ffi::GCancellable;
}

#[repr(C)]
pub struct InputStream {
    raw: ffi::GInputStream,
    _marker: marker::ObjectMarker
}

unsafe impl wrap::Wrapper for InputStream {
    type Raw = ffi::GInputStream;
}

#[repr(C)]
pub struct FileInputStream {
    raw: ffi::GFileInputStream,
    _marker: marker::ObjectMarker
}

unsafe impl wrap::Wrapper for FileInputStream {
    type Raw = ffi::GFileInputStream;
}

#[derive(Copy, PartialEq, Eq, FromPrimitive, Debug)]
#[repr(C)]
pub enum IOErrorEnum {
    Failed = 0,
    NotFound = 1,
    Exists = 2,
    // ...
}

impl enumeration::IntrospectedEnum for IOErrorEnum {

    fn from_int(v: gint) -> Result<Self, enumeration::UnknownValue> {
        num::from_i32(v as i32)
            .ok_or_else(|| enumeration::UnknownValue(v))
    }

    fn to_int(&self) -> gint {
        *self as gint
    }

    fn name(&self) -> &'static str {
        match *self {
            IOErrorEnum::Failed => "failed",
            IOErrorEnum::NotFound => "not-found",
            IOErrorEnum::Exists   => "exists",
            // ...
        }
    }
}

impl enumeration::EnumType for IOErrorEnum {
    fn get_type() -> GType {
        unsafe { GType::from_raw(ffi::g_io_error_enum_get_type()) }
    }
}

impl error::Domain for IOErrorEnum {
    fn domain() -> quark::Quark {
        g_static_quark!(b"g-io-error-quark\0")
    }
}

impl fmt::Display for IOErrorEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

pub mod flags {

    pub mod file_attribute_info {
        use grust::flags::prelude::*;
        use ffi;
        use std::fmt;

        bitflags! {
            flags Flags: guint {
                const NONE            = 0,
                const COPY_WITH_FILE  = 1,
                const COPY_WHEN_MOVED = 2,
            }
        }

        impl IntrospectedFlags for Flags {

            fn from_uint(v: guint) -> Result<Flags, UnknownFlags> {
                Flags::from_bits(v)
                    .ok_or_else(|| UnknownFlags::new(v, Flags::all().bits()))
            }

            #[inline]
            fn to_uint(&self) -> guint {
                self.bits()
            }
        }

        impl FlagsType for Flags {
            fn get_type() -> GType {
                unsafe {
                    let raw = ffi::g_file_attribute_info_flags_get_type();
                    GType::from_raw(raw)
                }
            }
        }

        impl fmt::Debug for Flags {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                const FLAG_INFO: &'static [(Flags, &'static str)] = &[
                    (COPY_WITH_FILE,  "COPY_WITH_FILE"),
                    (COPY_WHEN_MOVED, "COPY_WHEN_MOVED")
                ];
                let mut contents = String::new();
                for &(flag, name) in FLAG_INFO.iter() {
                    if self.contains(flag) {
                        if contents.is_empty() {
                            contents.push_str(name);
                        } else {
                            contents.push('|');
                            contents.push_str(name);
                        }
                    }
                }
                write!(f, "FileAttributeInfoFlags({})", contents)
            }
        }
    }
}

pub use flags::file_attribute_info::Flags as FileAttributeInfoFlags;

mod async {
    use ffi;
    use gobject_ffi;
    use gobject;

    use grust::types::gpointer;
    use grust::wrap;
    use std::mem;

    pub extern "C" fn async_ready_callback<F>(source_object: *mut gobject_ffi::GObject,
                                              res: *mut ffi::GAsyncResult,
                                              user_data: gpointer)
        where F: FnOnce(&gobject::Object, &super::AsyncResult)
    {
        let cb: Box<F> = unsafe { mem::transmute(user_data) };
        let arg1 = unsafe { wrap::from_raw::<gobject::Object>(source_object) };
        let arg2 = unsafe { wrap::from_raw::<super::AsyncResult>(res) };
        cb(arg1, arg2);
    }
}

pub mod cast {
    use grust::object;

    use gobject;

    pub trait AsAsyncResult {
        fn as_async_result(&self) -> &super::AsyncResult;
    }

    impl<T> AsAsyncResult for T where T: object::Upcast<super::AsyncResult> {

        #[inline]
        fn as_async_result(&self) -> &super::AsyncResult {
            self.upcast()
        }
    }

    pub trait AsCancellable : gobject::cast::AsObject {
        fn as_cancellable(&self) -> &super::Cancellable;
    }

    impl<T> AsCancellable for T
        where T: object::Upcast<super::Cancellable>,
              T: object::Upcast<gobject::Object>
    {
        #[inline]
        fn as_cancellable(&self) -> &super::Cancellable {
            self.upcast()
        }
    }

    pub trait AsInputStream : gobject::cast::AsObject {
        fn as_input_stream(&self) -> &super::InputStream;
    }

    impl<T> AsInputStream for T
        where T: object::Upcast<super::InputStream>,
              T: object::Upcast<gobject::Object>
    {
        #[inline]
        fn as_input_stream(&self) -> &super::InputStream {
            self.upcast()
        }
    }

    pub trait AsFileInputStream : AsInputStream {
        fn as_file_input_stream(&self) -> &super::FileInputStream;
    }

    impl<T> AsFileInputStream for T
        where T: object::Upcast<super::FileInputStream>,
              T: object::Upcast<super::InputStream>,
              T: object::Upcast<gobject::Object>
    {
        #[inline]
        fn as_file_input_stream(&self) -> &super::FileInputStream {
            self.upcast()
        }
    }

    pub trait AsFile {
        fn as_file(&self) -> &super::File;
    }

    impl<T> AsFile for T where T: object::Upcast<super::File> {

        #[inline]
        fn as_file(&self) -> &super::File {
            self.upcast()
        }
    }
}

impl File {

    pub fn new_for_path(path: &gstr::Utf8) -> refcount::Ref<File> {
        unsafe {
            let ret = ffi::g_file_new_for_path(path.as_ptr());
            refcount::Ref::from_raw(ret)
        }
    }

    pub fn get_path(&self) -> gstr::OwnedGStr {
        unsafe {
            use grust::wrap::Wrapper;
            let ret = ffi::g_file_get_path(self.as_mut_ptr());
            gstr::OwnedGStr::from_ptr(ret)
        }
    }

    pub fn read_async<F>(&self,
                         io_priority: gint,
                         cancellable: Option<&Cancellable>,
                         callback: F)
        where F: FnOnce(&gobject::Object, &AsyncResult),
              F: 'static
    {
        unsafe {
            use grust::wrap::Wrapper;
            let self_raw = self.as_mut_ptr();
            let cancellable = {
                match cancellable {
                    Some(c) => c.as_mut_ptr(),
                    None    => std::ptr::null_mut()
                }
            };
            let callback: gpointer = mem::transmute(Box::new(callback));

            ffi::g_file_read_async(self_raw,
                                   io_priority,
                                   cancellable,
                                   async::async_ready_callback::<F>,
                                   callback);
        }
    }

    pub fn read_finish(&self, res: &AsyncResult)
                      -> result::Result<refcount::Ref<FileInputStream>,
                                        error::Error> {
        let mut err: *mut glib_ffi::GError = ptr::null_mut();
        let ret = unsafe {
            use grust::wrap::Wrapper;
            ffi::g_file_read_finish(self.as_mut_ptr(),
                                    res.as_mut_ptr(),
                                    &mut err)
        };
        if err.is_null() {
            Ok(unsafe { refcount::Ref::from_raw(ret) })
        } else {
            Err(unsafe { error::Error::from_raw(err) })
        }
    }
}

unsafe impl object::ObjectType for AsyncResult {
    fn get_type() -> GType {
        unsafe {
            GType::from_raw(ffi::g_async_result_get_type())
        }
    }
}

unsafe impl object::ObjectType for File {
    fn get_type() -> GType {
        unsafe {
            GType::from_raw(ffi::g_file_get_type())
        }
    }
}

unsafe impl object::ObjectType for InputStream {
    fn get_type() -> GType {
        unsafe {
            GType::from_raw(ffi::g_input_stream_get_type())
        }
    }
}

unsafe impl object::ObjectType for FileInputStream {
    fn get_type() -> GType {
        unsafe {
            GType::from_raw(ffi::g_file_input_stream_get_type())
        }
    }
}

impl object::Upcast<gobject::Object> for Cancellable {

    #[inline]
    fn upcast(&self) -> &gobject::Object {
        unsafe {
            wrap::from_raw(&self.raw.parent_instance)
        }
    }
}

impl object::Upcast<gobject::Object> for InputStream {

    #[inline]
    fn upcast(&self) -> &gobject::Object {
        unsafe {
            wrap::from_raw(&self.raw.parent_instance)
        }
    }
}

impl object::Upcast<InputStream> for FileInputStream {

    #[inline]
    fn upcast(&self) -> &InputStream {
        unsafe {
            wrap::from_raw(&self.raw.parent_instance)
        }
    }
}

impl object::Upcast<gobject::Object> for FileInputStream {

    #[inline]
    fn upcast(&self) -> &gobject::Object {
        use cast::AsInputStream;
        self.as_input_stream().upcast()
    }
}
