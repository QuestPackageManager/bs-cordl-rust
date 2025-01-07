#[cfg(feature = "UnityEngine+HumanDescription")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HumanDescription {
    pub human: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::HumanBone>,
    >,
    pub skeleton: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::SkeletonBone>,
    >,
    pub m_ArmTwist: f32,
    pub m_ForeArmTwist: f32,
    pub m_UpperLegTwist: f32,
    pub m_LegTwist: f32,
    pub m_ArmStretch: f32,
    pub m_LegStretch: f32,
    pub m_FeetSpacing: f32,
    pub m_GlobalScale: f32,
    pub m_RootMotionBoneName: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub m_HasTranslationDoF: bool,
    pub m_HasExtraRoot: bool,
    pub m_SkeletonHasParents: bool,
}
#[cfg(feature = "UnityEngine+HumanDescription")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::HumanDescription {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "HumanDescription";
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
#[cfg(feature = "UnityEngine+HumanDescription")]
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::HumanDescription {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+HumanDescription")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::HumanDescription {
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
#[cfg(feature = "UnityEngine+HumanDescription")]
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::HumanDescription {
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
#[cfg(feature = "UnityEngine+HumanDescription")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::HumanDescription {
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
#[cfg(feature = "UnityEngine+HumanDescription")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::HumanDescription {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+HumanDescription")]
impl crate::UnityEngine::HumanDescription {}
