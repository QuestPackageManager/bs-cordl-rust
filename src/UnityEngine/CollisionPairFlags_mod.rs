#[cfg(feature = "UnityEngine+CollisionPairFlags")]
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollisionPairFlags {
    ActorPairHasFirstTouch = 4u16,
    ActorPairLostTouch = 8u16,
    InternalContactsAreFlipped = 32u16,
    InternalHasImpulses = 16u16,
    RemovedOtherShape = 2u16,
    RemovedShape = 1u16,
}
#[cfg(feature = "UnityEngine+CollisionPairFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::CollisionPairFlags => "UnityEngine"
    ."CollisionPairFlags"
);
