#[cfg(feature = "UnityEngine+UIElements+InvokePolicy")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvokePolicy {
    Default = 0i32,
    IncludeDisabled = 1i32,
}
#[cfg(feature = "UnityEngine+UIElements+InvokePolicy")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::InvokePolicy =>
    "UnityEngine.UIElements"."InvokePolicy"
);
