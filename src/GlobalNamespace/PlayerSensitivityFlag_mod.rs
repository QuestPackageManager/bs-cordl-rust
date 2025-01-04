#[cfg(feature = "PlayerSensitivityFlag")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlayerSensitivityFlag {
    #[default]
    Explicit = 3i32,
    Safe = 1i32,
    Themes = 2i32,
    Unknown = 0i32,
}
#[cfg(feature = "PlayerSensitivityFlag")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PlayerSensitivityFlag => ""
    ."PlayerSensitivityFlag"
);
