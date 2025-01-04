#[cfg(feature = "UnityEngine+AddressableAssets+AddressablesPlatform")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AddressablesPlatform {
    #[default]
    Android = 9i32,
    Linux = 3i32,
    OSX = 2i32,
    PS4 = 4i32,
    Switch = 5i32,
    Unknown = 0i32,
    WebGL = 7i32,
    Windows = 1i32,
    WindowsUniversal = 10i32,
    XboxOne = 6i32,
    iOS = 8i32,
}
#[cfg(feature = "UnityEngine+AddressableAssets+AddressablesPlatform")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::AddressableAssets::AddressablesPlatform =>
    "UnityEngine.AddressableAssets"."AddressablesPlatform"
);
