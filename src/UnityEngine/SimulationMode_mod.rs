#[cfg(feature = "UnityEngine+SimulationMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SimulationMode {
    #[default]
    FixedUpdate = 0i32,
    Script = 2i32,
    Update = 1i32,
}
#[cfg(feature = "UnityEngine+SimulationMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::SimulationMode => "UnityEngine"
    ."SimulationMode"
);
