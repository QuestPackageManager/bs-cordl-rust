#[cfg(feature = "EnvironmentContext")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EnvironmentContext {
    #[default]
    BeatmapEditor = 1i32,
    Gameplay = 0i32,
}
#[cfg(feature = "EnvironmentContext")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::EnvironmentContext => ""
    ."EnvironmentContext"
);
