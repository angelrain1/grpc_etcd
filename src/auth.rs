// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct User {
    // message fields
    pub name: ::std::vec::Vec<u8>,
    pub password: ::std::vec::Vec<u8>,
    pub roles: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for User {}

impl User {
    pub fn new() -> User {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static User {
        static mut instance: ::protobuf::lazy::Lazy<User> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const User,
        };
        unsafe {
            instance.get(User::new)
        }
    }

    // bytes name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::vec::Vec<u8>) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.name, ::std::vec::Vec::new())
    }

    pub fn get_name(&self) -> &[u8] {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.name
    }

    // bytes password = 2;

    pub fn clear_password(&mut self) {
        self.password.clear();
    }

    // Param is passed by value, moved
    pub fn set_password(&mut self, v: ::std::vec::Vec<u8>) {
        self.password = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_password(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.password
    }

    // Take field
    pub fn take_password(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.password, ::std::vec::Vec::new())
    }

    pub fn get_password(&self) -> &[u8] {
        &self.password
    }

    fn get_password_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.password
    }

    fn mut_password_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.password
    }

    // repeated string roles = 3;

    pub fn clear_roles(&mut self) {
        self.roles.clear();
    }

    // Param is passed by value, moved
    pub fn set_roles(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.roles = v;
    }

    // Mutable pointer to the field.
    pub fn mut_roles(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.roles
    }

    // Take field
    pub fn take_roles(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.roles, ::protobuf::RepeatedField::new())
    }

    pub fn get_roles(&self) -> &[::std::string::String] {
        &self.roles
    }

    fn get_roles_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.roles
    }

    fn mut_roles_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.roles
    }
}

