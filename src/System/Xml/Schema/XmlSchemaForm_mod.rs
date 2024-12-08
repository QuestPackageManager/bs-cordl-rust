#[cfg(feature = "System+Xml+Schema+XmlSchemaForm")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmlSchemaForm {
    None = 0i32,
    Qualified = 1i32,
    Unqualified = 2i32,
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaForm")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XmlSchemaForm =>
    "System.Xml.Schema"."XmlSchemaForm"
);
