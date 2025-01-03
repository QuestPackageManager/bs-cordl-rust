#[cfg(feature = "OVRSkeleton")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRSkeleton {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _skeletonType: crate::GlobalNamespace::OVRSkeleton_SkeletonType,
    pub _dataProvider: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::OVRSkeleton_IOVRSkeletonDataProvider,
    >,
    pub _updateRootPose: bool,
    pub _updateRootScale: bool,
    pub _enablePhysicsCapsules: bool,
    pub _applyBoneTranslations: bool,
    pub _bonesGO: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _bindPosesGO: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _capsulesGO: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _bones: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<*mut crate::GlobalNamespace::OVRBone>,
    >,
    pub _bindPoses: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<*mut crate::GlobalNamespace::OVRBone>,
    >,
    pub _capsules: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::OVRBoneCapsule,
        >,
    >,
    pub _skeleton: crate::GlobalNamespace::OVRPlugin_Skeleton2,
    pub wristFixupRotation: crate::UnityEngine::Quaternion,
    pub _IsInitialized_k__BackingField: bool,
    pub _IsDataValid_k__BackingField: bool,
    pub _IsDataHighConfidence_k__BackingField: bool,
    pub _Bones_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IList_1<
            *mut crate::GlobalNamespace::OVRBone,
        >,
    >,
    pub _BindPoses_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IList_1<
            *mut crate::GlobalNamespace::OVRBone,
        >,
    >,
    pub _Capsules_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IList_1<
            *mut crate::GlobalNamespace::OVRBoneCapsule,
        >,
    >,
    pub _SkeletonChangedCount_k__BackingField: i32,
}
#[cfg(feature = "OVRSkeleton")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRSkeleton => ""."OVRSkeleton"
);
#[cfg(feature = "OVRSkeleton")]
impl std::ops::Deref for crate::GlobalNamespace::OVRSkeleton {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSkeleton")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRSkeleton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSkeleton")]
impl crate::GlobalNamespace::OVRSkeleton {
    #[cfg(feature = "OVRSkeleton+BoneId")]
    pub type BoneId = crate::GlobalNamespace::OVRSkeleton_BoneId;
    #[cfg(feature = "OVRSkeleton+IOVRSkeletonDataProvider")]
    type IOVRSkeletonDataProvider = crate::GlobalNamespace::OVRSkeleton_IOVRSkeletonDataProvider;
    #[cfg(feature = "OVRSkeleton+SkeletonPoseData")]
    pub type SkeletonPoseData = crate::GlobalNamespace::OVRSkeleton_SkeletonPoseData;
    #[cfg(feature = "OVRSkeleton+SkeletonType")]
    pub type SkeletonType = crate::GlobalNamespace::OVRSkeleton_SkeletonType;
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn BoneLabelFromBoneId(
        skeletonType: crate::GlobalNamespace::OVRSkeleton_SkeletonType,
        boneId: crate::GlobalNamespace::OVRSkeleton_BoneId,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BoneLabelFromBoneId", (skeletonType, boneId))?;
        Ok(__cordl_ret.into())
    }
    pub fn FixedUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FixedUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBoneTransform(
        &mut self,
        boneId: crate::GlobalNamespace::OVRSkeleton_BoneId,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform> = __cordl_object
            .invoke("GetBoneTransform", (boneId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentEndBoneId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRSkeleton_BoneId> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRSkeleton_BoneId = __cordl_object
            .invoke("GetCurrentEndBoneId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentMaxSkinnableBoneId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRSkeleton_BoneId> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRSkeleton_BoneId = __cordl_object
            .invoke("GetCurrentMaxSkinnableBoneId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentNumBones(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetCurrentNumBones", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentNumSkinnableBones(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetCurrentNumSkinnableBones", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentStartBoneId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRSkeleton_BoneId> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRSkeleton_BoneId = __cordl_object
            .invoke("GetCurrentStartBoneId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSkeletonType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRSkeleton_SkeletonType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRSkeleton_SkeletonType = __cordl_object
            .invoke("GetSkeletonType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeBindPose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeBindPose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeBones(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeBones", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeCapsules(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeCapsules", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsBodySkeleton(
        _cordl_type: crate::GlobalNamespace::OVRSkeleton_SkeletonType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsBodySkeleton", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsHandSkeleton(
        _cordl_type: crate::GlobalNamespace::OVRSkeleton_SkeletonType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsHandSkeleton", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValidBone(
        &mut self,
        bone: crate::GlobalNamespace::OVRSkeleton_BoneId,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsValidBone", (bone))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SearchSkeletonDataProvider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRSkeleton_IOVRSkeletonDataProvider,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRSkeleton_IOVRSkeletonDataProvider,
        > = __cordl_object.invoke("SearchSkeletonDataProvider", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSkeletonType(
        &mut self,
        _cordl_type: crate::GlobalNamespace::OVRSkeleton_SkeletonType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSkeletonType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldInitialize(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ShouldInitialize", ())?;
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
    pub fn UpdateSkeleton(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateSkeleton", ())?;
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
    pub fn get_BindPoses(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::GlobalNamespace::OVRBone,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::GlobalNamespace::OVRBone,
            >,
        > = __cordl_object.invoke("get_BindPoses", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Bones(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::GlobalNamespace::OVRBone,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::GlobalNamespace::OVRBone,
            >,
        > = __cordl_object.invoke("get_Bones", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Capsules(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::GlobalNamespace::OVRBoneCapsule,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::GlobalNamespace::OVRBoneCapsule,
            >,
        > = __cordl_object.invoke("get_Capsules", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsDataHighConfidence(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsDataHighConfidence", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsDataValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsDataValid", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsInitialized(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsInitialized", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SkeletonChangedCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_SkeletonChangedCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_BindPoses(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::GlobalNamespace::OVRBone,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_BindPoses", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Bones(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::GlobalNamespace::OVRBone,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Bones", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Capsules(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::GlobalNamespace::OVRBoneCapsule,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Capsules", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_IsDataHighConfidence(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsDataHighConfidence", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_IsDataValid(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsDataValid", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_IsInitialized(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsInitialized", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_SkeletonChangedCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SkeletonChangedCount", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRSkeleton")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRSkeleton {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRSkeleton+BoneId")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRSkeleton_BoneId {
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
    Invalid = -1i32,
}
#[cfg(feature = "OVRSkeleton+BoneId")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRSkeleton_BoneId => ""
    ."OVRSkeleton/BoneId"
);
#[cfg(feature = "OVRSkeleton+IOVRSkeletonDataProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRSkeleton_IOVRSkeletonDataProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRSkeleton+IOVRSkeletonDataProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRSkeleton_IOVRSkeletonDataProvider => ""
    ."OVRSkeleton/IOVRSkeletonDataProvider"
);
#[cfg(feature = "OVRSkeleton+IOVRSkeletonDataProvider")]
impl std::ops::Deref for crate::GlobalNamespace::OVRSkeleton_IOVRSkeletonDataProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSkeleton+IOVRSkeletonDataProvider")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OVRSkeleton_IOVRSkeletonDataProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSkeleton+IOVRSkeletonDataProvider")]
impl crate::GlobalNamespace::OVRSkeleton_IOVRSkeletonDataProvider {
    pub fn GetSkeletonPoseData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRSkeleton_SkeletonPoseData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRSkeleton_SkeletonPoseData = __cordl_object
            .invoke("GetSkeletonPoseData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSkeletonType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRSkeleton_SkeletonType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRSkeleton_SkeletonType = __cordl_object
            .invoke("GetSkeletonType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_enabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_enabled", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRSkeleton+IOVRSkeletonDataProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRSkeleton_IOVRSkeletonDataProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRSkeleton+SkeletonPoseData")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct OVRSkeleton_SkeletonPoseData {
    pub _RootPose_k__BackingField: crate::GlobalNamespace::OVRPlugin_Posef,
    pub _RootScale_k__BackingField: f32,
    pub _BoneRotations_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::GlobalNamespace::OVRPlugin_Quatf>,
    >,
    pub _IsDataValid_k__BackingField: bool,
    pub _IsDataHighConfidence_k__BackingField: bool,
    pub _BoneTranslations_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::GlobalNamespace::OVRPlugin_Vector3f>,
    >,
    pub _SkeletonChangedCount_k__BackingField: i32,
}
#[cfg(feature = "OVRSkeleton+SkeletonPoseData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRSkeleton_SkeletonPoseData =>
    ""."OVRSkeleton/SkeletonPoseData"
);
#[cfg(feature = "OVRSkeleton+SkeletonPoseData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRSkeleton_SkeletonPoseData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRSkeleton+SkeletonPoseData")]
impl crate::GlobalNamespace::OVRSkeleton_SkeletonPoseData {
    pub fn get_BoneRotations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::GlobalNamespace::OVRPlugin_Quatf>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::GlobalNamespace::OVRPlugin_Quatf>,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_BoneRotations", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_BoneTranslations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::GlobalNamespace::OVRPlugin_Vector3f,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::GlobalNamespace::OVRPlugin_Vector3f,
            >,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_BoneTranslations",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsDataHighConfidence(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsDataHighConfidence",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsDataValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsDataValid",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RootPose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Posef> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Posef = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_RootPose",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RootScale(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_RootScale",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SkeletonChangedCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_SkeletonChangedCount",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_BoneRotations(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::GlobalNamespace::OVRPlugin_Quatf>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_BoneRotations",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_BoneTranslations(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::GlobalNamespace::OVRPlugin_Vector3f,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_BoneTranslations",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_IsDataHighConfidence(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_IsDataHighConfidence",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_IsDataValid(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_IsDataValid",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_RootPose(
        &mut self,
        value: crate::GlobalNamespace::OVRPlugin_Posef,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_RootPose",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_RootScale(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_RootScale",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_SkeletonChangedCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_SkeletonChangedCount",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRSkeleton+SkeletonType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRSkeleton_SkeletonType {
    Body = 2i32,
    HandLeft = 0i32,
    HandRight = 1i32,
    None = -1i32,
}
#[cfg(feature = "OVRSkeleton+SkeletonType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRSkeleton_SkeletonType => ""
    ."OVRSkeleton/SkeletonType"
);
