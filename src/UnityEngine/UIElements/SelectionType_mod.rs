#[cfg(feature = "UnityEngine+UIElements+SelectionType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectionType {
    #[default]
    Multiple = 2i32,
    None = 0i32,
    Single = 1i32,
}
#[cfg(feature = "UnityEngine+UIElements+SelectionType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::SelectionType =>
    "UnityEngine.UIElements"."SelectionType"
);
