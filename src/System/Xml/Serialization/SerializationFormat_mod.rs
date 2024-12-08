#[cfg(feature = "System+Xml+Serialization+SerializationFormat")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SerializationFormat {
    Encoded = 0i32,
    Literal = 1i32,
}
#[cfg(feature = "System+Xml+Serialization+SerializationFormat")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Serialization::SerializationFormat
    => "System.Xml.Serialization"."SerializationFormat"
);
