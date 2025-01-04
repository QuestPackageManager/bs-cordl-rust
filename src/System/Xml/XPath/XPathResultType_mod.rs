#[cfg(feature = "System+Xml+XPath+XPathResultType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum XPathResultType {
    #[default]
    Any = 5i32,
    Boolean = 2i32,
    Error = 6i32,
    Navigator = 1i32,
    NodeSet = 3i32,
    Number = 0i32,
}
#[cfg(feature = "System+Xml+XPath+XPathResultType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XPath::XPathResultType =>
    "System.Xml.XPath"."XPathResultType"
);
