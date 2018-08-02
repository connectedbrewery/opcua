//! Contains data types and enumerations for OPC UA.
//!
//! 1. All of the built-in data types described in OPC Part 6 Chapter 5 that are encodable
//! 2. All of the standard data types described in OPC Part 3 Chapter 8 (if not covered by 1.)
//! 3. Autogenerated data types and request / responses as described in OPC Part 4

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate byteorder;
extern crate chrono;
extern crate regex;
extern crate ring;
extern crate uuid;
extern crate url as url_external;
extern crate base64;
#[cfg(test)]
extern crate serde_json;

#[macro_export]
macro_rules! supported_message_as {
    ($v: expr, $i: ident) => {
        if let SupportedMessage::$i(value) = $v {
            value
        } else {
            panic!("Failed to get a supported message of type {}", stringify!($i));
        }
    }
}

#[macro_export]
macro_rules! supported_message_as_ref {
    ($v: expr, $i: ident) => {
        if let SupportedMessage::$i(ref value) = $v {
            value
        } else {
            panic!("Failed to get a supported message of type {}", stringify!($i));
        }
    }
}

#[macro_export]
macro_rules! supported_message_as_ref_mut {
    ($v: expr, $i: ident) => {
        if let SupportedMessage::$i(ref mut v) = $v {
            v
        } else {
            panic!("Failed to get a supported message of type {}", stringify!($i));
        }
    }
}

///Contains constants recognized by OPC UA clients and servers to describe various protocols and
/// profiles used during communication and encryption.
pub mod profiles {
    pub const TRANSPORT_PROFILE_URI_BINARY: &'static str = "http://opcfoundation.org/UA-Profile/Transport/uatcp-uasc-uabinary";

    pub const SECURITY_USER_TOKEN_POLICY_ANONYMOUS: &'static str = "http://opcfoundation.org/UA-Profile/Security/UserToken/Anonymous";
    pub const SECURITY_USER_TOKEN_POLICY_USERPASS: &'static str = "http://opcfoundation.org/UA-Profile/ Security/UserToken-Server/UserNamePassword";
}

pub mod constants {
    /// Default OPC UA port number. Used by a discovery server. Other servers would normally run
    /// on a different port. So OPC UA for Rust does not use this nr by default but it is used
    /// implicitly in opc.tcp:// urls and elsewhere.
    pub const DEFAULT_OPC_UA_SERVER_PORT: u16 = 4840;
    /// Maximum number of elements in an array
    pub const MAX_ARRAY_LENGTH: u32 = 1000;
    /// Maximum size of a string in chars
    pub const MAX_STRING_LENGTH: u32 = 65536;
    /// Maximum size of a byte string in bytes
    pub const MAX_BYTE_STRING_LENGTH: u32 = 65536;
    /// Maximum size of a certificate to send
    pub const MAX_CERTIFICATE_LENGTH: u32 = 32768;
}

/// Bits for status code
bitflags! {
    pub struct StatusCodeBits: u32 {
        // Mask for the status code section
        const STATUS_MASK = 0xffff0000;
        // Mask for the bits section
        const BIT_MASK = 0x0000ffff;

        // Historian bits 0:4
        const HISTORICAL_RAW          = 0b00000000_00000000;
        const HISTORICAL_CALCULATED   = 0b00000000_00000001;
        const HISTORICAL_INTERPOLATED = 0b00000000_00000010;
        const HISTORICAL_RESERVED     = 0b00000000_00000011;
        const HISTORICAL_PARTIAL      = 0b00000000_00000100;
        const HISTORICAL_EXTRA_DATA   = 0b00000000_00001000;
        const HISTORICAL_MULTI_VALUE  = 0b00000000_00010000;
        // Overflow bit 7
        const OVERFLOW                = 0b00000000_10000000;
        // Limit bits 8:9
        const LIMIT_LOW               = 0b00000001_00000000;
        const LIMIT_HIGH              = 0b00000010_00000000;
        const LIMIT_CONSTANT          = 0b00000011_00000000;
        // Info type bits 10:11
        const LIMIT_DATA_VALUE        = 0b00000010_00000000;
        // Semantics changed bit 14
        const SEMANTICS_CHANGED       = 0b01000000_00000000;
        // Semantics changed bit 15
        const STRUCTURE_CHANGED       = 0b10000000_00000000;
    }
}

