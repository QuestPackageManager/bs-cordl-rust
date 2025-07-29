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
#[cfg(
    feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+OVRSkeletonMetadata+BoneData"
)]
impl std::ops::Deref
for crate::GlobalNamespace::OVRSkeletonMetadata_OVRUnityHumanoidSkeletonRetargeter_BoneData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+OVRSkeletonMetadata+BoneData"
)]
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
    pub _lastSkelChangeCount: i32,
    pub _adjustments: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_JointAdjustment,
            >,
        >,
    >,
    pub _bodySectionsToAlign: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodySection,
        >,
    >,
    pub _bodySectionToPosition: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodySection,
        >,
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
#[cfg(feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter")]
impl std::ops::Deref for crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter {
    type Target = crate::GlobalNamespace::OVRSkeleton;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter")]
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
    pub RotationChange: crate::UnityEngine::Quaternion,
    pub DisableRotationTransform: bool,
    pub DisablePositionTransform: bool,
    pub BoneIdOverrideValue: crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodyTrackingBoneId,
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
#[cfg(feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+JointAdjustment")]
impl std::ops::Deref
for crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_JointAdjustment {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+JointAdjustment")]
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
#[cfg(
    feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+OVRHumanBodyBonesMappings"
)]
impl std::ops::Deref
for crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRHumanBodyBonesMappings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+OVRHumanBodyBonesMappings"
)]
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
#[cfg(feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+OVRSkeletonMetadata")]
impl std::ops::Deref
for crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRSkeletonMetadata {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_OVRUnityHumanoidSkeletonRetargeter+OVRSkeletonMetadata")]
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
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("AssembleSkeleton")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AssembleSkeleton", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (skeleton, useBindPose, customBoneIdToHumanBodyBone),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BuildBoneData(
        &mut self,
        animator: quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("BuildBoneData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BuildBoneData", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (animator))?
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
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("BuildBoneDataSkeleton")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BuildBoneDataSkeleton", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (skeleton, useBindPose, customBoneIdToHumanBodyBone),
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
    pub fn New_Animator1(
        animator: quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (animator))?;
        Ok(__cordl_object.into())
    }
    pub fn New_OVRSkeleton__cordl_bool_Dictionary_2_2(
        skeleton: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSkeleton>,
        useBindPose: bool,
        customBoneIdToHumanBodyBone: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::GlobalNamespace::OVRSkeleton_BoneId,
                crate::UnityEngine::HumanBodyBones,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (skeleton, useBindPose, customBoneIdToHumanBodyBone))?;
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
    pub fn _ctor_Animator1(
        &mut self,
        animator: quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>),
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
            cordl_method_info.invoke_unchecked(self, (animator))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_OVRSkeleton__cordl_bool_Dictionary_2_2(
        &mut self,
        skeleton: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSkeleton>,
        useBindPose: bool,
        customBoneIdToHumanBodyBone: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::GlobalNamespace::OVRSkeleton_BoneId,
                crate::UnityEngine::HumanBodyBones,
            >,
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
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (skeleton, useBindPose, customBoneIdToHumanBodyBone),
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
