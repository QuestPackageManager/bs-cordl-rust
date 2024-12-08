#[cfg(feature = "System+Xml+Schema+SchemaType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchemaType {
    DTD = 1i32,
    None = 0i32,
    XDR = 2i32,
    XSD = 3i32,
}
#[cfg(feature = "System+Xml+Schema+SchemaType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::SchemaType =>
    "System.Xml.Schema"."SchemaType"
);
