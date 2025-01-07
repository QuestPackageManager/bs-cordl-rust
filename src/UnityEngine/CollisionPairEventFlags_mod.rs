#[cfg(feature = "UnityEngine+CollisionPairEventFlags")]
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CollisionPairEventFlags {
    #[default]
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
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::CollisionPairEventFlags {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "CollisionPairEventFlags";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+CollisionPairEventFlags")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::CollisionPairEventFlags {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+CollisionPairEventFlags")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::CollisionPairEventFlags {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+CollisionPairEventFlags")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::CollisionPairEventFlags {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+CollisionPairEventFlags")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::CollisionPairEventFlags {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
