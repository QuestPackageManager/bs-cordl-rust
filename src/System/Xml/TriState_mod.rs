#[cfg(feature = "System+Xml+TriState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriState {
    False = 0i32,
    True = 1i32,
    Unknown = -1i32,
}
#[cfg(feature = "System+Xml+TriState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::TriState => "System.Xml"."TriState"
);