/// Write mask bits
bitflags! {
    pub struct WriteMask: u32 {
        /// Indicates if the AccessLevel Attribute is writable.
        const ACCESS_LEVEL = 1 << 0;
        /// Indicates if the ArrayDimensions Attribute is writable.
        const ARRAY_DIMENSTIONS = 1 << 1;
        ///Indicates if the BrowseName Attribute is writable.
        const BROWSE_NAME = 1 << 2;
        /// Indicates if the ContainsNoLoops Attribute is writable.
        const CONTAINS_NO_LOOPS = 1 << 3;
        /// Indicates if the DataType Attribute is writable.
        const DATA_TYPE = 1 << 4;
        /// Indicates if the Description Attribute is writable.
        const DESCRIPTION = 1 << 5;
        /// Indicates if the DisplayName Attribute is writable.
        const DISPLAY_NAME = 1 << 6;
        /// Indicates if the EventNotifier Attribute is writable.
        const EVENT_NOTIFIER = 1 << 7;
        /// Indicates if the Executable Attribute is writable.
        const EXECUTABLE = 1 << 8;
        /// Indicates if the Historizing Attribute is writable.
        const HISTORIZING = 1 << 9;
        /// Indicates if the InverseName Attribute is writable.
        const INVERSE_NAME = 1 << 10;
        /// Indicates if the IsAbstract Attribute is writable.
        const IS_ABSTRACT = 1 << 11;
        /// Indicates if the MinimumSamplingInterval Attribute is writable.
        const MINIMUM_SAMPLING_INTERVAL = 1 << 12;
        /// Indicates if the NodeClass Attribute is writable.
        const NODE_CLASS = 1 << 13;
        /// Indicates if the NodeId Attribute is writable.
        const NODE_ID = 1 << 14;
        /// Indicates if the Symmetric Attribute is writable.
        const SYMMETRIC = 1 << 15;
        /// Indicates if the UserAccessLevel Attribute is writable.
        const USER_ACCESS_LEVEL = 1 << 16;
        /// Indicates if the UserExecutable Attribute is writable.
        const USER_EXECUTABLE = 1 << 17;
        /// Indicates if the UserWriteMask Attribute is writable.
        const USER_WRITE_MASK = 1 << 18;
        /// Indicates if the ValueRank Attribute is writable.
        const VALUE_RANK = 1 << 19;
        /// Indicates if the WriteMask Attribute is writable.
        const WRITE_MASK = 1 << 20;
        /// Indicates if the Value Attribute is writable for a VariableType. It does not apply for Variables
        /// since this is handled by the AccessLevel and UserAccessLevel Attributes for the Variable.
        /// For Variables this bit shall be set to 0.
        const VALUE_FOR_VARIABLE_TYPE = 1 << 21;
    }
}

pub mod encoding;
pub mod basic_types;
pub mod string;
pub mod extension_object;
pub mod byte_string;
pub mod data_value;
pub mod date_time;
pub mod guid;
pub mod node_id;
pub mod variant;
pub mod data_types;
pub mod notification_message;
pub mod attribute;
pub mod supported_message;
pub mod numeric_range;
pub mod url;
pub mod argument;

pub use encoding::*;
pub use basic_types::*;
pub use string::*;
pub use extension_object::*;
pub use byte_string::*;
pub use data_value::*;
pub use date_time::*;
pub use guid::*;
pub use node_id::*;
pub use variant::*;
pub use data_types::*;
pub use attribute::*;
pub use supported_message::*;
pub use numeric_range::*;
pub use url::*;
pub use argument::*;

// These mods are not use'd into this mod - too many types
pub mod service_types;
pub mod node_ids;
pub mod status_codes;

#[cfg(test)]
mod tests;
