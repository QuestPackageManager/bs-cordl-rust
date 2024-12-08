#[cfg(feature = "System+Net+TriState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriState {
    False = 0i32,
    True = 1i32,
    Unspecified = -1i32,
}
#[cfg(feature = "System+Net+TriState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::TriState => "System.Net"."TriState"
);
