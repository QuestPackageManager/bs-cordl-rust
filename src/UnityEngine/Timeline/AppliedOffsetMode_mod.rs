#[cfg(feature = "UnityEngine+Timeline+AppliedOffsetMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AppliedOffsetMode {
    #[default]
    NoRootTransform = 0i32,
    SceneOffset = 2i32,
    SceneOffsetEditor = 5i32,
    SceneOffsetLegacy = 4i32,
    SceneOffsetLegacyEditor = 6i32,
    TransformOffset = 1i32,
    TransformOffsetLegacy = 3i32,
}
#[cfg(feature = "UnityEngine+Timeline+AppliedOffsetMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::AppliedOffsetMode =>
    "UnityEngine.Timeline"."AppliedOffsetMode"
);
