#[cfg(feature = "UnityEngine+StateInfoIndex")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StateInfoIndex {
    #[default]
    CurrentState = 0i32,
    ExitState = 2i32,
    InterruptedState = 3i32,
    NextState = 1i32,
}
#[cfg(feature = "UnityEngine+StateInfoIndex")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::StateInfoIndex => "UnityEngine"
    ."StateInfoIndex"
);
