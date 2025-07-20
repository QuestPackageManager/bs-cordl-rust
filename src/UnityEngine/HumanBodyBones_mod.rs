#[cfg(feature = "UnityEngine+HumanBodyBones")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HumanBodyBones {
    #[default]
    Chest = 8i32,
    Head = 10i32,
    Hips = 0i32,
    Jaw = 23i32,
    LastBone = 55i32,
    LeftEye = 21i32,
    LeftFoot = 5i32,
    LeftHand = 17i32,
    LeftIndexDistal = 29i32,
    LeftIndexIntermediate = 28i32,
    LeftIndexProximal = 27i32,
    LeftLittleDistal = 38i32,
    LeftLittleIntermediate = 37i32,
    LeftLittleProximal = 36i32,
    LeftLowerArm = 15i32,
    LeftLowerLeg = 3i32,
    LeftMiddleDistal = 32i32,
    LeftMiddleIntermediate = 31i32,
    LeftMiddleProximal = 30i32,
    LeftRingDistal = 35i32,
    LeftRingIntermediate = 34i32,
    LeftRingProximal = 33i32,
    LeftShoulder = 11i32,
    LeftThumbDistal = 26i32,
    LeftThumbIntermediate = 25i32,
    LeftThumbProximal = 24i32,
    LeftToes = 19i32,
    LeftUpperArm = 13i32,
    LeftUpperLeg = 1i32,
    Neck = 9i32,
    RightEye = 22i32,
    RightFoot = 6i32,
    RightHand = 18i32,
    RightIndexDistal = 44i32,
    RightIndexIntermediate = 43i32,
    RightIndexProximal = 42i32,
    RightLittleDistal = 53i32,
    RightLittleIntermediate = 52i32,
    RightLittleProximal = 51i32,
    RightLowerArm = 16i32,
    RightLowerLeg = 4i32,
    RightMiddleDistal = 47i32,
    RightMiddleIntermediate = 46i32,
    RightMiddleProximal = 45i32,
    RightRingDistal = 50i32,
    RightRingIntermediate = 49i32,
    RightRingProximal = 48i32,
    RightShoulder = 12i32,
    RightThumbDistal = 41i32,
    RightThumbIntermediate = 40i32,
    RightThumbProximal = 39i32,
    RightToes = 20i32,
    RightUpperArm = 14i32,
    RightUpperLeg = 2i32,
    Spine = 7i32,
    UpperChest = 54i32,
}
#[cfg(feature = "UnityEngine+HumanBodyBones")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::HumanBodyBones {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "HumanBodyBones";
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
#[cfg(feature = "UnityEngine+HumanBodyBones")]
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::HumanBodyBones {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+HumanBodyBones")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::HumanBodyBones {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+HumanBodyBones")]
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::HumanBodyBones {
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
#[cfg(feature = "UnityEngine+HumanBodyBones")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::HumanBodyBones {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
