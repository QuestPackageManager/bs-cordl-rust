#[cfg(feature = "Oculus+Platform+LeaderboardFilterType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LeaderboardFilterType {
    #[default]
    Friends = 1i32,
    None = 0i32,
    Unknown = 2i32,
    UserIds = 3i32,
}
#[cfg(feature = "Oculus+Platform+LeaderboardFilterType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::LeaderboardFilterType =>
    "Oculus.Platform"."LeaderboardFilterType"
);
