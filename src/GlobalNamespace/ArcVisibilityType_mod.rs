#[cfg(feature = "ArcVisibilityType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArcVisibilityType {
    High = 3i32,
    Low = 1i32,
    None = 0i32,
    Standard = 2i32,
}
#[cfg(feature = "ArcVisibilityType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for ArcVisibilityType => ""."ArcVisibilityType"
);
