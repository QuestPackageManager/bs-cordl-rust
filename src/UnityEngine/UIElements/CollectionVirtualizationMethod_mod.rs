#[cfg(feature = "UnityEngine+UIElements+CollectionVirtualizationMethod")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectionVirtualizationMethod {
    DynamicHeight = 1i32,
    FixedHeight = 0i32,
}
#[cfg(feature = "UnityEngine+UIElements+CollectionVirtualizationMethod")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::CollectionVirtualizationMethod => "UnityEngine.UIElements"
    ."CollectionVirtualizationMethod"
);
