#[cfg(feature = "UnityEngine+ProBuilder+IndexFormat")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexFormat {
    Both = 2i32,
    Common = 1i32,
    Local = 0i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+IndexFormat")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::IndexFormat =>
    "UnityEngine.ProBuilder"."IndexFormat"
);
