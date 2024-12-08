#[cfg(feature = "System+Xml+Schema+XmlSchemaUse")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmlSchemaUse {
    None = 0i32,
    Optional = 1i32,
    Prohibited = 2i32,
    Required = 3i32,
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaUse")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XmlSchemaUse =>
    "System.Xml.Schema"."XmlSchemaUse"
);
