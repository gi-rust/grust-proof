// This file is part of Grust, GObject introspection bindings for Rust
//
// Copyright (C) 2015  Mikhail Zabaluev <mikhail.zabaluev@gmail.com>
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

use grust::flags;
use grust::value::Value;
use gio::FileAttributeInfoFlags;
use gio::flags::file_attribute_info::{NONE, COPY_WITH_FILE, COPY_WHEN_MOVED};

#[test]
fn flags() {
    assert_eq!(NONE, FileAttributeInfoFlags::empty());
    let flags: FileAttributeInfoFlags
        = flags::from_uint(COPY_WITH_FILE.bits() | COPY_WHEN_MOVED.bits())
            .unwrap();
    assert_eq!(flags, COPY_WITH_FILE | COPY_WHEN_MOVED);
}

#[test]
fn unknown_flags() {
    let a = COPY_WITH_FILE.bits() | 0b10000;
    let unknown_flags = flags::from_uint::<FileAttributeInfoFlags>(a)
                        .err().unwrap();
    assert_eq!(unknown_flags.actual(), a);
    assert_eq!(unknown_flags.known(), COPY_WITH_FILE.bits());
    assert_eq!(unknown_flags.unknown(), 0b10000);
}

#[test]
#[should_panic]
fn flags_unknown_panic() {
    let a = COPY_WITH_FILE.bits() | 0b10000;
    let _ = flags::from_uint::<FileAttributeInfoFlags>(a).unwrap();
}

#[test]
fn value_flags() {
    let mut value = Value::new(flags::type_of::<FileAttributeInfoFlags>());
    let flags = value.get_flags::<FileAttributeInfoFlags>().unwrap();
    assert_eq!(flags, FileAttributeInfoFlags::empty());
    value.set_flags(COPY_WITH_FILE | COPY_WHEN_MOVED);
    let value = value.clone();
    let flags = value.get_flags::<FileAttributeInfoFlags>().unwrap();
    assert_eq!(flags, COPY_WITH_FILE | COPY_WHEN_MOVED);
}
