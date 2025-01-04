#[cfg(feature = "UnityEngine+HorizontalWrapMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HorizontalWrapMode {
    #[default]
    Overflow = 1i32,
    Wrap = 0i32,
}
#[cfg(feature = "UnityEngine+HorizontalWrapMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::HorizontalWrapMode => "UnityEngine"
    ."HorizontalWrapMode"
);
