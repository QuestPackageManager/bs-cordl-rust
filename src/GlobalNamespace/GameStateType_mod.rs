#[cfg(feature = "GameStateType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameStateType {
    #[default]
    EmptyServer = 0i32,
    RunningLevel = 3i32,
    SelectingLevel = 1i32,
    StartingLevel = 2i32,
}
#[cfg(feature = "GameStateType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GameStateType => ""
    ."GameStateType"
);
