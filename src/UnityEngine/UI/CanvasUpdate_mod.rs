#[cfg(feature = "UnityEngine+UI+CanvasUpdate")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanvasUpdate {
    LatePreRender = 4i32,
    Layout = 1i32,
    MaxUpdateValue = 5i32,
    PostLayout = 2i32,
    PreRender = 3i32,
    Prelayout = 0i32,
}
#[cfg(feature = "UnityEngine+UI+CanvasUpdate")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::CanvasUpdate =>
    "UnityEngine.UI"."CanvasUpdate"
);
