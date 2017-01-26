// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

use types::*;
use services::*;

/// Creates a secure channel with a server.
#[derive(Debug, Clone, PartialEq)]
pub struct OpenSecureChannelResponse {
    pub response_header: ResponseHeader,
    pub server_protocol_version: UInt32,
    pub security_token: ChannelSecurityToken,
    pub server_nonce: ByteString,
}

impl MessageInfo for OpenSecureChannelResponse {
    fn object_id(&self) -> ObjectId {
        ObjectId::OpenSecureChannelResponse_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<OpenSecureChannelResponse> for OpenSecureChannelResponse {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.response_header.byte_len();
        size += self.server_protocol_version.byte_len();
        size += self.security_token.byte_len();
        size += self.server_nonce.byte_len();
        size
    }
    
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.response_header.encode(stream)?;
        size += self.server_protocol_version.encode(stream)?;
        size += self.security_token.encode(stream)?;
        size += self.server_nonce.encode(stream)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let response_header = ResponseHeader::decode(stream)?;
        let server_protocol_version = UInt32::decode(stream)?;
        let security_token = ChannelSecurityToken::decode(stream)?;
        let server_nonce = ByteString::decode(stream)?;
        Ok(OpenSecureChannelResponse {
            response_header: response_header,
            server_protocol_version: server_protocol_version,
            security_token: security_token,
            server_nonce: server_nonce,
        })
    }
}
