#[cfg(feature = "System+Xml+ValidationType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationType {
    Auto = 1i32,
    DTD = 2i32,
    None = 0i32,
    Schema = 4i32,
    XDR = 3i32,
}
#[cfg(feature = "System+Xml+ValidationType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::ValidationType => "System.Xml"
    ."ValidationType"
);
