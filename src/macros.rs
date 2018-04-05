use super::{ Error, Asn1DerError, DerObject, FromDerObject };

#[inline] #[doc(hidden)]
pub fn parse_next<T: FromDerObject>(iter: &mut Iterator<Item = &DerObject>) -> Result<T, Error<Asn1DerError>> {
	let der_object = some_or!(iter.next(), throw_err!(Asn1DerError::NotEnoughBytes));
	Ok(try_err!(T::from_der_object(der_object.clone())))
}

/// This macro helps you to implement the DER-conversion-traits on your own structs
///
/// Usage: `asn1_der_impl!(StructName{ field0_name, field1_name, ..., fieldN_name })`
///
/// Example:
///
/// ```ignore
/// struct Address {
/// 	street: String,
/// 	house_number: u64,
/// 	postal_code: u64,
/// 	state: String,
/// 	country: String
/// }
/// asn1_der_impl!(Address{ street, house_number, postal_code, state, country }); // Now our struct supports all DER-conversion-traits
///
/// struct Customer {
/// 	name: String,
/// 	e_mail_address: String,
/// 	postal_address: Address
/// }
/// asn1_der_impl!(Customer{ name, e_mail_address, postal_address }); // Now this struct supports all DER-conversion-traits too! It's only necessary that all fields implement the DER-conversion-traits
///
/// // Serialization:
/// let encoded = my_customer.clone().into_der_encoded(); // This returns a vector containing the DER-encoded representation of this customer (a sequence containing the struct's fields)
///
/// // Parsing:
/// let my_decoded_customer = Customer::from_der_encoded(encoded).unwrap(); // This returns our customer (if the data is valid)
/// ```
#[macro_export]
macro_rules! asn1_der_impl {
    ($struct_name:ident { $($field_name:ident),+ }) => {
    	impl $crate::FromDerObject for $struct_name {
    		fn from_der_object(der_object: $crate::DerObject) -> Result<Self, $crate::Error<$crate::Asn1DerError>> {
    			let seq = try_err!(Vec::<$crate::DerObject>::from_der_object(der_object));
    			let mut seq_iter = seq.iter();
    			
    			Ok($struct_name {
    				$($field_name: try_err!($crate::macros::parse_next(&mut seq_iter))),+
    			})
    		}
    	}
    	
    	impl $crate::IntoDerObject for $struct_name {
    		fn into_der_object(self) -> $crate::DerObject {
    			let mut seq = Vec::new();
    			$(seq.push(self.$field_name.into_der_object());)+;
    			seq.into_der_object()
    		}
    	}
    };
}