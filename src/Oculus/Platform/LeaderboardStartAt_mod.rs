#[cfg(feature = "Oculus+Platform+LeaderboardStartAt")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LeaderboardStartAt {
    #[default]
    CenteredOnViewer = 1i32,
    CenteredOnViewerOrTop = 2i32,
    Top = 0i32,
    Unknown = 3i32,
}
#[cfg(feature = "Oculus+Platform+LeaderboardStartAt")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::LeaderboardStartAt =>
    "Oculus.Platform"."LeaderboardStartAt"
);
