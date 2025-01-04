#[cfg(feature = "UnityEngine+Rendering+LightProbeUsage")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LightProbeUsage {
    #[default]
    BlendProbes = 1i32,
    CustomProvided = 4i32,
    Off = 0i32,
    UseProxyVolume = 2i32,
}
#[cfg(feature = "UnityEngine+Rendering+LightProbeUsage")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::LightProbeUsage =>
    "UnityEngine.Rendering"."LightProbeUsage"
);
