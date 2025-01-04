#[cfg(feature = "UnityEngine+ProBuilder+Axis")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Axis {
    #[default]
    Backward = 5i32,
    Down = 3i32,
    Forward = 4i32,
    Left = 1i32,
    Right = 0i32,
    Up = 2i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+Axis")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::Axis =>
    "UnityEngine.ProBuilder"."Axis"
);
