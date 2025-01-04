#[cfg(feature = "UnityEngine+EventModifiers")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EventModifiers {
    #[default]
    Alt = 4i32,
    CapsLock = 32i32,
    Command = 8i32,
    Control = 2i32,
    FunctionKey = 64i32,
    None = 0i32,
    Numeric = 16i32,
    Shift = 1i32,
}
#[cfg(feature = "UnityEngine+EventModifiers")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::EventModifiers => "UnityEngine"
    ."EventModifiers"
);
