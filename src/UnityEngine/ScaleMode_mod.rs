#[cfg(feature = "UnityEngine+ScaleMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScaleMode {
    ScaleAndCrop = 1i32,
    ScaleToFit = 2i32,
    StretchToFill = 0i32,
}
#[cfg(feature = "UnityEngine+ScaleMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ScaleMode => "UnityEngine"
    ."ScaleMode"
);
