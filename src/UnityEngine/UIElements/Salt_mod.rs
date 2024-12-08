#[cfg(feature = "UnityEngine+UIElements+Salt")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Salt {
    ClassSalt = 19i32,
    IdSalt = 17i32,
    TagNameSalt = 13i32,
}
#[cfg(feature = "UnityEngine+UIElements+Salt")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::Salt =>
    "UnityEngine.UIElements"."Salt"
);
