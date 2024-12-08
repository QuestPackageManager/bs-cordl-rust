#[cfg(feature = "UnityEngine+TouchType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TouchType {
    Direct = 0i32,
    Indirect = 1i32,
    Stylus = 2i32,
}
#[cfg(feature = "UnityEngine+TouchType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TouchType => "UnityEngine"
    ."TouchType"
);
