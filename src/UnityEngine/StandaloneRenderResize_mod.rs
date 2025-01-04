#[cfg(feature = "UnityEngine+StandaloneRenderResize")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StandaloneRenderResize {
    #[default]
    Disabled = 1i32,
    Enabled = 0i32,
}
#[cfg(feature = "UnityEngine+StandaloneRenderResize")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::StandaloneRenderResize =>
    "UnityEngine"."StandaloneRenderResize"
);
