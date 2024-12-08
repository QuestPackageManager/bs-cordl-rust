#[cfg(feature = "System+Xml+Formatting")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Formatting {
    Indented = 1i32,
    None = 0i32,
}
#[cfg(feature = "System+Xml+Formatting")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Formatting => "System.Xml"
    ."Formatting"
);
