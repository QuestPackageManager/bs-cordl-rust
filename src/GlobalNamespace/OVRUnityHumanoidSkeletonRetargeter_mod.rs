#[cfg(
    feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+OVRHumanBodyBonesMappings+BodySection"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodySection {
    #[default]
    Back = 9i32,
    Head = 11i32,
    Hips = 8i32,
    LeftArm = 4i32,
    LeftFoot = 1i32,
    LeftHand = 5i32,
    LeftLeg = 0i32,
    Neck = 10i32,
    RightArm = 6i32,
    RightFoot = 3i32,
    RightHand = 7i32,
    RightLeg = 2i32,
}
#[cfg(
    feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+OVRHumanBodyBonesMappings+BodySection"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodySection {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRUnityHumanoidSkeletonRetargeter/OVRHumanBodyBonesMappings/BodySection";
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
#[cfg(
    feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+OVRHumanBodyBonesMappings+BodySection"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodySection {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+OVRHumanBodyBonesMappings+BodySection"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodySection {
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
#[cfg(
    feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+OVRHumanBodyBonesMappings+BodySection"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodySection {
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
#[cfg(
    feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+OVRHumanBodyBonesMappings+BodySection"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodySection {
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
#[cfg(
    feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+OVRHumanBodyBonesMappings+BodyTrackingBoneId"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodyTrackingBoneId {
    #[default]
    Body_Chest = 5i32,
    Body_End = 70i32,
    Body_Head = 7i32,
    Body_Hips = 1i32,
    Body_LeftArmLower = 11i32,
    Body_LeftArmUpper = 10i32,
    Body_LeftHandIndexDistal = 27i32,
    Body_LeftHandIndexIntermediate = 26i32,
    Body_LeftHandIndexMetacarpal = 24i32,
    Body_LeftHandIndexProximal = 25i32,
    Body_LeftHandIndexTip = 28i32,
    Body_LeftHandLittleDistal = 42i32,
    Body_LeftHandLittleIntermediate = 41i32,
    Body_LeftHandLittleMetacarpal = 39i32,
    Body_LeftHandLittleProximal = 40i32,
    Body_LeftHandLittleTip = 43i32,
    Body_LeftHandMiddleDistal = 32i32,
    Body_LeftHandMiddleIntermediate = 31i32,
    Body_LeftHandMiddleMetacarpal = 29i32,
    Body_LeftHandMiddleProximal = 30i32,
    Body_LeftHandMiddleTip = 33i32,
    Body_LeftHandPalm = 18i32,
    Body_LeftHandRingDistal = 37i32,
    Body_LeftHandRingIntermediate = 36i32,
    Body_LeftHandRingMetacarpal = 34i32,
    Body_LeftHandRingProximal = 35i32,
    Body_LeftHandRingTip = 38i32,
    Body_LeftHandThumbDistal = 22i32,
    Body_LeftHandThumbMetacarpal = 20i32,
    Body_LeftHandThumbProximal = 21i32,
    Body_LeftHandThumbTip = 23i32,
    Body_LeftHandWrist = 19i32,
    Body_LeftHandWristTwist = 12i32,
    Body_LeftScapula = 9i32,
    Body_LeftShoulder = 8i32,
    Body_Neck = 6i32,
    Body_RightArmLower = 16i32,
    Body_RightArmUpper = 15i32,
    Body_RightHandIndexDistal = 53i32,
    Body_RightHandIndexIntermediate = 52i32,
    Body_RightHandIndexMetacarpal = 50i32,
    Body_RightHandIndexProximal = 51i32,
    Body_RightHandIndexTip = 54i32,
    Body_RightHandLittleDistal = 68i32,
    Body_RightHandLittleIntermediate = 67i32,
    Body_RightHandLittleMetacarpal = 65i32,
    Body_RightHandLittleProximal = 66i32,
    Body_RightHandLittleTip = 69i32,
    Body_RightHandMiddleDistal = 58i32,
    Body_RightHandMiddleIntermediate = 57i32,
    Body_RightHandMiddleMetacarpal = 55i32,
    Body_RightHandMiddleProximal = 56i32,
    Body_RightHandMiddleTip = 59i32,
    Body_RightHandPalm = 44i32,
    Body_RightHandRingDistal = 63i32,
    Body_RightHandRingIntermediate = 62i32,
    Body_RightHandRingMetacarpal = 60i32,
    Body_RightHandRingProximal = 61i32,
    Body_RightHandRingTip = 64i32,
    Body_RightHandThumbDistal = 48i32,
    Body_RightHandThumbMetacarpal = 46i32,
    Body_RightHandThumbProximal = 47i32,
    Body_RightHandThumbTip = 49i32,
    Body_RightHandWrist = 45i32,
    Body_RightHandWristTwist = 17i32,
    Body_RightScapula = 14i32,
    Body_RightShoulder = 13i32,
    Body_Root = 0i32,
    Body_SpineLower = 2i32,
    Body_SpineMiddle = 3i32,
    Body_SpineUpper = 4i32,
    NoOverride = 71i32,
    Remove = 72i32,
}
#[cfg(
    feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+OVRHumanBodyBonesMappings+BodyTrackingBoneId"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodyTrackingBoneId {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRUnityHumanoidSkeletonRetargeter/OVRHumanBodyBonesMappings/BodyTrackingBoneId";
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
#[cfg(
    feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+OVRHumanBodyBonesMappings+BodyTrackingBoneId"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodyTrackingBoneId {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+OVRHumanBodyBonesMappings+BodyTrackingBoneId"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodyTrackingBoneId {
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
#[cfg(
    feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+OVRHumanBodyBonesMappings+BodyTrackingBoneId"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodyTrackingBoneId {
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
#[cfg(
    feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+OVRHumanBodyBonesMappings+BodyTrackingBoneId"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodyTrackingBoneId {
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
#[cfg(
    feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+OVRHumanBodyBonesMappings+FullBodyTrackingBoneId"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_FullBodyTrackingBoneId {
    #[default]
    FullBody_Chest = 5i32,
    FullBody_End = 84i32,
    FullBody_Head = 7i32,
    FullBody_Hips = 1i32,
    FullBody_LeftArmLower = 11i32,
    FullBody_LeftArmUpper = 10i32,
    FullBody_LeftFootAnkle = 73i32,
    FullBody_LeftFootAnkleTwist = 72i32,
    FullBody_LeftFootBall = 76i32,
    FullBody_LeftFootSubtalar = 74i32,
    FullBody_LeftFootTransverse = 75i32,
    FullBody_LeftHandIndexDistal = 27i32,
    FullBody_LeftHandIndexIntermediate = 26i32,
    FullBody_LeftHandIndexMetacarpal = 24i32,
    FullBody_LeftHandIndexProximal = 25i32,
    FullBody_LeftHandIndexTip = 28i32,
    FullBody_LeftHandLittleDistal = 42i32,
    FullBody_LeftHandLittleIntermediate = 41i32,
    FullBody_LeftHandLittleMetacarpal = 39i32,
    FullBody_LeftHandLittleProximal = 40i32,
    FullBody_LeftHandLittleTip = 43i32,
    FullBody_LeftHandMiddleDistal = 32i32,
    FullBody_LeftHandMiddleIntermediate = 31i32,
    FullBody_LeftHandMiddleMetacarpal = 29i32,
    FullBody_LeftHandMiddleProximal = 30i32,
    FullBody_LeftHandMiddleTip = 33i32,
    FullBody_LeftHandPalm = 18i32,
    FullBody_LeftHandRingDistal = 37i32,
    FullBody_LeftHandRingIntermediate = 36i32,
    FullBody_LeftHandRingMetacarpal = 34i32,
    FullBody_LeftHandRingProximal = 35i32,
    FullBody_LeftHandRingTip = 38i32,
    FullBody_LeftHandThumbDistal = 22i32,
    FullBody_LeftHandThumbMetacarpal = 20i32,
    FullBody_LeftHandThumbProximal = 21i32,
    FullBody_LeftHandThumbTip = 23i32,
    FullBody_LeftHandWrist = 19i32,
    FullBody_LeftHandWristTwist = 12i32,
    FullBody_LeftLowerLeg = 71i32,
    FullBody_LeftScapula = 9i32,
    FullBody_LeftShoulder = 8i32,
    FullBody_LeftUpperLeg = 70i32,
    FullBody_Neck = 6i32,
    FullBody_RightArmLower = 16i32,
    FullBody_RightArmUpper = 15i32,
    FullBody_RightFootAnkle = 80i32,
    FullBody_RightFootAnkleTwist = 79i32,
    FullBody_RightFootBall = 83i32,
    FullBody_RightFootSubtalar = 81i32,
    FullBody_RightFootTransverse = 82i32,
    FullBody_RightHandIndexDistal = 53i32,
    FullBody_RightHandIndexIntermediate = 52i32,
    FullBody_RightHandIndexMetacarpal = 50i32,
    FullBody_RightHandIndexProximal = 51i32,
    FullBody_RightHandIndexTip = 54i32,
    FullBody_RightHandLittleDistal = 68i32,
    FullBody_RightHandLittleIntermediate = 67i32,
    FullBody_RightHandLittleMetacarpal = 65i32,
    FullBody_RightHandLittleProximal = 66i32,
    FullBody_RightHandLittleTip = 69i32,
    FullBody_RightHandMiddleDistal = 58i32,
    FullBody_RightHandMiddleIntermediate = 57i32,
    FullBody_RightHandMiddleMetacarpal = 55i32,
    FullBody_RightHandMiddleProximal = 56i32,
    FullBody_RightHandMiddleTip = 59i32,
    FullBody_RightHandPalm = 44i32,
    FullBody_RightHandRingDistal = 63i32,
    FullBody_RightHandRingIntermediate = 62i32,
    FullBody_RightHandRingMetacarpal = 60i32,
    FullBody_RightHandRingProximal = 61i32,
    FullBody_RightHandRingTip = 64i32,
    FullBody_RightHandThumbDistal = 48i32,
    FullBody_RightHandThumbMetacarpal = 46i32,
    FullBody_RightHandThumbProximal = 47i32,
    FullBody_RightHandThumbTip = 49i32,
    FullBody_RightHandWrist = 45i32,
    FullBody_RightHandWristTwist = 17i32,
    FullBody_RightLowerLeg = 78i32,
    FullBody_RightScapula = 14i32,
    FullBody_RightShoulder = 13i32,
    FullBody_RightUpperLeg = 77i32,
    FullBody_Root = 0i32,
    FullBody_SpineLower = 2i32,
    FullBody_SpineMiddle = 3i32,
    FullBody_SpineUpper = 4i32,
    NoOverride = 85i32,
    Remove = 86i32,
}
#[cfg(
    feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+OVRHumanBodyBonesMappings+FullBodyTrackingBoneId"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_FullBodyTrackingBoneId {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRUnityHumanoidSkeletonRetargeter/OVRHumanBodyBonesMappings/FullBodyTrackingBoneId";
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
#[cfg(
    feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+OVRHumanBodyBonesMappings+FullBodyTrackingBoneId"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_FullBodyTrackingBoneId {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+OVRHumanBodyBonesMappings+FullBodyTrackingBoneId"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_FullBodyTrackingBoneId {
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
#[cfg(
    feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+OVRHumanBodyBonesMappings+FullBodyTrackingBoneId"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_FullBodyTrackingBoneId {
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
#[cfg(
    feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+OVRHumanBodyBonesMappings+FullBodyTrackingBoneId"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_FullBodyTrackingBoneId {
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
#[cfg(
    feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+OVRSkeletonMetadata+BoneData"
)]
#[repr(C)]
#[derive(Debug)]
pub struct OVRSkeletonMetadata_OVRUnityHumanoidSkeletonRetargeter_BoneData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub OriginalJoint: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub FromPosition: crate::UnityEngine::Vector3,
    pub ToPosition: crate::UnityEngine::Vector3,
    pub JointPairStart: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub JointPairEnd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub JointPairOrientation: crate::UnityEngine::Quaternion,
    pub CorrectionQuaternion: crate::System::Nullable_1<crate::UnityEngine::Quaternion>,
    pub ParentTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub DegenerateJoint: bool,
}
#[cfg(
    feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+OVRSkeletonMetadata+BoneData"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRSkeletonMetadata_OVRUnityHumanoidSkeletonRetargeter_BoneData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRUnityHumanoidSkeletonRetargeter/OVRSkeletonMetadata/BoneData";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter+OVRSkeletonMetadata+BoneData")]
