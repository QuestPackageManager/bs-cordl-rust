#[cfg(feature = "SceneType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SceneType {
    #[default]
    Game = 2i32,
    Menu = 1i32,
    Undefined = 0i32,
}
#[cfg(feature = "SceneType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SceneType => ""."SceneType"
);
