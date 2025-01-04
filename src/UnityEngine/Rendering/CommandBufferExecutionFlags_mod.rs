#[cfg(feature = "UnityEngine+Rendering+CommandBufferExecutionFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CommandBufferExecutionFlags {
    #[default]
    AsyncCompute = 2i32,
    None = 0i32,
}
#[cfg(feature = "UnityEngine+Rendering+CommandBufferExecutionFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Rendering::CommandBufferExecutionFlags => "UnityEngine.Rendering"
    ."CommandBufferExecutionFlags"
);
