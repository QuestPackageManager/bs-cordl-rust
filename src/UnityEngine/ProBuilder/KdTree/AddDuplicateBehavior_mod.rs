#[cfg(feature = "UnityEngine+ProBuilder+KdTree+AddDuplicateBehavior")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AddDuplicateBehavior {
    #[default]
    Collect = 3i32,
    Error = 1i32,
    Skip = 0i32,
    Update = 2i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+AddDuplicateBehavior")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::KdTree::AddDuplicateBehavior =>
    "UnityEngine.ProBuilder.KdTree"."AddDuplicateBehavior"
);
