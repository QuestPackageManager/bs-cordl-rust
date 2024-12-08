#[cfg(feature = "System+DateTimeKind")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateTimeKind {
    Local = 2i32,
    Unspecified = 0i32,
    Utc = 1i32,
}
#[cfg(feature = "System+DateTimeKind")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::DateTimeKind => "System"."DateTimeKind"
);
