#[cfg(feature = "System+Xml+Schema+XmlSchemaValidationFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum XmlSchemaValidationFlags {
    #[default]
    AllowXmlAttributes = 16i32,
    None = 0i32,
    ProcessIdentityConstraints = 8i32,
    ProcessInlineSchema = 1i32,
    ProcessSchemaLocation = 2i32,
    ReportValidationWarnings = 4i32,
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaValidationFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XmlSchemaValidationFlags =>
    "System.Xml.Schema"."XmlSchemaValidationFlags"
);
