#[cfg(feature = "UnityEngine+ProBuilder+HandleOrientation")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandleOrientation {
    ActiveElement = 2i32,
    ActiveObject = 1i32,
    World = 0i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+HandleOrientation")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::HandleOrientation =>
    "UnityEngine.ProBuilder"."HandleOrientation"
);
