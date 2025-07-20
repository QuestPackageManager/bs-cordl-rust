#[cfg(feature = "VideoProjectionDataModelSO")]
#[repr(C)]
#[derive(Debug)]
pub struct VideoProjectionDataModelSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _videoClipsWithId: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::VideoProjectionDataModelSO_VideoClipWithId,
            >,
        >,
    >,
}
#[cfg(feature = "VideoProjectionDataModelSO")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::VideoProjectionDataModelSO {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "VideoProjectionDataModelSO";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::VideoProjectionDataModelSO as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::VideoProjectionDataModelSO as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_videoClipWithIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::VideoProjectionDataModelSO_VideoClipWithId,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::VideoProjectionDataModelSO as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::VideoProjectionDataModelSO_VideoClipWithId,
                        >,
                    >,
                >,
                0usize,
            >("get_videoClipWithIds")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::VideoProjectionDataModelSO as
                    quest_hook::libil2cpp::Type > ::class(), "get_videoClipWithIds",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::VideoProjectionDataModelSO_VideoClipWithId,
                >,
            >,
        > = unsafe { method.invoke_unchecked(self, ())? };
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
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::VideoProjectionDataModelSO_VideoClipWithId {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "VideoProjectionDataModelSO/VideoClipWithId";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::VideoProjectionDataModelSO_VideoClipWithId as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::VideoProjectionDataModelSO_VideoClipWithId
                    as quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_id(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::VideoProjectionDataModelSO_VideoClipWithId as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_id")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::VideoProjectionDataModelSO_VideoClipWithId
                    as quest_hook::libil2cpp::Type > ::class(), "get_id", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_videoAssetReference(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AddressableAssets::AssetReference>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::VideoProjectionDataModelSO_VideoClipWithId as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::AddressableAssets::AssetReference,
                >,
                0usize,
            >("get_videoAssetReference")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::VideoProjectionDataModelSO_VideoClipWithId
                    as quest_hook::libil2cpp::Type > ::class(),
                    "get_videoAssetReference", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::AssetReference,
        > = unsafe { method.invoke_unchecked(self, ())? };
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
