#[cfg(feature = "UnityEngine+Bindings+TargetType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetType {
    Field = 1i32,
    Function = 0i32,
}
#[cfg(feature = "UnityEngine+Bindings+TargetType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Bindings::TargetType =>
    "UnityEngine.Bindings"."TargetType"
);
