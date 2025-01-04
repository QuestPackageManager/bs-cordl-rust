#[cfg(feature = "UnityEngine+ProBuilder+WindingOrder")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WindingOrder {
    #[default]
    Clockwise = 1i32,
    CounterClockwise = 2i32,
    Unknown = 0i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+WindingOrder")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::WindingOrder =>
    "UnityEngine.ProBuilder"."WindingOrder"
);
