#[cfg(feature = "System+Xml+AttributeProperties")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AttributeProperties {
    #[default]
    BOOLEAN = 33555458u32,
    DEFAULT = 67240192u32,
    NAME = 67239940u32,
    URI = 262657u32,
}
#[cfg(feature = "System+Xml+AttributeProperties")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::AttributeProperties => "System.Xml"
    ."AttributeProperties"
);
