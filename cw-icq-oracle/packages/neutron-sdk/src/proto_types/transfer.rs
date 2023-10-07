// This file is generated by rust-protobuf 3.3.0. Do not edit
// .proto file is parsed by protoc --rust-out=...
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `transfer/v1/transfer.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_3_0;

///  MsgTransferResponse is the modified response type for
///  ibc-go MsgTransfer.
// @@protoc_insertion_point(message:neutron.transfer.MsgTransferResponse)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MsgTransferResponse {
    // message fields
    ///  channel's sequence_id for outgoing ibc packet. Unique per a channel.
    // @@protoc_insertion_point(field:neutron.transfer.MsgTransferResponse.sequence_id)
    pub sequence_id: u64,
    ///  channel src channel on neutron side trasaction was submitted from
    // @@protoc_insertion_point(field:neutron.transfer.MsgTransferResponse.channel)
    pub channel: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:neutron.transfer.MsgTransferResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MsgTransferResponse {
    fn default() -> &'a MsgTransferResponse {
        <MsgTransferResponse as ::protobuf::Message>::default_instance()
    }
}

impl MsgTransferResponse {
    pub fn new() -> MsgTransferResponse {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "sequence_id",
            |m: &MsgTransferResponse| { &m.sequence_id },
            |m: &mut MsgTransferResponse| { &mut m.sequence_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "channel",
            |m: &MsgTransferResponse| { &m.channel },
            |m: &mut MsgTransferResponse| { &mut m.channel },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MsgTransferResponse>(
            "MsgTransferResponse",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MsgTransferResponse {
    const NAME: &'static str = "MsgTransferResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.sequence_id = is.read_uint64()?;
                },
                18 => {
                    self.channel = is.read_string()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.sequence_id != 0 {
            my_size += ::protobuf::rt::uint64_size(1, self.sequence_id);
        }
        if !self.channel.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.channel);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.sequence_id != 0 {
            os.write_uint64(1, self.sequence_id)?;
        }
        if !self.channel.is_empty() {
            os.write_string(2, &self.channel)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> MsgTransferResponse {
        MsgTransferResponse::new()
    }

    fn clear(&mut self) {
        self.sequence_id = 0;
        self.channel.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MsgTransferResponse {
        static instance: MsgTransferResponse = MsgTransferResponse {
            sequence_id: 0,
            channel: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MsgTransferResponse {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MsgTransferResponse").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MsgTransferResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MsgTransferResponse {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1atransfer/v1/transfer.proto\x12\x10neutron.transfer\"P\n\x13MsgTran\
    sferResponse\x12\x1f\n\x0bsequence_id\x18\x01\x20\x01(\x04R\nsequenceId\
    \x12\x18\n\x07channel\x18\x02\x20\x01(\tR\x07channelJ\xfd\x02\n\x06\x12\
    \x04\0\0\x0b\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\
    \x02\0\x19\nX\n\x02\x04\0\x12\x04\x06\0\x0b\x01\x1aL\x20MsgTransferRespo\
    nse\x20is\x20the\x20modified\x20response\x20type\x20for\n\x20ibc-go\x20M\
    sgTransfer.\n\n\n\n\x03\x04\0\x01\x12\x03\x06\x08\x1b\nS\n\x04\x04\0\x02\
    \0\x12\x03\x08\x02\x19\x1aF\x20channel's\x20sequence_id\x20for\x20outgoi\
    ng\x20ibc\x20packet.\x20Unique\x20per\x20a\x20channel.\n\n\x0c\n\x05\x04\
    \0\x02\0\x05\x12\x03\x08\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x08\
    \t\x14\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x08\x17\x18\nP\n\x04\x04\0\
    \x02\x01\x12\x03\n\x02\x15\x1aC\x20channel\x20src\x20channel\x20on\x20ne\
    utron\x20side\x20trasaction\x20was\x20submitted\x20from\n\n\x0c\n\x05\
    \x04\0\x02\x01\x05\x12\x03\n\x02\x08\n\x0c\n\x05\x04\0\x02\x01\x01\x12\
    \x03\n\t\x10\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\n\x13\x14b\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(MsgTransferResponse::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}