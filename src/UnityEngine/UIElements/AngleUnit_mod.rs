#[cfg(feature = "UnityEngine+UIElements+AngleUnit")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AngleUnit {
    Degree = 0i32,
    Gradian = 1i32,
    Radian = 2i32,
    Turn = 3i32,
}
#[cfg(feature = "UnityEngine+UIElements+AngleUnit")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::AngleUnit =>
    "UnityEngine.UIElements"."AngleUnit"
);
