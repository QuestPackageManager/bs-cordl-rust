#[cfg(feature = "UnityEngine+UIElements+HierarchyChangeType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HierarchyChangeType {
    Add = 0i32,
    Move = 2i32,
    Remove = 1i32,
}
#[cfg(feature = "UnityEngine+UIElements+HierarchyChangeType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::HierarchyChangeType =>
    "UnityEngine.UIElements"."HierarchyChangeType"
);
