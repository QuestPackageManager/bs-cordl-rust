#[cfg(feature = "UnityEngine+UIElements+OverflowClipBox")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OverflowClipBox {
    #[default]
    ContentBox = 1i32,
    PaddingBox = 0i32,
}
#[cfg(feature = "UnityEngine+UIElements+OverflowClipBox")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::OverflowClipBox =>
    "UnityEngine.UIElements"."OverflowClipBox"
);
