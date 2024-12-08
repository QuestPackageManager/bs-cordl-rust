#[cfg(feature = "UnityEngine+CollisionPairEventFlags")]
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollisionPairEventFlags {
    ContactDefault = 1025u16,
    ContactEventPose = 16384u16,
    DetectCCDContact = 2048u16,
    DetectDiscreteContact = 1024u16,
    ModifyContacts = 2u16,
    NextFree = 32768u16,
    NotifyContactPoint = 512u16,
    NotifyThresholdForceFound = 64u16,
    NotifyThresholdForceLost = 256u16,
    NotifyThresholdForcePersists = 128u16,
    NotifyTouchCCD = 32u16,
    NotifyTouchFound = 4u16,
    NotifyTouchLost = 16u16,
    NotifyTouchPersists = 8u16,
    PostSolverVelocity = 8192u16,
    PreSolverVelocity = 4096u16,
    SolveContacts = 1u16,
    TriggerDefault = 1044u16,
}
#[cfg(feature = "UnityEngine+CollisionPairEventFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::CollisionPairEventFlags =>
    "UnityEngine"."CollisionPairEventFlags"
);
