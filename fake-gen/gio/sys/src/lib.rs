// Copyright (C) 2013-2015  Mikhail Zabaluev <mikhail.zabaluev@gmail.com>
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

#![crate_name = "gio-2_0-sys"]
#![crate_type = "lib"]

#![allow(missing_copy_implementations)]

extern crate "glib-2_0-sys" as glib;
extern crate "gobject-2_0-sys" as gobject;

use glib::types::{gchar, gint, gpointer};
use glib::GError;
use gobject::GType;

#[repr(C)]
pub struct GAsyncResult;

#[repr(C)]
pub struct GFile;

#[repr(C)]
pub struct GCancellable {
    pub parent_instance: gobject::GObject,
    _priv: gpointer
}

#[repr(C)]
pub struct GInputStream {
    pub parent_instance: gobject::GObject,
    _priv: gpointer
}

#[repr(C)]
pub struct GFileInputStream {
    pub parent_instance: GInputStream,
    _priv: gpointer
}

pub type GAsyncReadyCallback = extern "C" fn(source_object: *mut gobject::GObject,
                                             res: *mut GAsyncResult,
                                             user_data: gpointer);  

extern {
    pub fn g_async_result_get_type() -> GType;
    pub fn g_file_get_path(file: *mut GFile) -> *mut gchar;
    pub fn g_file_get_type() -> GType;
    pub fn g_file_new_for_path(path: *const gchar) -> *mut GFile;
    pub fn g_file_read_async(file: *mut GFile,
                             io_priority: gint,
                             cancellable: *mut GCancellable,
                             callback: GAsyncReadyCallback,
                             user_data: gpointer);
    pub fn g_file_read_finish(file: *mut GFile,
                              res: *mut GAsyncResult,
                              error: *mut *mut GError)
                              -> *mut GFileInputStream;
    pub fn g_file_attribute_info_flags_get_type() -> GType;
    pub fn g_file_input_stream_get_type() -> GType;
    pub fn g_input_stream_get_type() -> GType;
    pub fn g_io_error_enum_get_type() -> GType;
}
