#[cfg(feature = "GameplayServerControlSettings")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameplayServerControlSettings {
    #[default]
    All = 3i32,
    AllowModifierSelection = 1i32,
    AllowSpectate = 2i32,
    None = 0i32,
}
#[cfg(feature = "GameplayServerControlSettings")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GameplayServerControlSettings
    => ""."GameplayServerControlSettings"
);
