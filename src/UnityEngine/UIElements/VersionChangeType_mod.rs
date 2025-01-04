#[cfg(feature = "UnityEngine+UIElements+VersionChangeType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VersionChangeType {
    #[default]
    Bindings = 1i32,
    BorderRadius = 128i32,
    BorderWidth = 256i32,
    Color = 8192i32,
    EventCallbackCategories = 65536i32,
    Hierarchy = 4i32,
    Layout = 8i32,
    Opacity = 4096i32,
    Overflow = 64i32,
    Picking = 1048576i32,
    RenderHints = 16384i32,
    Repaint = 2048i32,
    Size = 1024i32,
    StyleSheet = 16i32,
    Styles = 32i32,
    Transform = 512i32,
    TransitionProperty = 32768i32,
    ViewData = 2i32,
}
#[cfg(feature = "UnityEngine+UIElements+VersionChangeType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::VersionChangeType =>
    "UnityEngine.UIElements"."VersionChangeType"
);
