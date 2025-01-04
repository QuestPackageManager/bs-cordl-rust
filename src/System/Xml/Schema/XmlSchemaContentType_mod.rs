#[cfg(feature = "System+Xml+Schema+XmlSchemaContentType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum XmlSchemaContentType {
    #[default]
    ElementOnly = 2i32,
    Empty = 1i32,
    Mixed = 3i32,
    TextOnly = 0i32,
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaContentType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XmlSchemaContentType =>
    "System.Xml.Schema"."XmlSchemaContentType"
);
