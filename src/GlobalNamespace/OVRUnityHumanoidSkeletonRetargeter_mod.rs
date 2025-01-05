#[cfg(
    feature = "OVRUnityHumanoidSkeletonRetargeter+OVRHumanBodyBonesMappings+BodySection"
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
    feature = "OVRUnityHumanoidSkeletonRetargeter+OVRHumanBodyBonesMappings+BodySection"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodySection
    => ""."OVRUnityHumanoidSkeletonRetargeter/OVRHumanBodyBonesMappings/BodySection"
);
#[cfg(
    feature = "OVRUnityHumanoidSkeletonRetargeter+OVRHumanBodyBonesMappings+BodyTrackingBoneId"
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
    feature = "OVRUnityHumanoidSkeletonRetargeter+OVRHumanBodyBonesMappings+BodyTrackingBoneId"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodyTrackingBoneId
    => ""
    ."OVRUnityHumanoidSkeletonRetargeter/OVRHumanBodyBonesMappings/BodyTrackingBoneId"
);
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter+OVRSkeletonMetadata+BoneData")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRSkeletonMetadata_OVRUnityHumanoidSkeletonRetargeter_BoneData {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter+OVRSkeletonMetadata+BoneData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRSkeletonMetadata_OVRUnityHumanoidSkeletonRetargeter_BoneData =>
    ""."OVRUnityHumanoidSkeletonRetargeter/OVRSkeletonMetadata/BoneData"
);
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter+OVRSkeletonMetadata+BoneData")]
impl std::ops::Deref
for crate::GlobalNamespace::OVRSkeletonMetadata_OVRUnityHumanoidSkeletonRetargeter_BoneData {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter+OVRSkeletonMetadata+BoneData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OVRSkeletonMetadata_OVRUnityHumanoidSkeletonRetargeter_BoneData {
    fn deref_mut(&mut self) -> &mut Self::Target {
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
    pub fn New_Gc1(
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Gc1(
        &mut self,
        otherBoneData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRSkeletonMetadata_OVRUnityHumanoidSkeletonRetargeter_BoneData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (otherBoneData))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter+OVRSkeletonMetadata+BoneData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRSkeletonMetadata_OVRUnityHumanoidSkeletonRetargeter_BoneData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRUnityHumanoidSkeletonRetargeter {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSkeleton>,
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
        crate::GlobalNamespace::OVRSkeleton_BoneId,
        crate::UnityEngine::HumanBodyBones,
    >,
    pub _targetTPoseRotations: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::HumanBodyBones,
        crate::UnityEngine::Quaternion,
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
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter => ""
    ."OVRUnityHumanoidSkeletonRetargeter"
);
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter")]
impl std::ops::Deref for crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter {
    type Target = quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSkeleton>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter {
    fn deref_mut(&mut self) -> &mut Self::Target {
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AdjustCustomBoneIdToHumanBodyBoneMapping", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AlignTargetWithSource(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AlignTargetWithSource", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ComputeOffsetsUsingSkeletonComponent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ComputeOffsetsUsingSkeletonComponent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyBoneIdToHumanBodyBoneMapping(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyBoneIdToHumanBodyBoneMapping", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateCustomBoneIdToHumanBodyBoneMapping(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateCustomBoneIdToHumanBodyBoneMapping", ())?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_JointAdjustment,
        > = __cordl_object.invoke("FindAdjustment", (boneId))?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsBodySectionInArray", (bodySectionToCheck, sectionArrayToCheck))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecomputeSkeletalOffsetsIfNecessary", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveMappingCorrespondingToHumanBodyBone(
        &mut self,
        boneId: crate::UnityEngine::HumanBodyBones,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveMappingCorrespondingToHumanBodyBone", (boneId))?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StoreTTargetPoseRotations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StoreTTargetPoseRotations", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateGameObjectForUnityHumanoidRetargeting(
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValidateGameObjectForUnityHumanoidRetargeting", (go))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_JointAdjustment,
                >,
            >,
        > = __cordl_object.invoke("get_Adjustments", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AnimatorTargetSkeleton(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator> = __cordl_object
            .invoke("get_AnimatorTargetSkeleton", ())?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodySection,
            >,
        > = __cordl_object.invoke("get_BodySectionToPosition", ())?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodySection,
            >,
        > = __cordl_object.invoke("get_BodySectionsToAlign", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CustomBoneIdToHumanBodyBone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRSkeleton_BoneId,
            crate::UnityEngine::HumanBodyBones,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRSkeleton_BoneId,
            crate::UnityEngine::HumanBodyBones,
        > = __cordl_object.invoke("get_CustomBoneIdToHumanBodyBone", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SourceSkeletonData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRSkeletonMetadata,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRSkeletonMetadata,
        > = __cordl_object.invoke("get_SourceSkeletonData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SourceSkeletonTPoseData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRSkeletonMetadata,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRSkeletonMetadata,
        > = __cordl_object.invoke("get_SourceSkeletonTPoseData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TargetSkeletonData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRSkeletonMetadata,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRSkeletonMetadata,
        > = __cordl_object.invoke("get_TargetSkeletonData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TargetTPoseRotations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::HumanBodyBones,
            crate::UnityEngine::Quaternion,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::HumanBodyBones,
            crate::UnityEngine::Quaternion,
        > = __cordl_object.invoke("get_TargetTPoseRotations", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter+JointAdjustment")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRUnityHumanoidSkeletonRetargeter_JointAdjustment {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub Joint: crate::UnityEngine::HumanBodyBones,
    pub RotationChange: crate::UnityEngine::Quaternion,
    pub DisableRotationTransform: bool,
    pub DisablePositionTransform: bool,
    pub BoneIdOverrideValue: crate::GlobalNamespace::OVRHumanBodyBonesMappings_OVRUnityHumanoidSkeletonRetargeter_BodyTrackingBoneId,
}
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter+JointAdjustment")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_JointAdjustment => ""
    ."OVRUnityHumanoidSkeletonRetargeter/JointAdjustment"
);
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter+JointAdjustment")]
impl std::ops::Deref
for crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_JointAdjustment {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter+JointAdjustment")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_JointAdjustment {
    fn deref_mut(&mut self) -> &mut Self::Target {
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter+JointAdjustment")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_JointAdjustment {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter+OVRHumanBodyBonesMappings")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRUnityHumanoidSkeletonRetargeter_OVRHumanBodyBonesMappings {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter+OVRHumanBodyBonesMappings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRHumanBodyBonesMappings => ""
    ."OVRUnityHumanoidSkeletonRetargeter/OVRHumanBodyBonesMappings"
);
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter+OVRHumanBodyBonesMappings")]
impl std::ops::Deref
for crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRHumanBodyBonesMappings {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter+OVRHumanBodyBonesMappings")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRHumanBodyBonesMappings {
    fn deref_mut(&mut self) -> &mut Self::Target {
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
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter+OVRHumanBodyBonesMappings")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRHumanBodyBonesMappings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter+OVRSkeletonMetadata")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRUnityHumanoidSkeletonRetargeter_OVRSkeletonMetadata {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _BodyToBoneData_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::HumanBodyBones,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRSkeletonMetadata_OVRUnityHumanoidSkeletonRetargeter_BoneData,
        >,
    >,
    pub _boneEnumValues: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::HumanBodyBones>,
    >,
}
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter+OVRSkeletonMetadata")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRSkeletonMetadata => ""
    ."OVRUnityHumanoidSkeletonRetargeter/OVRSkeletonMetadata"
);
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter+OVRSkeletonMetadata")]
impl std::ops::Deref
for crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRSkeletonMetadata {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter+OVRSkeletonMetadata")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRSkeletonMetadata {
    fn deref_mut(&mut self) -> &mut Self::Target {
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
            crate::GlobalNamespace::OVRSkeleton_BoneId,
            crate::UnityEngine::HumanBodyBones,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AssembleSkeleton",
                (skeleton, useBindPose, customBoneIdToHumanBodyBone),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildBoneData(
        &mut self,
        animator: quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BuildBoneData", (animator))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildBoneDataSkeleton(
        &mut self,
        skeleton: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSkeleton>,
        useBindPose: bool,
        customBoneIdToHumanBodyBone: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRSkeleton_BoneId,
            crate::UnityEngine::HumanBodyBones,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "BuildBoneDataSkeleton",
                (skeleton, useBindPose, customBoneIdToHumanBodyBone),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildCoordinateAxesForAllBones(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BuildCoordinateAxesForAllBones", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateQuaternionForBoneData(
        fromPosition: crate::UnityEngine::Vector3,
        toPosition: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateQuaternionForBoneData", (fromPosition, toPosition))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateQuaternionForBoneDataWithRightVec(
        fromPosition: crate::UnityEngine::Vector3,
        toPosition: crate::UnityEngine::Vector3,
        rightVector: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateQuaternionForBoneDataWithRightVec",
                (fromPosition, toPosition, rightVector),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn FindBoneWithBoneId(
        bones: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRBone>,
        >,
        boneId: crate::GlobalNamespace::OVRSkeleton_BoneId,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRBone>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRBone> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindBoneWithBoneId", (bones, boneId))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindFirstChild(
        startTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        currTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindFirstChild", (startTransform, currTransform))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Gc0(
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
    pub fn New_Gc1(
        animator: quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (animator))?;
        Ok(__cordl_object.into())
    }
    pub fn New__cordl_bool_Gc2(
        skeleton: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSkeleton>,
        useBindPose: bool,
        customBoneIdToHumanBodyBone: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRSkeleton_BoneId,
            crate::UnityEngine::HumanBodyBones,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (skeleton, useBindPose, customBoneIdToHumanBodyBone))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_Gc0(
        &mut self,
        otherSkeletonMetaData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRSkeletonMetadata,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (otherSkeletonMetaData))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Gc1(
        &mut self,
        animator: quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (animator))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool_Gc2(
        &mut self,
        skeleton: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSkeleton>,
        useBindPose: bool,
        customBoneIdToHumanBodyBone: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRSkeleton_BoneId,
            crate::UnityEngine::HumanBodyBones,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (skeleton, useBindPose, customBoneIdToHumanBodyBone))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_BodyToBoneData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::HumanBodyBones,
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::OVRSkeletonMetadata_OVRUnityHumanoidSkeletonRetargeter_BoneData,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::HumanBodyBones,
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::OVRSkeletonMetadata_OVRUnityHumanoidSkeletonRetargeter_BoneData,
            >,
        > = __cordl_object.invoke("get_BodyToBoneData", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRUnityHumanoidSkeletonRetargeter+OVRSkeletonMetadata")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRUnityHumanoidSkeletonRetargeter_OVRSkeletonMetadata {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
