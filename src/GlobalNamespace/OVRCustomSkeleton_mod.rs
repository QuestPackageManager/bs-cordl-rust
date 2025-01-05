#[cfg(feature = "OVRCustomSkeleton")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRCustomSkeleton {
    __cordl_parent: crate::GlobalNamespace::OVRSkeleton,
    pub _customBones_V2: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        >,
    >,
    pub retargetingType: crate::GlobalNamespace::OVRCustomSkeleton_RetargetingType,
}
#[cfg(feature = "OVRCustomSkeleton")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRCustomSkeleton => ""
    ."OVRCustomSkeleton"
);
#[cfg(feature = "OVRCustomSkeleton")]
impl std::ops::Deref for crate::GlobalNamespace::OVRCustomSkeleton {
    type Target = crate::GlobalNamespace::OVRSkeleton;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRCustomSkeleton")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRCustomSkeleton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRCustomSkeleton")]
impl crate::GlobalNamespace::OVRCustomSkeleton {
    #[cfg(feature = "OVRCustomSkeleton+RetargetingType")]
    pub type RetargetingType = crate::GlobalNamespace::OVRCustomSkeleton_RetargetingType;
    pub fn AllocateBones(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AllocateBones", ())?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetSkeletonType(
        &mut self,
        skeletonType: crate::GlobalNamespace::OVRSkeleton_SkeletonType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSkeletonType", (skeletonType))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_ISerializationCallbackReceiver_OnAfterDeserialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UnityEngine.ISerializationCallbackReceiver.OnAfterDeserialize",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_ISerializationCallbackReceiver_OnBeforeSerialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.ISerializationCallbackReceiver.OnBeforeSerialize", ())?;
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
    pub fn get_CustomBones(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
            >,
        > = __cordl_object.invoke("get_CustomBones", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRCustomSkeleton")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRCustomSkeleton {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRCustomSkeleton")]
impl AsRef<crate::UnityEngine::ISerializationCallbackReceiver>
for crate::GlobalNamespace::OVRCustomSkeleton {
    fn as_ref(&self) -> &crate::UnityEngine::ISerializationCallbackReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OVRCustomSkeleton")]
impl AsMut<crate::UnityEngine::ISerializationCallbackReceiver>
for crate::GlobalNamespace::OVRCustomSkeleton {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::ISerializationCallbackReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OVRCustomSkeleton+RetargetingType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRCustomSkeleton_RetargetingType {
    #[default]
    OculusSkeleton = 0i32,
}
#[cfg(feature = "OVRCustomSkeleton+RetargetingType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRCustomSkeleton_RetargetingType => ""
    ."OVRCustomSkeleton/RetargetingType"
);
