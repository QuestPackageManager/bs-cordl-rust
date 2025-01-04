#[cfg(feature = "System+Xml+XmlOutputMethod")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum XmlOutputMethod {
    #[default]
    AutoDetect = 3i32,
    Html = 1i32,
    Text = 2i32,
    Xml = 0i32,
}
#[cfg(feature = "System+Xml+XmlOutputMethod")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlOutputMethod => "System.Xml"
    ."XmlOutputMethod"
);
