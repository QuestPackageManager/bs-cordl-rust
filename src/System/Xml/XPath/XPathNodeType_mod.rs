#[cfg(feature = "System+Xml+XPath+XPathNodeType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum XPathNodeType {
    #[default]
    All = 9i32,
    Attribute = 2i32,
    Comment = 8i32,
    Element = 1i32,
    Namespace = 3i32,
    ProcessingInstruction = 7i32,
    Root = 0i32,
    SignificantWhitespace = 5i32,
    Text = 4i32,
    Whitespace = 6i32,
}
#[cfg(feature = "System+Xml+XPath+XPathNodeType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XPath::XPathNodeType =>
    "System.Xml.XPath"."XPathNodeType"
);
