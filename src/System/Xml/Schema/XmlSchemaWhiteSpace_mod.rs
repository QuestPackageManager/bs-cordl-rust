#[cfg(feature = "System+Xml+Schema+XmlSchemaWhiteSpace")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum XmlSchemaWhiteSpace {
    #[default]
    Collapse = 2i32,
    Preserve = 0i32,
    Replace = 1i32,
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaWhiteSpace")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XmlSchemaWhiteSpace =>
    "System.Xml.Schema"."XmlSchemaWhiteSpace"
);
