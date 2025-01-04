#[cfg(feature = "UnityEngine+Rendering+CompareFunction")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CompareFunction {
    #[default]
    Always = 8i32,
    Disabled = 0i32,
    Equal = 3i32,
    Greater = 5i32,
    GreaterEqual = 7i32,
    Less = 2i32,
    LessEqual = 4i32,
    Never = 1i32,
    NotEqual = 6i32,
}
#[cfg(feature = "UnityEngine+Rendering+CompareFunction")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::CompareFunction =>
    "UnityEngine.Rendering"."CompareFunction"
);
