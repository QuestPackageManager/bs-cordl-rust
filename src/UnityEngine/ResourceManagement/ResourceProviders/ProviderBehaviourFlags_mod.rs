#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+ProviderBehaviourFlags"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProviderBehaviourFlags {
    #[default]
    CanProvideWithFailedDependencies = 1i32,
    None = 0i32,
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+ProviderBehaviourFlags"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::ResourceProviders::ProviderBehaviourFlags =>
    "UnityEngine.ResourceManagement.ResourceProviders"."ProviderBehaviourFlags"
);
