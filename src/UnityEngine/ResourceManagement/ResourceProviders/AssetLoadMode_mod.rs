#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+AssetLoadMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetLoadMode {
    AllPackedAssetsAndDependencies = 1i32,
    RequestedAssetAndDependencies = 0i32,
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+AssetLoadMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::ResourceProviders::AssetLoadMode =>
    "UnityEngine.ResourceManagement.ResourceProviders"."AssetLoadMode"
);
