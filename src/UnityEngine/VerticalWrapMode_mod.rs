#[cfg(feature = "UnityEngine+VerticalWrapMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerticalWrapMode {
    Overflow = 1i32,
    Truncate = 0i32,
}
#[cfg(feature = "UnityEngine+VerticalWrapMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::VerticalWrapMode => "UnityEngine"
    ."VerticalWrapMode"
);
