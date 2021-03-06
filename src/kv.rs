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
pub struct KeyValue {
    // message fields
    pub key: ::std::vec::Vec<u8>,
    pub create_revision: i64,
    pub mod_revision: i64,
    pub version: i64,
    pub value: ::std::vec::Vec<u8>,
    pub lease: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for KeyValue {}

impl KeyValue {
    pub fn new() -> KeyValue {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static KeyValue {
        static mut instance: ::protobuf::lazy::Lazy<KeyValue> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const KeyValue,
        };
        unsafe {
            instance.get(KeyValue::new)
        }
    }

    // bytes key = 1;

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

    // int64 create_revision = 2;

    pub fn clear_create_revision(&mut self) {
        self.create_revision = 0;
    }

    // Param is passed by value, moved
    pub fn set_create_revision(&mut self, v: i64) {
        self.create_revision = v;
    }

    pub fn get_create_revision(&self) -> i64 {
        self.create_revision
    }

    fn get_create_revision_for_reflect(&self) -> &i64 {
        &self.create_revision
    }

    fn mut_create_revision_for_reflect(&mut self) -> &mut i64 {
        &mut self.create_revision
    }

    // int64 mod_revision = 3;

    pub fn clear_mod_revision(&mut self) {
        self.mod_revision = 0;
    }

    // Param is passed by value, moved
    pub fn set_mod_revision(&mut self, v: i64) {
        self.mod_revision = v;
    }

    pub fn get_mod_revision(&self) -> i64 {
        self.mod_revision
    }

    fn get_mod_revision_for_reflect(&self) -> &i64 {
        &self.mod_revision
    }

    fn mut_mod_revision_for_reflect(&mut self) -> &mut i64 {
        &mut self.mod_revision
    }

    // int64 version = 4;

    pub fn clear_version(&mut self) {
        self.version = 0;
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: i64) {
        self.version = v;
    }

    pub fn get_version(&self) -> i64 {
        self.version
    }

    fn get_version_for_reflect(&self) -> &i64 {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut i64 {
        &mut self.version
    }

    // bytes value = 5;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.value, ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        &self.value
    }

    fn get_value_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.value
    }

    // int64 lease = 6;

    pub fn clear_lease(&mut self) {
        self.lease = 0;
    }

    // Param is passed by value, moved
    pub fn set_lease(&mut self, v: i64) {
        self.lease = v;
    }

    pub fn get_lease(&self) -> i64 {
        self.lease
    }

    fn get_lease_for_reflect(&self) -> &i64 {
        &self.lease
    }

    fn mut_lease_for_reflect(&mut self) -> &mut i64 {
        &mut self.lease
    }
}

