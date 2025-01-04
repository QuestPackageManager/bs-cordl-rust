#[cfg(feature = "UnityEngine+InspectorSort")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InspectorSort {
    #[default]
    ByName = 0i32,
    ByValue = 1i32,
}
#[cfg(feature = "UnityEngine+InspectorSort")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InspectorSort => "UnityEngine"
    ."InspectorSort"
);
