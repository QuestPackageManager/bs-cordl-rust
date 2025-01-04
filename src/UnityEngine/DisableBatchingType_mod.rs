#[cfg(feature = "UnityEngine+DisableBatchingType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DisableBatchingType {
    #[default]
    False = 0i32,
    True = 1i32,
    WhenLODFading = 2i32,
}
#[cfg(feature = "UnityEngine+DisableBatchingType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::DisableBatchingType =>
    "UnityEngine"."DisableBatchingType"
);
