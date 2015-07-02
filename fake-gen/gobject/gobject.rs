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

#![crate_name = "grust_gobject_2_0"]
#![crate_type = "lib"]

extern crate grust;
extern crate grust_glib_2_0 as glib;
extern crate gobject_2_0_sys as ffi;

use grust::gtype::GType;
use grust::object;
use grust::wrap;

#[repr(C)]
pub struct TypeInstance {
    raw: ffi::GTypeInstance
}

unsafe impl wrap::Wrapper for TypeInstance {
    type Raw = ffi::GTypeInstance;
}

#[repr(C)]
pub struct Object {
    raw: ffi::GObject
}

unsafe impl wrap::Wrapper for Object {
    type Raw = ffi::GObject;
}

pub mod cast {
    use grust::object;

    pub trait AsObject {
        fn as_object(&self) -> &super::Object;
    }

    impl<T> AsObject for T where T: object::Upcast<super::Object> {

        #[inline]
        fn as_object(&self) -> &super::Object { self.upcast() }
    }
}

unsafe impl object::ObjectType for Object {
    fn get_type() -> GType {
        unsafe {
            GType::from_raw(ffi::g_object_get_type())
        }
    }
}
