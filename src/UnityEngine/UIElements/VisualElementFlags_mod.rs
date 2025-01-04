#[cfg(feature = "UnityEngine+UIElements+VisualElementFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VisualElementFlags {
    #[default]
    BoundingBoxDirty = 8i32,
    CompositeRoot = 128i32,
    DisableClipping = 1024i32,
    EnableViewDataPersistence = 512i32,
    EventCallbackParentCategoriesDirty = 32i32,
    HierarchyDisplayed = 4096i32,
    Init = 4159i32,
    LayoutManual = 64i32,
    NeedsAttachToPanelEvent = 2048i32,
    RequireMeasureFunction = 256i32,
    StyleInitialized = 8192i32,
    WorldBoundingBoxDirty = 16i32,
    WorldClipDirty = 4i32,
    WorldTransformDirty = 1i32,
    WorldTransformInverseDirty = 2i32,
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::VisualElementFlags =>
    "UnityEngine.UIElements"."VisualElementFlags"
);
