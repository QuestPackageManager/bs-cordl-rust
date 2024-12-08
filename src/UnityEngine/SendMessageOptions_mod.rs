#[cfg(feature = "UnityEngine+SendMessageOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SendMessageOptions {
    DontRequireReceiver = 1i32,
    RequireReceiver = 0i32,
}
#[cfg(feature = "UnityEngine+SendMessageOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::SendMessageOptions => "UnityEngine"
    ."SendMessageOptions"
);
