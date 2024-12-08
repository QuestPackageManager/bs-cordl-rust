#[cfg(feature = "UnityEngine+CollisionPairHeaderFlags")]
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollisionPairHeaderFlags {
    RemovedActor = 1u16,
    RemovedOtherActor = 2u16,
}
#[cfg(feature = "UnityEngine+CollisionPairHeaderFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::CollisionPairHeaderFlags =>
    "UnityEngine"."CollisionPairHeaderFlags"
);
