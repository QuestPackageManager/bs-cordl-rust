#[cfg(feature = "VideoProjectionDataModelSO")]
#[repr(C)]
#[derive(Debug)]
pub struct VideoProjectionDataModelSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _videoClipsWithId: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::VideoProjectionDataModelSO_VideoClipWithId,
        >,
    >,
}
#[cfg(feature = "VideoProjectionDataModelSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::VideoProjectionDataModelSO =>
    ""."VideoProjectionDataModelSO"
);
#[cfg(feature = "VideoProjectionDataModelSO")]
impl std::ops::Deref for crate::GlobalNamespace::VideoProjectionDataModelSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "VideoProjectionDataModelSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::VideoProjectionDataModelSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "VideoProjectionDataModelSO")]
impl crate::GlobalNamespace::VideoProjectionDataModelSO {
    #[cfg(feature = "VideoProjectionDataModelSO+VideoClipWithId")]
    pub type VideoClipWithId = crate::GlobalNamespace::VideoProjectionDataModelSO_VideoClipWithId;
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
    pub fn get_videoClipWithIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::VideoProjectionDataModelSO_VideoClipWithId,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::VideoProjectionDataModelSO_VideoClipWithId,
            >,
        > = __cordl_object.invoke("get_videoClipWithIds", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "VideoProjectionDataModelSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::VideoProjectionDataModelSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "VideoProjectionDataModelSO+VideoClipWithId")]
#[repr(C)]
#[derive(Debug)]
pub struct VideoProjectionDataModelSO_VideoClipWithId {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _id: i32,
    pub _videoAssetReference: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AddressableAssets::AssetReference,
    >,
}
#[cfg(feature = "VideoProjectionDataModelSO+VideoClipWithId")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::VideoProjectionDataModelSO_VideoClipWithId => ""
    ."VideoProjectionDataModelSO/VideoClipWithId"
);
#[cfg(feature = "VideoProjectionDataModelSO+VideoClipWithId")]
impl std::ops::Deref
for crate::GlobalNamespace::VideoProjectionDataModelSO_VideoClipWithId {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "VideoProjectionDataModelSO+VideoClipWithId")]
impl std::ops::DerefMut
for crate::GlobalNamespace::VideoProjectionDataModelSO_VideoClipWithId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "VideoProjectionDataModelSO+VideoClipWithId")]
impl crate::GlobalNamespace::VideoProjectionDataModelSO_VideoClipWithId {
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
    pub fn get_id(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_id", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_videoAssetReference(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AddressableAssets::AssetReference>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::AssetReference,
        > = __cordl_object.invoke("get_videoAssetReference", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "VideoProjectionDataModelSO+VideoClipWithId")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::VideoProjectionDataModelSO_VideoClipWithId {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
