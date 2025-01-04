#[cfg(feature = "UnityEngine+CursorMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CursorMode {
    #[default]
    Auto = 0i32,
    ForceSoftware = 1i32,
}
#[cfg(feature = "UnityEngine+CursorMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::CursorMode => "UnityEngine"
    ."CursorMode"
);
