#[cfg(feature = "UnityEngine+PhysicMaterialCombine")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicMaterialCombine {
    Average = 0i32,
    Maximum = 3i32,
    Minimum = 2i32,
    Multiply = 1i32,
}
#[cfg(feature = "UnityEngine+PhysicMaterialCombine")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::PhysicMaterialCombine =>
    "UnityEngine"."PhysicMaterialCombine"
);
