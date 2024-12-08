#[cfg(feature = "UnityEngine+RuntimeInitializeLoadType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeInitializeLoadType {
    AfterAssembliesLoaded = 2i32,
    AfterSceneLoad = 0i32,
    BeforeSceneLoad = 1i32,
    BeforeSplashScreen = 3i32,
    SubsystemRegistration = 4i32,
}
#[cfg(feature = "UnityEngine+RuntimeInitializeLoadType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::RuntimeInitializeLoadType =>
    "UnityEngine"."RuntimeInitializeLoadType"
);
