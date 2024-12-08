#[cfg(feature = "UnityEngine+DeviceType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    Console = 2i32,
    Desktop = 3i32,
    Handheld = 1i32,
    Unknown = 0i32,
}
#[cfg(feature = "UnityEngine+DeviceType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::DeviceType => "UnityEngine"
    ."DeviceType"
);