impl ::protobuf::Message for User {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.password)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.roles)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.name);
        }
        if !self.password.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.password);
        }
        for value in &self.roles {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_bytes(1, &self.name)?;
        }
        if !self.password.is_empty() {
            os.write_bytes(2, &self.password)?;
        }
        for v in &self.roles {
            os.write_string(3, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for User {
    fn new() -> User {
        User::new()
    }

    fn descriptor_static(_: ::std::option::Option<User>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "name",
                    User::get_name_for_reflect,
                    User::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "password",
                    User::get_password_for_reflect,
                    User::mut_password_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "roles",
                    User::get_roles_for_reflect,
                    User::mut_roles_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<User>(
                    "User",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for User {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_password();
        self.clear_roles();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for User {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for User {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Permission {
    // message fields
    pub permType: Permission_Type,
    pub key: ::std::vec::Vec<u8>,
    pub range_end: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Permission {}

impl Permission {
    pub fn new() -> Permission {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Permission {
        static mut instance: ::protobuf::lazy::Lazy<Permission> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Permission,
        };
        unsafe {
            instance.get(Permission::new)
        }
    }

    // .authpb.Permission.Type permType = 1;

    pub fn clear_permType(&mut self) {
        self.permType = Permission_Type::READ;
    }

    // Param is passed by value, moved
    pub fn set_permType(&mut self, v: Permission_Type) {
        self.permType = v;
    }

    pub fn get_permType(&self) -> Permission_Type {
        self.permType
    }

    fn get_permType_for_reflect(&self) -> &Permission_Type {
        &self.permType
    }

    fn mut_permType_for_reflect(&mut self) -> &mut Permission_Type {
        &mut self.permType
    }

    // bytes key = 2;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // bytes range_end = 3;

    pub fn clear_range_end(&mut self) {
        self.range_end.clear();
    }

    // Param is passed by value, moved
    pub fn set_range_end(&mut self, v: ::std::vec::Vec<u8>) {
        self.range_end = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_range_end(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.range_end
    }

    // Take field
    pub fn take_range_end(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.range_end, ::std::vec::Vec::new())
    }

    pub fn get_range_end(&self) -> &[u8] {
        &self.range_end
    }

    fn get_range_end_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.range_end
    }

    fn mut_range_end_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.range_end
    }
}

impl ::protobuf::Message for Permission {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.permType = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.key)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.range_end)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.permType != Permission_Type::READ {
            my_size += ::protobuf::rt::enum_size(1, self.permType);
        }
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.key);
        }
        if !self.range_end.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.range_end);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.permType != Permission_Type::READ {
            os.write_enum(1, self.permType.value())?;
        }
        if !self.key.is_empty() {
            os.write_bytes(2, &self.key)?;
        }
        if !self.range_end.is_empty() {
            os.write_bytes(3, &self.range_end)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Permission {
    fn new() -> Permission {
        Permission::new()
    }

    fn descriptor_static(_: ::std::option::Option<Permission>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Permission_Type>>(
                    "permType",
                    Permission::get_permType_for_reflect,
                    Permission::mut_permType_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    Permission::get_key_for_reflect,
                    Permission::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "range_end",
                    Permission::get_range_end_for_reflect,
                    Permission::mut_range_end_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Permission>(
                    "Permission",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Permission {
    fn clear(&mut self) {
        self.clear_permType();
        self.clear_key();
        self.clear_range_end();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Permission {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Permission {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Permission_Type {
    READ = 0,
    WRITE = 1,
    READWRITE = 2,
}

impl ::protobuf::ProtobufEnum for Permission_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Permission_Type> {
        match value {
            0 => ::std::option::Option::Some(Permission_Type::READ),
            1 => ::std::option::Option::Some(Permission_Type::WRITE),
            2 => ::std::option::Option::Some(Permission_Type::READWRITE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Permission_Type] = &[
            Permission_Type::READ,
            Permission_Type::WRITE,
            Permission_Type::READWRITE,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Permission_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Permission_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Permission_Type {
}

impl ::std::default::Default for Permission_Type {
    fn default() -> Self {
        Permission_Type::READ
    }
}

impl ::protobuf::reflect::ProtobufValue for Permission_Type {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Role {
    // message fields
    pub name: ::std::vec::Vec<u8>,
    pub keyPermission: ::protobuf::RepeatedField<Permission>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Role {}

impl Role {
    pub fn new() -> Role {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Role {
        static mut instance: ::protobuf::lazy::Lazy<Role> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Role,
        };
        unsafe {
            instance.get(Role::new)
        }
    }

    // bytes name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::vec::Vec<u8>) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.name, ::std::vec::Vec::new())
    }

    pub fn get_name(&self) -> &[u8] {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.name
    }

    // repeated .authpb.Permission keyPermission = 2;

    pub fn clear_keyPermission(&mut self) {
        self.keyPermission.clear();
    }

    // Param is passed by value, moved
    pub fn set_keyPermission(&mut self, v: ::protobuf::RepeatedField<Permission>) {
        self.keyPermission = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keyPermission(&mut self) -> &mut ::protobuf::RepeatedField<Permission> {
        &mut self.keyPermission
    }

    // Take field
    pub fn take_keyPermission(&mut self) -> ::protobuf::RepeatedField<Permission> {
        ::std::mem::replace(&mut self.keyPermission, ::protobuf::RepeatedField::new())
    }

    pub fn get_keyPermission(&self) -> &[Permission] {
        &self.keyPermission
    }

    fn get_keyPermission_for_reflect(&self) -> &::protobuf::RepeatedField<Permission> {
        &self.keyPermission
    }

    fn mut_keyPermission_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Permission> {
        &mut self.keyPermission
    }
}

impl ::protobuf::Message for Role {
    fn is_initialized(&self) -> bool {
        for v in &self.keyPermission {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.keyPermission)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.name);
        }
        for value in &self.keyPermission {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_bytes(1, &self.name)?;
        }
        for v in &self.keyPermission {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Role {
    fn new() -> Role {
        Role::new()
    }

    fn descriptor_static(_: ::std::option::Option<Role>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "name",
                    Role::get_name_for_reflect,
                    Role::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Permission>>(
                    "keyPermission",
                    Role::get_keyPermission_for_reflect,
                    Role::mut_keyPermission_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Role>(
                    "Role",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Role {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_keyPermission();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Role {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Role {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0esrc/auth.proto\x12\x06authpb\"L\n\x04User\x12\x12\n\x04name\x18\
    \x01\x20\x01(\x0cR\x04name\x12\x1a\n\x08password\x18\x02\x20\x01(\x0cR\
    \x08password\x12\x14\n\x05roles\x18\x03\x20\x03(\tR\x05roles\"\x9c\x01\n\
    \nPermission\x123\n\x08permType\x18\x01\x20\x01(\x0e2\x17.authpb.Permiss\
    ion.TypeR\x08permType\x12\x10\n\x03key\x18\x02\x20\x01(\x0cR\x03key\x12\
    \x1b\n\trange_end\x18\x03\x20\x01(\x0cR\x08rangeEnd\"*\n\x04Type\x12\x08\
    \n\x04READ\x10\0\x12\t\n\x05WRITE\x10\x01\x12\r\n\tREADWRITE\x10\x02\"T\
    \n\x04Role\x12\x12\n\x04name\x18\x01\x20\x01(\x0cR\x04name\x128\n\rkeyPe\
    rmission\x18\x02\x20\x03(\x0b2\x12.authpb.PermissionR\rkeyPermissionb\
    \x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
