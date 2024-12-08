#[cfg(feature = "System+Runtime+InteropServices+CharSet")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharSet {
    Ansi = 2i32,
    Auto = 4i32,
    None = 1i32,
    Unicode = 3i32,
}
#[cfg(feature = "System+Runtime+InteropServices+CharSet")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::InteropServices::CharSet =>
    "System.Runtime.InteropServices"."CharSet"
);
