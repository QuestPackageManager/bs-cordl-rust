#[cfg(feature = "System+Xml+Serialization+SchemaTypes")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchemaTypes {
    Array = 3i32,
    Class = 4i32,
    Enum = 2i32,
    NotSet = 0i32,
    Primitive = 1i32,
    Void = 7i32,
    XmlNode = 6i32,
    XmlSerializable = 5i32,
}
#[cfg(feature = "System+Xml+Serialization+SchemaTypes")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Serialization::SchemaTypes =>
    "System.Xml.Serialization"."SchemaTypes"
);
