#[cfg(feature = "OVRBody")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRBody {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _bodyState: crate::GlobalNamespace::OVRPlugin_BodyState,
    pub _boneRotations: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::OVRPlugin_Quatf,
    >,
    pub _boneTranslations: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::OVRPlugin_Vector3f,
    >,
    pub _dataChangedSinceLastQuery: bool,
    pub _hasData: bool,
    pub _onPermissionGranted: *mut crate::System::Action_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "OVRBody")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRBody => ""."OVRBody"
);
#[cfg(feature = "OVRBody")]
impl std::ops::Deref for crate::GlobalNamespace::OVRBody {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRBody")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRBody {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRBody")]
impl crate::GlobalNamespace::OVRBody {
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
    pub fn GetBodyState(
        &mut self,
        step: crate::GlobalNamespace::OVRPlugin_Step,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetBodyState", (step))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OVRSkeletonRenderer_IOVRSkeletonRendererDataProvider_GetSkeletonRendererData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRSkeletonRenderer_SkeletonRendererData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRSkeletonRenderer_SkeletonRendererData = __cordl_object
            .invoke(
                "OVRSkeletonRenderer.IOVRSkeletonRendererDataProvider.GetSkeletonRendererData",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRSkeleton_IOVRSkeletonDataProvider_GetSkeletonPoseData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRSkeleton_SkeletonPoseData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRSkeleton_SkeletonPoseData = __cordl_object
            .invoke("OVRSkeleton.IOVRSkeletonDataProvider.GetSkeletonPoseData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRSkeleton_IOVRSkeletonDataProvider_GetSkeletonType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRSkeleton_SkeletonType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRSkeleton_SkeletonType = __cordl_object
            .invoke("OVRSkeleton.IOVRSkeletonDataProvider.GetSkeletonType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRSkeleton_IOVRSkeletonDataProvider_get_enabled(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("OVRSkeleton.IOVRSkeletonDataProvider.get_enabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnPermissionGranted(
        &mut self,
        permissionId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPermissionGranted", (permissionId))?;
        Ok(__cordl_ret.into())
    }
    pub fn StartBodyTracking(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("StartBodyTracking", ())?;
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
    pub fn get_BodyState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::GlobalNamespace::OVRPlugin_BodyState>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<
            crate::GlobalNamespace::OVRPlugin_BodyState,
        > = __cordl_object.invoke("get_BodyState", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRBody")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRBody {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRBody")]
impl AsRef<crate::GlobalNamespace::OVRSkeletonRenderer_IOVRSkeletonRendererDataProvider>
for crate::GlobalNamespace::OVRBody {
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::OVRSkeletonRenderer_IOVRSkeletonRendererDataProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OVRBody")]
impl AsMut<crate::GlobalNamespace::OVRSkeletonRenderer_IOVRSkeletonRendererDataProvider>
for crate::GlobalNamespace::OVRBody {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::OVRSkeletonRenderer_IOVRSkeletonRendererDataProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OVRBody")]
impl AsRef<crate::GlobalNamespace::OVRSkeleton_IOVRSkeletonDataProvider>
for crate::GlobalNamespace::OVRBody {
    fn as_ref(&self) -> &crate::GlobalNamespace::OVRSkeleton_IOVRSkeletonDataProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OVRBody")]
impl AsMut<crate::GlobalNamespace::OVRSkeleton_IOVRSkeletonDataProvider>
for crate::GlobalNamespace::OVRBody {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::OVRSkeleton_IOVRSkeletonDataProvider {
        unsafe { std::mem::transmute(self) }
    }
}
