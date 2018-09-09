//! This library helps you to DER-en-/-decode various types. It provides some traits to convert
//! between encoded data, DER-objects and native types as well as some trait-implementations for
//! common types. There is also a macro ([`asn1_der_impl!`](macro.asn1_der_impl.html)) that helps
//! you to implement the traits for your structs.
//!
//! The following types have built-in support:
//!  - [`DerObject`](der/struct.DerObject.html): A generic DER-object-wrapper that can hold any DER-object
//!  - `()`: The ASN.1-NULL-type
//!  - `bool`: The ASN.1-BOOLEAN-type
//!  - `Vec<u8>`: The ASN.1-OctetString-type
//!  - `String`: The ASN.1-UTF8String-type
//!  - `u64`: The ASN.1-INTEGER-type (within `0..u64::MAX`)
//!  - `Vec<DerObject>`: The ASN.1-SEQUENCE-type
//!  - `Vec<T>`: The ASN.1-SEQUENCE-type for sequences that contain only one type `T` (e.g. `Vec<String>` for a sequence
//!    that contains only UTF8Strings)


#[macro_use] extern crate etrace;

#[doc(hidden)] #[macro_use] pub mod macros;
/// Contains various DER-conversion-traits
pub mod traits;
/// Contains a generic ASN.1-DER-object-implementation
pub mod der;
/// Contains DER-conversion-trait implementations for some native types
pub mod type_impls;
/// Contains an experimental en-/decode implementation for map types
#[cfg(feature="map")] pub mod map;

pub use etrace::Error;
pub use traits::{ IntoDerObject, FromDerObject, FromDerEncoded, IntoDerEncoded };
pub use der::DerObject;


#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Asn1DerError {
	/// There are not enough bytes available to decode the DER-object or the resulting object does
	/// not match the expectations (e.g. a sequence with missing elements)
	NotEnoughBytes,
	/// The tag does not match the DER-object-type
	InvalidTag,
	/// The encoding does not conform to the DER-standard
	InvalidEncoding,
	/// The element might be valid but is not supported by this implementation
	Unsupported
}
unsafe impl Send for Asn1DerError {}
pub type Result<T> = std::result::Result<T, Error<Asn1DerError>>;