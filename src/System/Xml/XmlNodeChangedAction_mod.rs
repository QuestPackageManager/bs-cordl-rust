#[cfg(feature = "System+Xml+XmlNodeChangedAction")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmlNodeChangedAction {
    Change = 2i32,
    Insert = 0i32,
    Remove = 1i32,
}
#[cfg(feature = "System+Xml+XmlNodeChangedAction")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlNodeChangedAction =>
    "System.Xml"."XmlNodeChangedAction"
);
