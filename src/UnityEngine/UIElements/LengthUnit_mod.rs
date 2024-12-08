#[cfg(feature = "UnityEngine+UIElements+LengthUnit")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LengthUnit {
    Percent = 1i32,
    Pixel = 0i32,
}
#[cfg(feature = "UnityEngine+UIElements+LengthUnit")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::LengthUnit =>
    "UnityEngine.UIElements"."LengthUnit"
);