impl ::protobuf::Message for KeyValue {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.create_revision = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.mod_revision = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.version = tmp;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.value)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.lease = tmp;
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
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.key);
        }
        if self.create_revision != 0 {
            my_size += ::protobuf::rt::value_size(2, self.create_revision, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.mod_revision != 0 {
            my_size += ::protobuf::rt::value_size(3, self.mod_revision, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.version != 0 {
            my_size += ::protobuf::rt::value_size(4, self.version, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::bytes_size(5, &self.value);
        }
        if self.lease != 0 {
            my_size += ::protobuf::rt::value_size(6, self.lease, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_bytes(1, &self.key)?;
        }
        if self.create_revision != 0 {
            os.write_int64(2, self.create_revision)?;
        }
        if self.mod_revision != 0 {
            os.write_int64(3, self.mod_revision)?;
        }
        if self.version != 0 {
            os.write_int64(4, self.version)?;
        }
        if !self.value.is_empty() {
            os.write_bytes(5, &self.value)?;
        }
        if self.lease != 0 {
            os.write_int64(6, self.lease)?;
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

impl ::protobuf::MessageStatic for KeyValue {
    fn new() -> KeyValue {
        KeyValue::new()
    }

    fn descriptor_static(_: ::std::option::Option<KeyValue>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    KeyValue::get_key_for_reflect,
                    KeyValue::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "create_revision",
                    KeyValue::get_create_revision_for_reflect,
                    KeyValue::mut_create_revision_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "mod_revision",
                    KeyValue::get_mod_revision_for_reflect,
                    KeyValue::mut_mod_revision_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "version",
                    KeyValue::get_version_for_reflect,
                    KeyValue::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    KeyValue::get_value_for_reflect,
                    KeyValue::mut_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "lease",
                    KeyValue::get_lease_for_reflect,
                    KeyValue::mut_lease_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<KeyValue>(
                    "KeyValue",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for KeyValue {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_create_revision();
        self.clear_mod_revision();
        self.clear_version();
        self.clear_value();
        self.clear_lease();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for KeyValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KeyValue {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Event {
    // message fields
    pub field_type: Event_EventType,
    pub kv: ::protobuf::SingularPtrField<KeyValue>,
    pub prev_kv: ::protobuf::SingularPtrField<KeyValue>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Event {}

impl Event {
    pub fn new() -> Event {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Event {
        static mut instance: ::protobuf::lazy::Lazy<Event> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Event,
        };
        unsafe {
            instance.get(Event::new)
        }
    }

    // .mvccpb.Event.EventType type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = Event_EventType::PUT;
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: Event_EventType) {
        self.field_type = v;
    }

    pub fn get_field_type(&self) -> Event_EventType {
        self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &Event_EventType {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut Event_EventType {
        &mut self.field_type
    }

    // .mvccpb.KeyValue kv = 2;

    pub fn clear_kv(&mut self) {
        self.kv.clear();
    }

    pub fn has_kv(&self) -> bool {
        self.kv.is_some()
    }

    // Param is passed by value, moved
    pub fn set_kv(&mut self, v: KeyValue) {
        self.kv = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_kv(&mut self) -> &mut KeyValue {
        if self.kv.is_none() {
            self.kv.set_default();
        }
        self.kv.as_mut().unwrap()
    }

    // Take field
    pub fn take_kv(&mut self) -> KeyValue {
        self.kv.take().unwrap_or_else(|| KeyValue::new())
    }

    pub fn get_kv(&self) -> &KeyValue {
        self.kv.as_ref().unwrap_or_else(|| KeyValue::default_instance())
    }

    fn get_kv_for_reflect(&self) -> &::protobuf::SingularPtrField<KeyValue> {
        &self.kv
    }

    fn mut_kv_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<KeyValue> {
        &mut self.kv
    }

    // .mvccpb.KeyValue prev_kv = 3;

    pub fn clear_prev_kv(&mut self) {
        self.prev_kv.clear();
    }

    pub fn has_prev_kv(&self) -> bool {
        self.prev_kv.is_some()
    }

    // Param is passed by value, moved
    pub fn set_prev_kv(&mut self, v: KeyValue) {
        self.prev_kv = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_prev_kv(&mut self) -> &mut KeyValue {
        if self.prev_kv.is_none() {
            self.prev_kv.set_default();
        }
        self.prev_kv.as_mut().unwrap()
    }

    // Take field
    pub fn take_prev_kv(&mut self) -> KeyValue {
        self.prev_kv.take().unwrap_or_else(|| KeyValue::new())
    }

    pub fn get_prev_kv(&self) -> &KeyValue {
        self.prev_kv.as_ref().unwrap_or_else(|| KeyValue::default_instance())
    }

    fn get_prev_kv_for_reflect(&self) -> &::protobuf::SingularPtrField<KeyValue> {
        &self.prev_kv
    }

    fn mut_prev_kv_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<KeyValue> {
        &mut self.prev_kv
    }
}

impl ::protobuf::Message for Event {
    fn is_initialized(&self) -> bool {
        for v in &self.kv {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.prev_kv {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.field_type = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.kv)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.prev_kv)?;
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
        if self.field_type != Event_EventType::PUT {
            my_size += ::protobuf::rt::enum_size(1, self.field_type);
        }
        if let Some(ref v) = self.kv.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.prev_kv.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.field_type != Event_EventType::PUT {
            os.write_enum(1, self.field_type.value())?;
        }
        if let Some(ref v) = self.kv.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.prev_kv.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for Event {
    fn new() -> Event {
        Event::new()
    }

    fn descriptor_static(_: ::std::option::Option<Event>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Event_EventType>>(
                    "type",
                    Event::get_field_type_for_reflect,
                    Event::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<KeyValue>>(
                    "kv",
                    Event::get_kv_for_reflect,
                    Event::mut_kv_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<KeyValue>>(
                    "prev_kv",
                    Event::get_prev_kv_for_reflect,
                    Event::mut_prev_kv_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Event>(
                    "Event",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Event {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_kv();
        self.clear_prev_kv();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Event {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Event {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Event_EventType {
    PUT = 0,
    DELETE = 1,
}

impl ::protobuf::ProtobufEnum for Event_EventType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Event_EventType> {
        match value {
            0 => ::std::option::Option::Some(Event_EventType::PUT),
            1 => ::std::option::Option::Some(Event_EventType::DELETE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Event_EventType] = &[
            Event_EventType::PUT,
            Event_EventType::DELETE,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Event_EventType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Event_EventType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Event_EventType {
}

impl ::std::default::Default for Event_EventType {
    fn default() -> Self {
        Event_EventType::PUT
    }
}

impl ::protobuf::reflect::ProtobufValue for Event_EventType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0csrc/kv.proto\x12\x06mvccpb\"\xae\x01\n\x08KeyValue\x12\x10\n\x03ke\
    y\x18\x01\x20\x01(\x0cR\x03key\x12'\n\x0fcreate_revision\x18\x02\x20\x01\
    (\x03R\x0ecreateRevision\x12!\n\x0cmod_revision\x18\x03\x20\x01(\x03R\
    \x0bmodRevision\x12\x18\n\x07version\x18\x04\x20\x01(\x03R\x07version\
    \x12\x14\n\x05value\x18\x05\x20\x01(\x0cR\x05value\x12\x14\n\x05lease\
    \x18\x06\x20\x01(\x03R\x05lease\"\xa3\x01\n\x05Event\x12+\n\x04type\x18\
    \x01\x20\x01(\x0e2\x17.mvccpb.Event.EventTypeR\x04type\x12\x20\n\x02kv\
    \x18\x02\x20\x01(\x0b2\x10.mvccpb.KeyValueR\x02kv\x12)\n\x07prev_kv\x18\
    \x03\x20\x01(\x0b2\x10.mvccpb.KeyValueR\x06prevKv\"\x20\n\tEventType\x12\
    \x07\n\x03PUT\x10\0\x12\n\n\x06DELETE\x10\x01b\x06proto3\
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
