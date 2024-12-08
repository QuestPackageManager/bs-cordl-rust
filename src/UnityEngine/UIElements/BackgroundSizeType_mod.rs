#[cfg(feature = "UnityEngine+UIElements+BackgroundSizeType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackgroundSizeType {
    Contain = 2i32,
    Cover = 1i32,
    Length = 0i32,
}
#[cfg(feature = "UnityEngine+UIElements+BackgroundSizeType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::BackgroundSizeType =>
    "UnityEngine.UIElements"."BackgroundSizeType"
);
