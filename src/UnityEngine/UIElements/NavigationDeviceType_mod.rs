#[cfg(feature = "UnityEngine+UIElements+NavigationDeviceType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationDeviceType {
    Keyboard = 1i32,
    NonKeyboard = 2i32,
    Unknown = 0i32,
}
#[cfg(feature = "UnityEngine+UIElements+NavigationDeviceType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::NavigationDeviceType =>
    "UnityEngine.UIElements"."NavigationDeviceType"
);
