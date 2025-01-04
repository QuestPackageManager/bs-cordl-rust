#[cfg(feature = "EnvironmentType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EnvironmentType {
    #[default]
    Circle = 1i32,
    Multiplayer = 2i32,
    Normal = 0i32,
    Tutorial = 3i32,
}
#[cfg(feature = "EnvironmentType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::EnvironmentType => ""
    ."EnvironmentType"
);
