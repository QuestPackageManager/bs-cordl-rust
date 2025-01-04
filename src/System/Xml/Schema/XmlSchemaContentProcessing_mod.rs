#[cfg(feature = "System+Xml+Schema+XmlSchemaContentProcessing")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum XmlSchemaContentProcessing {
    #[default]
    Lax = 2i32,
    None = 0i32,
    Skip = 1i32,
    Strict = 3i32,
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaContentProcessing")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XmlSchemaContentProcessing
    => "System.Xml.Schema"."XmlSchemaContentProcessing"
);
