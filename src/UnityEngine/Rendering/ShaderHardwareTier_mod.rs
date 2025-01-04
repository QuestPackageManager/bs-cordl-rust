#[cfg(feature = "UnityEngine+Rendering+ShaderHardwareTier")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ShaderHardwareTier {
    #[default]
    Tier1 = 0i32,
    Tier2 = 1i32,
    Tier3 = 2i32,
}
#[cfg(feature = "UnityEngine+Rendering+ShaderHardwareTier")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::ShaderHardwareTier =>
    "UnityEngine.Rendering"."ShaderHardwareTier"
);
