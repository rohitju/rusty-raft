// This file is generated. Do not edit

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct VoteRequest {
    // message fields
    candidateId: ::protobuf::SingularField<::std::string::String>,
    term: ::std::option::Option<i32>,
    lastLogIndex: ::std::option::Option<i32>,
    lastLogterm: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl VoteRequest {
    pub fn new() -> VoteRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VoteRequest {
        static mut instance: ::protobuf::lazy::Lazy<VoteRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VoteRequest,
        };
        unsafe {
            instance.get(|| {
                VoteRequest {
                    candidateId: ::protobuf::SingularField::none(),
                    term: ::std::option::Option::None,
                    lastLogIndex: ::std::option::Option::None,
                    lastLogterm: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string candidateId = 1;

    pub fn clear_candidateId(&mut self) {
        self.candidateId.clear();
    }

    pub fn has_candidateId(&self) -> bool {
        self.candidateId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_candidateId(&mut self, v: ::std::string::String) {
        self.candidateId = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_candidateId<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.candidateId.is_none() {
            self.candidateId.set_default();
        };
        self.candidateId.as_mut().unwrap()
    }

    // Take field
    pub fn take_candidateId(&mut self) -> ::std::string::String {
        self.candidateId.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_candidateId<'a>(&'a self) -> &'a str {
        match self.candidateId.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required int32 term = 2;

    pub fn clear_term(&mut self) {
        self.term = ::std::option::Option::None;
    }

    pub fn has_term(&self) -> bool {
        self.term.is_some()
    }

    // Param is passed by value, moved
    pub fn set_term(&mut self, v: i32) {
        self.term = ::std::option::Option::Some(v);
    }

    pub fn get_term<'a>(&self) -> i32 {
        self.term.unwrap_or(0)
    }

    // required int32 lastLogIndex = 3;

    pub fn clear_lastLogIndex(&mut self) {
        self.lastLogIndex = ::std::option::Option::None;
    }

    pub fn has_lastLogIndex(&self) -> bool {
        self.lastLogIndex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lastLogIndex(&mut self, v: i32) {
        self.lastLogIndex = ::std::option::Option::Some(v);
    }

    pub fn get_lastLogIndex<'a>(&self) -> i32 {
        self.lastLogIndex.unwrap_or(0)
    }

    // required int32 lastLogterm = 4;

    pub fn clear_lastLogterm(&mut self) {
        self.lastLogterm = ::std::option::Option::None;
    }

    pub fn has_lastLogterm(&self) -> bool {
        self.lastLogterm.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lastLogterm(&mut self, v: i32) {
        self.lastLogterm = ::std::option::Option::Some(v);
    }

    pub fn get_lastLogterm<'a>(&self) -> i32 {
        self.lastLogterm.unwrap_or(0)
    }
}

impl ::protobuf::Message for VoteRequest {
    fn is_initialized(&self) -> bool {
        if self.candidateId.is_none() {
            return false;
        };
        if self.term.is_none() {
            return false;
        };
        if self.lastLogIndex.is_none() {
            return false;
        };
        if self.lastLogterm.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.candidateId.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.term = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.lastLogIndex = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.lastLogterm = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.candidateId.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.term.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.lastLogIndex.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.lastLogterm.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.candidateId.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.term {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.lastLogIndex {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.lastLogterm {
            try!(os.write_int32(4, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<VoteRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for VoteRequest {
    fn new() -> VoteRequest {
        VoteRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<VoteRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "candidateId",
                    VoteRequest::has_candidateId,
                    VoteRequest::get_candidateId,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "term",
                    VoteRequest::has_term,
                    VoteRequest::get_term,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "lastLogIndex",
                    VoteRequest::has_lastLogIndex,
                    VoteRequest::get_lastLogIndex,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "lastLogterm",
                    VoteRequest::has_lastLogterm,
                    VoteRequest::get_lastLogterm,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VoteRequest>(
                    "VoteRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VoteRequest {
    fn clear(&mut self) {
        self.clear_candidateId();
        self.clear_term();
        self.clear_lastLogIndex();
        self.clear_lastLogterm();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for VoteRequest {
    fn eq(&self, other: &VoteRequest) -> bool {
        self.candidateId == other.candidateId &&
        self.term == other.term &&
        self.lastLogIndex == other.lastLogIndex &&
        self.lastLogterm == other.lastLogterm &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for VoteRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct VoteReply {
    // message fields
    term: ::std::option::Option<i32>,
    granted: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl VoteReply {
    pub fn new() -> VoteReply {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VoteReply {
        static mut instance: ::protobuf::lazy::Lazy<VoteReply> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VoteReply,
        };
        unsafe {
            instance.get(|| {
                VoteReply {
                    term: ::std::option::Option::None,
                    granted: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int32 term = 1;

    pub fn clear_term(&mut self) {
        self.term = ::std::option::Option::None;
    }

    pub fn has_term(&self) -> bool {
        self.term.is_some()
    }

    // Param is passed by value, moved
    pub fn set_term(&mut self, v: i32) {
        self.term = ::std::option::Option::Some(v);
    }

    pub fn get_term<'a>(&self) -> i32 {
        self.term.unwrap_or(0)
    }

    // required bool granted = 2;

    pub fn clear_granted(&mut self) {
        self.granted = ::std::option::Option::None;
    }

    pub fn has_granted(&self) -> bool {
        self.granted.is_some()
    }

    // Param is passed by value, moved
    pub fn set_granted(&mut self, v: bool) {
        self.granted = ::std::option::Option::Some(v);
    }

    pub fn get_granted<'a>(&self) -> bool {
        self.granted.unwrap_or(false)
    }
}

impl ::protobuf::Message for VoteReply {
    fn is_initialized(&self) -> bool {
        if self.term.is_none() {
            return false;
        };
        if self.granted.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.term = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.granted = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.term.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.granted.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.term {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.granted {
            try!(os.write_bool(2, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<VoteReply>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for VoteReply {
    fn new() -> VoteReply {
        VoteReply::new()
    }

    fn descriptor_static(_: ::std::option::Option<VoteReply>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "term",
                    VoteReply::has_term,
                    VoteReply::get_term,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "granted",
                    VoteReply::has_granted,
                    VoteReply::get_granted,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VoteReply>(
                    "VoteReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VoteReply {
    fn clear(&mut self) {
        self.clear_term();
        self.clear_granted();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for VoteReply {
    fn eq(&self, other: &VoteReply) -> bool {
        self.term == other.term &&
        self.granted == other.granted &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for VoteReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x12, 0x76, 0x6f, 0x74, 0x65, 0x5f, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x22, 0x5b, 0x0a, 0x0b, 0x56, 0x6f, 0x74, 0x65, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x12, 0x13, 0x0a, 0x0b, 0x63, 0x61, 0x6e, 0x64, 0x69, 0x64, 0x61, 0x74, 0x65,
    0x49, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x65, 0x72, 0x6d,
    0x18, 0x02, 0x20, 0x02, 0x28, 0x05, 0x12, 0x14, 0x0a, 0x0c, 0x6c, 0x61, 0x73, 0x74, 0x4c, 0x6f,
    0x67, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x03, 0x20, 0x02, 0x28, 0x05, 0x12, 0x13, 0x0a, 0x0b,
    0x6c, 0x61, 0x73, 0x74, 0x4c, 0x6f, 0x67, 0x74, 0x65, 0x72, 0x6d, 0x18, 0x04, 0x20, 0x02, 0x28,
    0x05, 0x22, 0x2a, 0x0a, 0x09, 0x56, 0x6f, 0x74, 0x65, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x12, 0x0c,
    0x0a, 0x04, 0x74, 0x65, 0x72, 0x6d, 0x18, 0x01, 0x20, 0x02, 0x28, 0x05, 0x12, 0x0f, 0x0a, 0x07,
    0x67, 0x72, 0x61, 0x6e, 0x74, 0x65, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x08, 0x4a, 0xd6, 0x03,
    0x0a, 0x06, 0x12, 0x04, 0x01, 0x00, 0x0c, 0x01, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04,
    0x01, 0x00, 0x07, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x01, 0x08, 0x13,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x03, 0x04, 0x24, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x03, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x03, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x03, 0x14, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x03, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x04,
    0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x04, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x04, 0x0d, 0x12, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x04, 0x13, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x04, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x02, 0x12, 0x03, 0x05, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04,
    0x12, 0x03, 0x05, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x05, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x05, 0x13,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x05, 0x22, 0x23, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x06, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x06, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x05, 0x12, 0x03, 0x06, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x06, 0x13, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x06, 0x21, 0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x09, 0x00, 0x0c, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x09, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0a, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x0a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x0a, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x0a, 0x13, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0a, 0x1a,
    0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x04, 0x1e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0b, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x0b, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x0b, 0x1c, 0x1d,
];

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
