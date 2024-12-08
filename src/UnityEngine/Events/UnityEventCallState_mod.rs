#[cfg(feature = "UnityEngine+Events+UnityEventCallState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnityEventCallState {
    EditorAndRuntime = 1i32,
    Off = 0i32,
    RuntimeOnly = 2i32,
}
#[cfg(feature = "UnityEngine+Events+UnityEventCallState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Events::UnityEventCallState =>
    "UnityEngine.Events"."UnityEventCallState"
);
