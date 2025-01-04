#[cfg(feature = "System+Xml+XmlStandalone")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum XmlStandalone {
    #[default]
    No = 2i32,
    Omit = 0i32,
    Yes = 1i32,
}
#[cfg(feature = "System+Xml+XmlStandalone")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlStandalone => "System.Xml"
    ."XmlStandalone"
);