impl std::ops::Deref
for crate::GlobalNamespace::OVRSkeletonMetadata_OVRUnityHumanoidSkeletonRetargeter_BoneData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter+OVRSkeletonMetadata+BoneData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OVRSkeletonMetadata_OVRUnityHumanoidSkeletonRetargeter_BoneData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter+OVRSkeletonMetadata+BoneData")]
impl crate::GlobalNamespace::OVRSkeletonMetadata_OVRUnityHumanoidSkeletonRetargeter_BoneData {
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_OVRSkeletonMetadata_OVRUnityHumanoidSkeletonRetargeter_BoneData1(
        otherBoneData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRSkeletonMetadata_OVRUnityHumanoidSkeletonRetargeter_BoneData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (otherBoneData))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_OVRSkeletonMetadata_OVRUnityHumanoidSkeletonRetargeter_BoneData1(
        &mut self,
        otherBoneData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRSkeletonMetadata_OVRUnityHumanoidSkeletonRetargeter_BoneData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::OVRSkeletonMetadata_OVRUnityHumanoidSkeletonRetargeter_BoneData,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (otherBoneData))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+OVRSkeletonMetadata+BoneData"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRSkeletonMetadata_OVRUnityHumanoidSkeletonRetargeter_BoneData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRUnityHumanoidSkeletonRetargeter {
    __cordl_parent: crate::GlobalNamespace::OVRSkeleton,
    pub _sourceSkeletonData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRSkeletonMetadata,
    >,
    pub _sourceSkeletonTPoseData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRSkeletonMetadata,
    >,
    pub _targetSkeletonData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRSkeletonMetadata,
    >,
    pub _animatorTargetSkeleton: quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>,
    pub _customBoneIdToHumanBodyBone: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::GlobalNamespace::OVRSkeleton_BoneId,
            crate::UnityEngine::HumanBodyBones,
        >,
    >,
    pub _targetTPoseRotations: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::UnityEngine::HumanBodyBones,
            crate::UnityEngine::Quaternion,
        >,
    >,
    pub _targetTPoseTransformDup: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::UnityEngine::HumanBodyBones,
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        >,
    >,
    pub _lastSkelChangeCount: i32,
    pub _lastTrackedScale: crate::UnityEngine::Vector3,
    pub _adjustments: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_JointAdjustment,
            >,
        >,
    >,
    pub _fullBodySectionsToAlign: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodySection,
        >,
    >,
    pub _bodySectionsToAlign: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodySection,
        >,
    >,
    pub _fullBodySectionToPosition: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodySection,
        >,
    >,
    pub _bodySectionToPosition: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodySection,
        >,
    >,
    pub _updateType: crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_UpdateType,
    pub _bodyBonesMappingInterface: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::OVRHumanBodyBonesMappingsInterface,
    >,
}
#[cfg(feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRUnityHumanoidSkeletonRetargeter";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter")]
impl std::ops::Deref for crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter {
    type Target = crate::GlobalNamespace::OVRSkeleton;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter")]
impl crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter {
    #[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter+JointAdjustment")]
    pub type JointAdjustment = crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_JointAdjustment;
    #[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter+OVRHumanBodyBonesMappings")]
    pub type OVRHumanBodyBonesMappings = crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRHumanBodyBonesMappings;
    #[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter+OVRSkeletonMetadata")]
    pub type OVRSkeletonMetadata = crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRSkeletonMetadata;
    #[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter+UpdateType")]
    pub type UpdateType = crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_UpdateType;
    pub fn AdjustCustomBoneIdToHumanBodyBoneMapping(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("AdjustCustomBoneIdToHumanBodyBoneMapping")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AdjustCustomBoneIdToHumanBodyBoneMapping", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AlignHierarchies(
        &mut self,
        transformToAlign: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        referenceTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("AlignHierarchies")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AlignHierarchies", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (transformToAlign, referenceTransform))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AlignTargetWithSource(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("AlignTargetWithSource")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AlignTargetWithSource", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeOffsetsUsingSkeletonComponent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("ComputeOffsetsUsingSkeletonComponent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ComputeOffsetsUsingSkeletonComponent", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CopyBoneIdToHumanBodyBoneMapping(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("CopyBoneIdToHumanBodyBoneMapping")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CopyBoneIdToHumanBodyBoneMapping", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateCustomBoneIdToHumanBodyBoneMapping(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("CreateCustomBoneIdToHumanBodyBoneMapping")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateCustomBoneIdToHumanBodyBoneMapping", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateDuplicateTransformHierarchy(
        &mut self,
        transformFromOriginalHierarchy: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Transform,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                        1usize,
                    >("CreateDuplicateTransformHierarchy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateDuplicateTransformHierarchy", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform> = unsafe {
            cordl_method_info.invoke_unchecked(self, (transformFromOriginalHierarchy))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FindAdjustment(
        &mut self,
        boneId: crate::UnityEngine::HumanBodyBones,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_JointAdjustment,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::HumanBodyBones),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_JointAdjustment,
                        >,
                        1usize,
                    >("FindAdjustment")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FindAdjustment", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_JointAdjustment,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (boneId))? };
        Ok(__cordl_ret.into())
    }
    pub fn FindHumanBodyBoneFromTransform(
        &mut self,
        candidateTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::HumanBodyBones> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>),
                        crate::UnityEngine::HumanBodyBones,
                        1usize,
                    >("FindHumanBodyBoneFromTransform")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FindHumanBodyBoneFromTransform", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::HumanBodyBones = unsafe {
            cordl_method_info.invoke_unchecked(self, (candidateTransform))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsBodySectionInArray(
        bodySectionToCheck: crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodySection,
        sectionArrayToCheck: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodySection,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodySection,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodySection,
                                >,
                            >,
                        ),
                        bool,
                        2usize,
                    >("IsBodySectionInArray")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsBodySectionInArray", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (bodySectionToCheck, sectionArrayToCheck))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OffsetComputationNeededThisFrame(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("OffsetComputationNeededThisFrame")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OffsetComputationNeededThisFrame", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn OnValidate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnValidate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnValidate", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PrecomputeAllRotationTweaks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("PrecomputeAllRotationTweaks")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PrecomputeAllRotationTweaks", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RecomputeSkeletalOffsetsIfNecessary(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("RecomputeSkeletalOffsetsIfNecessary")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RecomputeSkeletalOffsetsIfNecessary", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveMappingCorrespondingToHumanBodyBone(
        &mut self,
        boneId: crate::UnityEngine::HumanBodyBones,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::HumanBodyBones),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("RemoveMappingCorrespondingToHumanBodyBone")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RemoveMappingCorrespondingToHumanBodyBone", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (boneId))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShouldRunUpdateThisFrame(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("ShouldRunUpdateThisFrame")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ShouldRunUpdateThisFrame", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Start")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Start",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StoreTTargetPoseRotations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("StoreTTargetPoseRotations")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "StoreTTargetPoseRotations", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Update")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Update",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ValidateGameObjectForUnityHumanoidRetargeting(
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ValidateGameObjectForUnityHumanoidRetargeting")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ValidateGameObjectForUnityHumanoidRetargeting", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (go))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Adjustments(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_JointAdjustment,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_JointAdjustment,
                                >,
                            >,
                        >,
                        0usize,
                    >("get_Adjustments")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Adjustments", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_JointAdjustment,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_AnimatorTargetSkeleton(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>,
                        0usize,
                    >("get_AnimatorTargetSkeleton")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_AnimatorTargetSkeleton", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator> = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_BodyBoneMappingsInterface(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRHumanBodyBonesMappingsInterface,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::OVRHumanBodyBonesMappingsInterface,
                        >,
                        0usize,
                    >("get_BodyBoneMappingsInterface")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_BodyBoneMappingsInterface", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRHumanBodyBonesMappingsInterface,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_BodySectionToPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodySection,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodySection,
                            >,
                        >,
                        0usize,
                    >("get_BodySectionToPosition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_BodySectionToPosition", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodySection,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_BodySectionsToAlign(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodySection,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodySection,
                            >,
                        >,
                        0usize,
                    >("get_BodySectionsToAlign")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_BodySectionsToAlign", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodySection,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_CustomBoneIdToHumanBodyBone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::GlobalNamespace::OVRSkeleton_BoneId,
                crate::UnityEngine::HumanBodyBones,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::Dictionary_2<
                                crate::GlobalNamespace::OVRSkeleton_BoneId,
                                crate::UnityEngine::HumanBodyBones,
                            >,
                        >,
                        0usize,
                    >("get_CustomBoneIdToHumanBodyBone")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_CustomBoneIdToHumanBodyBone", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::GlobalNamespace::OVRSkeleton_BoneId,
                crate::UnityEngine::HumanBodyBones,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_FullBodySectionToPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodySection,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodySection,
                            >,
                        >,
                        0usize,
                    >("get_FullBodySectionToPosition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_FullBodySectionToPosition", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodySection,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_FullBodySectionsToAlign(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodySection,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodySection,
                            >,
                        >,
                        0usize,
                    >("get_FullBodySectionsToAlign")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_FullBodySectionsToAlign", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodySection,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_SourceSkeletonData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRSkeletonMetadata,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRSkeletonMetadata,
                        >,
                        0usize,
                    >("get_SourceSkeletonData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_SourceSkeletonData", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRSkeletonMetadata,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_SourceSkeletonTPoseData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRSkeletonMetadata,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRSkeletonMetadata,
                        >,
                        0usize,
                    >("get_SourceSkeletonTPoseData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_SourceSkeletonTPoseData", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRSkeletonMetadata,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_TargetSkeletonData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRSkeletonMetadata,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRSkeletonMetadata,
                        >,
                        0usize,
                    >("get_TargetSkeletonData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_TargetSkeletonData", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRSkeletonMetadata,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_TargetTPoseRotations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::UnityEngine::HumanBodyBones,
                crate::UnityEngine::Quaternion,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::Dictionary_2<
                                crate::UnityEngine::HumanBodyBones,
                                crate::UnityEngine::Quaternion,
                            >,
                        >,
                        0usize,
                    >("get_TargetTPoseRotations")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_TargetTPoseRotations", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::UnityEngine::HumanBodyBones,
                crate::UnityEngine::Quaternion,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_BodyBoneMappingsInterface(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRHumanBodyBonesMappingsInterface,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::OVRHumanBodyBonesMappingsInterface,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_BodyBoneMappingsInterface")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_BodyBoneMappingsInterface", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+JointAdjustment")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRUnityHumanoidSkeletonRetargeter_JointAdjustment {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Joint: crate::UnityEngine::HumanBodyBones,
    pub PositionChange: crate::UnityEngine::Vector3,
    pub RotationChange: crate::UnityEngine::Quaternion,
    pub RotationTweaks: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Quaternion>,
    >,
    pub DisableRotationTransform: bool,
    pub DisablePositionTransform: bool,
    pub FullBodyBoneIdOverrideValue: crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_FullBodyTrackingBoneId,
    pub BoneIdOverrideValue: crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodyTrackingBoneId,
    pub _PrecomputedRotationTweaks_k__BackingField: crate::UnityEngine::Quaternion,
}
#[cfg(feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+JointAdjustment")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_JointAdjustment {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRUnityHumanoidSkeletonRetargeter/JointAdjustment";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter+JointAdjustment")]
impl std::ops::Deref
for crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_JointAdjustment {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter+JointAdjustment")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_JointAdjustment {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter+JointAdjustment")]
impl crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_JointAdjustment {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PrecomputeRotationTweaks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("PrecomputeRotationTweaks")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PrecomputeRotationTweaks", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_PrecomputedRotationTweaks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::Quaternion,
                        0usize,
                    >("get_PrecomputedRotationTweaks")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_PrecomputedRotationTweaks", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Quaternion = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_PrecomputedRotationTweaks(
        &mut self,
        value: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Quaternion),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_PrecomputedRotationTweaks")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_PrecomputedRotationTweaks", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+JointAdjustment")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_JointAdjustment {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+OVRHumanBodyBonesMappings"
)]
#[repr(C)]
#[derive(Debug)]
pub struct OVRUnityHumanoidSkeletonRetargeter_OVRHumanBodyBonesMappings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+OVRHumanBodyBonesMappings"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRHumanBodyBonesMappings {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRUnityHumanoidSkeletonRetargeter/OVRHumanBodyBonesMappings";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter+OVRHumanBodyBonesMappings")]
impl std::ops::Deref
for crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRHumanBodyBonesMappings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter+OVRHumanBodyBonesMappings")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRHumanBodyBonesMappings {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter+OVRHumanBodyBonesMappings")]
impl crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRHumanBodyBonesMappings {
    #[cfg(
        feature = "OVRUnityHumanoidSkeletonRetargeter+OVRHumanBodyBonesMappings+BodySection"
    )]
    pub type BodySection = crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodySection;
    #[cfg(
        feature = "OVRUnityHumanoidSkeletonRetargeter+OVRHumanBodyBonesMappings+BodyTrackingBoneId"
    )]
    pub type BodyTrackingBoneId = crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodyTrackingBoneId;
    #[cfg(
        feature = "OVRUnityHumanoidSkeletonRetargeter+OVRHumanBodyBonesMappings+FullBodyTrackingBoneId"
    )]
    pub type FullBodyTrackingBoneId = crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_FullBodyTrackingBoneId;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_GetBoneIdToHumanBodyBone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::GlobalNamespace::OVRSkeleton_BoneId,
                crate::UnityEngine::HumanBodyBones,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::Dictionary_2<
                                crate::GlobalNamespace::OVRSkeleton_BoneId,
                                crate::UnityEngine::HumanBodyBones,
                            >,
                        >,
                        0usize,
                    >("get_GetBoneIdToHumanBodyBone")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_GetBoneIdToHumanBodyBone", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::GlobalNamespace::OVRSkeleton_BoneId,
                crate::UnityEngine::HumanBodyBones,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_GetBoneIdToJointPair(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::GlobalNamespace::OVRSkeleton_BoneId,
                quest_hook::libil2cpp::Gc<
                    crate::System::Tuple_2<
                        crate::GlobalNamespace::OVRSkeleton_BoneId,
                        crate::GlobalNamespace::OVRSkeleton_BoneId,
                    >,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::Dictionary_2<
                                crate::GlobalNamespace::OVRSkeleton_BoneId,
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Tuple_2<
                                        crate::GlobalNamespace::OVRSkeleton_BoneId,
                                        crate::GlobalNamespace::OVRSkeleton_BoneId,
                                    >,
                                >,
                            >,
                        >,
                        0usize,
                    >("get_GetBoneIdToJointPair")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_GetBoneIdToJointPair", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::GlobalNamespace::OVRSkeleton_BoneId,
                quest_hook::libil2cpp::Gc<
                    crate::System::Tuple_2<
                        crate::GlobalNamespace::OVRSkeleton_BoneId,
                        crate::GlobalNamespace::OVRSkeleton_BoneId,
                    >,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_GetBoneToBodySection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::UnityEngine::HumanBodyBones,
                crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodySection,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::Dictionary_2<
                                crate::UnityEngine::HumanBodyBones,
                                crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodySection,
                            >,
                        >,
                        0usize,
                    >("get_GetBoneToBodySection")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_GetBoneToBodySection", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::UnityEngine::HumanBodyBones,
                crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodySection,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_GetBoneToJointPair(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::UnityEngine::HumanBodyBones,
                quest_hook::libil2cpp::Gc<
                    crate::System::Tuple_2<
                        crate::UnityEngine::HumanBodyBones,
                        crate::UnityEngine::HumanBodyBones,
                    >,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::Dictionary_2<
                                crate::UnityEngine::HumanBodyBones,
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Tuple_2<
                                        crate::UnityEngine::HumanBodyBones,
                                        crate::UnityEngine::HumanBodyBones,
                                    >,
                                >,
                            >,
                        >,
                        0usize,
                    >("get_GetBoneToJointPair")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_GetBoneToJointPair", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::UnityEngine::HumanBodyBones,
                quest_hook::libil2cpp::Gc<
                    crate::System::Tuple_2<
                        crate::UnityEngine::HumanBodyBones,
                        crate::UnityEngine::HumanBodyBones,
                    >,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_GetFullBodyBoneIdToHumanBodyBone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::GlobalNamespace::OVRSkeleton_BoneId,
                crate::UnityEngine::HumanBodyBones,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::Dictionary_2<
                                crate::GlobalNamespace::OVRSkeleton_BoneId,
                                crate::UnityEngine::HumanBodyBones,
                            >,
                        >,
                        0usize,
                    >("get_GetFullBodyBoneIdToHumanBodyBone")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_GetFullBodyBoneIdToHumanBodyBone", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::GlobalNamespace::OVRSkeleton_BoneId,
                crate::UnityEngine::HumanBodyBones,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_GetFullBodyBoneIdToJointPair(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::GlobalNamespace::OVRSkeleton_BoneId,
                quest_hook::libil2cpp::Gc<
                    crate::System::Tuple_2<
                        crate::GlobalNamespace::OVRSkeleton_BoneId,
                        crate::GlobalNamespace::OVRSkeleton_BoneId,
                    >,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::Dictionary_2<
                                crate::GlobalNamespace::OVRSkeleton_BoneId,
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Tuple_2<
                                        crate::GlobalNamespace::OVRSkeleton_BoneId,
                                        crate::GlobalNamespace::OVRSkeleton_BoneId,
                                    >,
                                >,
                            >,
                        >,
                        0usize,
                    >("get_GetFullBodyBoneIdToJointPair")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_GetFullBodyBoneIdToJointPair", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::GlobalNamespace::OVRSkeleton_BoneId,
                quest_hook::libil2cpp::Gc<
                    crate::System::Tuple_2<
                        crate::GlobalNamespace::OVRSkeleton_BoneId,
                        crate::GlobalNamespace::OVRSkeleton_BoneId,
                    >,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+OVRHumanBodyBonesMappings"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRHumanBodyBonesMappings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter+OVRHumanBodyBonesMappings")]
impl AsRef<crate::GlobalNamespace::OVRHumanBodyBonesMappingsInterface>
for crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRHumanBodyBonesMappings {
    fn as_ref(&self) -> &crate::GlobalNamespace::OVRHumanBodyBonesMappingsInterface {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter+OVRHumanBodyBonesMappings")]
impl AsMut<crate::GlobalNamespace::OVRHumanBodyBonesMappingsInterface>
for crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRHumanBodyBonesMappings {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::OVRHumanBodyBonesMappingsInterface {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+OVRSkeletonMetadata")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRUnityHumanoidSkeletonRetargeter_OVRSkeletonMetadata {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _BodyToBoneData_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::UnityEngine::HumanBodyBones,
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::OVRSkeletonMetadata_OVRUnityHumanoidSkeletonRetargeter_BoneData,
            >,
        >,
    >,
    pub _boneEnumValues: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::HumanBodyBones>,
    >,
}
#[cfg(feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+OVRSkeletonMetadata")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRSkeletonMetadata {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRUnityHumanoidSkeletonRetargeter/OVRSkeletonMetadata";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter+OVRSkeletonMetadata")]
impl std::ops::Deref
for crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRSkeletonMetadata {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter+OVRSkeletonMetadata")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRSkeletonMetadata {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter+OVRSkeletonMetadata")]
impl crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRSkeletonMetadata {
    #[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter+OVRSkeletonMetadata+BoneData")]
    pub type BoneData = crate::GlobalNamespace::OVRSkeletonMetadata_OVRUnityHumanoidSkeletonRetargeter_BoneData;
    pub fn AssembleSkeleton(
        &mut self,
        skeleton: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSkeleton>,
        useBindPose: bool,
        customBoneIdToHumanBodyBone: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::GlobalNamespace::OVRSkeleton_BoneId,
                crate::UnityEngine::HumanBodyBones,
            >,
        >,
        bodyBonesMappingInterface: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRHumanBodyBonesMappingsInterface,
        >,
        useFullBody: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::OVRSkeleton,
                            >,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::Dictionary_2<
                                    crate::GlobalNamespace::OVRSkeleton_BoneId,
                                    crate::UnityEngine::HumanBodyBones,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::OVRHumanBodyBonesMappingsInterface,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("AssembleSkeleton")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AssembleSkeleton", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        skeleton,
                        useBindPose,
                        customBoneIdToHumanBodyBone,
                        bodyBonesMappingInterface,
                        useFullBody,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BuildBoneData(
        &mut self,
        animator: quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>,
        bodyBonesMappingInterface: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRHumanBodyBonesMappingsInterface,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::OVRHumanBodyBonesMappingsInterface,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("BuildBoneData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BuildBoneData", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (animator, bodyBonesMappingInterface))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BuildBoneDataSkeleton(
        &mut self,
        skeleton: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSkeleton>,
        useBindPose: bool,
        customBoneIdToHumanBodyBone: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::GlobalNamespace::OVRSkeleton_BoneId,
                crate::UnityEngine::HumanBodyBones,
            >,
        >,
        bodyBonesMappingInterface: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRHumanBodyBonesMappingsInterface,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::OVRSkeleton,
                            >,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::Dictionary_2<
                                    crate::GlobalNamespace::OVRSkeleton_BoneId,
                                    crate::UnityEngine::HumanBodyBones,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::OVRHumanBodyBonesMappingsInterface,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("BuildBoneDataSkeleton")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BuildBoneDataSkeleton", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        skeleton,
                        useBindPose,
                        customBoneIdToHumanBodyBone,
                        bodyBonesMappingInterface,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BuildBoneDataSkeletonFullBody(
        &mut self,
        skeleton: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSkeleton>,
        useBindPose: bool,
        customBoneIdToHumanBodyBone: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::GlobalNamespace::OVRSkeleton_BoneId,
                crate::UnityEngine::HumanBodyBones,
            >,
        >,
        bodyBonesMappingInterface: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRHumanBodyBonesMappingsInterface,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::OVRSkeleton,
                            >,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::Dictionary_2<
                                    crate::GlobalNamespace::OVRSkeleton_BoneId,
                                    crate::UnityEngine::HumanBodyBones,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::OVRHumanBodyBonesMappingsInterface,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("BuildBoneDataSkeletonFullBody")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BuildBoneDataSkeletonFullBody", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        skeleton,
                        useBindPose,
                        customBoneIdToHumanBodyBone,
                        bodyBonesMappingInterface,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BuildCoordinateAxesForAllBones(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("BuildCoordinateAxesForAllBones")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BuildCoordinateAxesForAllBones", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateQuaternionForBoneData(
        fromPosition: crate::UnityEngine::Vector3,
        toPosition: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Vector3, crate::UnityEngine::Vector3),
                        crate::UnityEngine::Quaternion,
                        2usize,
                    >("CreateQuaternionForBoneData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateQuaternionForBoneData", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Quaternion = unsafe {
            cordl_method_info.invoke_unchecked((), (fromPosition, toPosition))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateQuaternionForBoneDataWithRightVec(
        fromPosition: crate::UnityEngine::Vector3,
        toPosition: crate::UnityEngine::Vector3,
        rightVector: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Vector3,
                        ),
                        crate::UnityEngine::Quaternion,
                        3usize,
                    >("CreateQuaternionForBoneDataWithRightVec")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateQuaternionForBoneDataWithRightVec", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Quaternion = unsafe {
            cordl_method_info
                .invoke_unchecked((), (fromPosition, toPosition, rightVector))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FindBoneWithBoneId(
        bones: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRBone>,
            >,
        >,
        boneId: crate::GlobalNamespace::OVRSkeleton_BoneId,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRBone>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IList_1<
                                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRBone>,
                                >,
                            >,
                            crate::GlobalNamespace::OVRSkeleton_BoneId,
                        ),
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRBone>,
                        2usize,
                    >("FindBoneWithBoneId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FindBoneWithBoneId", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRBone> = unsafe {
            cordl_method_info.invoke_unchecked((), (bones, boneId))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FindFirstChild(
        startTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        currTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                        ),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                        2usize,
                    >("FindFirstChild")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FindFirstChild", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform> = unsafe {
            cordl_method_info.invoke_unchecked((), (startTransform, currTransform))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FixJointPairEndPositionHand(
        &mut self,
        jointPairEndPosition: crate::UnityEngine::Vector3,
        humanBodyBone: crate::UnityEngine::HumanBodyBones,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::HumanBodyBones,
                        ),
                        crate::UnityEngine::Vector3,
                        2usize,
                    >("FixJointPairEndPositionHand")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FixJointPairEndPositionHand", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (jointPairEndPosition, humanBodyBone))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New_Animator_OVRHumanBodyBonesMappingsInterface1(
        animator: quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>,
        bodyBonesMappingInterface: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRHumanBodyBonesMappingsInterface,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (animator, bodyBonesMappingInterface))?;
        Ok(__cordl_object.into())
    }
    pub fn New_OVRSkeleton__cordl_bool_Dictionary_2_OVRHumanBodyBonesMappingsInterface2(
        skeleton: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSkeleton>,
        useBindPose: bool,
        customBoneIdToHumanBodyBone: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::GlobalNamespace::OVRSkeleton_BoneId,
                crate::UnityEngine::HumanBodyBones,
            >,
        >,
        bodyBonesMappingInterface: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRHumanBodyBonesMappingsInterface,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    skeleton,
                    useBindPose,
                    customBoneIdToHumanBodyBone,
                    bodyBonesMappingInterface,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_OVRSkeleton__cordl_bool_Dictionary_2__cordl_bool_OVRHumanBodyBonesMappingsInterface3(
        skeleton: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSkeleton>,
        useBindPose: bool,
        customBoneIdToHumanBodyBone: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::GlobalNamespace::OVRSkeleton_BoneId,
                crate::UnityEngine::HumanBodyBones,
            >,
        >,
        useFullBody: bool,
        bodyBonesMappingInterface: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRHumanBodyBonesMappingsInterface,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    skeleton,
                    useBindPose,
                    customBoneIdToHumanBodyBone,
                    useFullBody,
                    bodyBonesMappingInterface,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_OVRUnityHumanoidSkeletonRetargeter_OVRSkeletonMetadata0(
        otherSkeletonMetaData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRSkeletonMetadata,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (otherSkeletonMetaData))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_Animator_OVRHumanBodyBonesMappingsInterface1(
        &mut self,
        animator: quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>,
        bodyBonesMappingInterface: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRHumanBodyBonesMappingsInterface,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::OVRHumanBodyBonesMappingsInterface,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (animator, bodyBonesMappingInterface))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_OVRSkeleton__cordl_bool_Dictionary_2_OVRHumanBodyBonesMappingsInterface2(
        &mut self,
        skeleton: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSkeleton>,
        useBindPose: bool,
        customBoneIdToHumanBodyBone: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::GlobalNamespace::OVRSkeleton_BoneId,
                crate::UnityEngine::HumanBodyBones,
            >,
        >,
        bodyBonesMappingInterface: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRHumanBodyBonesMappingsInterface,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::OVRSkeleton,
                            >,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::Dictionary_2<
                                    crate::GlobalNamespace::OVRSkeleton_BoneId,
                                    crate::UnityEngine::HumanBodyBones,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::OVRHumanBodyBonesMappingsInterface,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        skeleton,
                        useBindPose,
                        customBoneIdToHumanBodyBone,
                        bodyBonesMappingInterface,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_OVRSkeleton__cordl_bool_Dictionary_2__cordl_bool_OVRHumanBodyBonesMappingsInterface3(
        &mut self,
        skeleton: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSkeleton>,
        useBindPose: bool,
        customBoneIdToHumanBodyBone: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::GlobalNamespace::OVRSkeleton_BoneId,
                crate::UnityEngine::HumanBodyBones,
            >,
        >,
        useFullBody: bool,
        bodyBonesMappingInterface: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRHumanBodyBonesMappingsInterface,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::OVRSkeleton,
                            >,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::Dictionary_2<
                                    crate::GlobalNamespace::OVRSkeleton_BoneId,
                                    crate::UnityEngine::HumanBodyBones,
                                >,
                            >,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::OVRHumanBodyBonesMappingsInterface,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        skeleton,
                        useBindPose,
                        customBoneIdToHumanBodyBone,
                        useFullBody,
                        bodyBonesMappingInterface,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_OVRUnityHumanoidSkeletonRetargeter_OVRSkeletonMetadata0(
        &mut self,
        otherSkeletonMetaData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRSkeletonMetadata,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRSkeletonMetadata,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (otherSkeletonMetaData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_BodyToBoneData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::UnityEngine::HumanBodyBones,
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::OVRSkeletonMetadata_OVRUnityHumanoidSkeletonRetargeter_BoneData,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::Dictionary_2<
                                crate::UnityEngine::HumanBodyBones,
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::OVRSkeletonMetadata_OVRUnityHumanoidSkeletonRetargeter_BoneData,
                                >,
                            >,
                        >,
                        0usize,
                    >("get_BodyToBoneData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_BodyToBoneData", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::UnityEngine::HumanBodyBones,
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::OVRSkeletonMetadata_OVRUnityHumanoidSkeletonRetargeter_BoneData,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+OVRSkeletonMetadata")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRSkeletonMetadata {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+UpdateType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRUnityHumanoidSkeletonRetargeter_UpdateType {
    #[default]
    FixedUpdateAndUpdate = 2i32,
    FixedUpdateOnly = 0i32,
    UpdateOnly = 1i32,
}
#[cfg(feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+UpdateType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_UpdateType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRUnityHumanoidSkeletonRetargeter/UpdateType";
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
#[cfg(feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+UpdateType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_UpdateType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+UpdateType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_UpdateType {
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
#[cfg(feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+UpdateType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_UpdateType {
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
#[cfg(feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+UpdateType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_UpdateType {
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
