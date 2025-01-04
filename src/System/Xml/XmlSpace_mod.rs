#[cfg(feature = "System+Xml+XmlSpace")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum XmlSpace {
    #[default]
    Default = 1i32,
    None = 0i32,
    Preserve = 2i32,
}
#[cfg(feature = "System+Xml+XmlSpace")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlSpace => "System.Xml"."XmlSpace"
);
