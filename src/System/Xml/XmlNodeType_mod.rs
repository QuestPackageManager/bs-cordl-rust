#[cfg(feature = "System+Xml+XmlNodeType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum XmlNodeType {
    #[default]
    Attribute = 2i32,
    CDATA = 4i32,
    Comment = 8i32,
    Document = 9i32,
    DocumentFragment = 11i32,
    DocumentType = 10i32,
    Element = 1i32,
    EndElement = 15i32,
    EndEntity = 16i32,
    Entity = 6i32,
    EntityReference = 5i32,
    None = 0i32,
    Notation = 12i32,
    ProcessingInstruction = 7i32,
    SignificantWhitespace = 14i32,
    Text = 3i32,
    Whitespace = 13i32,
    XmlDeclaration = 17i32,
}
#[cfg(feature = "System+Xml+XmlNodeType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlNodeType => "System.Xml"
    ."XmlNodeType"
);
