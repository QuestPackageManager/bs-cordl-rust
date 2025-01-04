#[cfg(feature = "UnityEngine+Rendering+OpaqueSortMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OpaqueSortMode {
    #[default]
    Default = 0i32,
    FrontToBack = 1i32,
    NoDistanceSort = 2i32,
}
#[cfg(feature = "UnityEngine+Rendering+OpaqueSortMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::OpaqueSortMode =>
    "UnityEngine.Rendering"."OpaqueSortMode"
);
