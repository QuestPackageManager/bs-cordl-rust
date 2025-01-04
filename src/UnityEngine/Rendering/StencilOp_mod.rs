#[cfg(feature = "UnityEngine+Rendering+StencilOp")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StencilOp {
    #[default]
    DecrementSaturate = 4i32,
    DecrementWrap = 7i32,
    IncrementSaturate = 3i32,
    IncrementWrap = 6i32,
    Invert = 5i32,
    Keep = 0i32,
    Replace = 2i32,
    Zero = 1i32,
}
#[cfg(feature = "UnityEngine+Rendering+StencilOp")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::StencilOp =>
    "UnityEngine.Rendering"."StencilOp"
);
