#[cfg(feature = "Oculus+Platform+AchievementType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AchievementType {
    Bitfield = 2i32,
    Count = 3i32,
    Simple = 1i32,
    Unknown = 0i32,
}
#[cfg(feature = "Oculus+Platform+AchievementType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::AchievementType =>
    "Oculus.Platform"."AchievementType"
);
