#[cfg(feature = "StaticPreviewMediaData")]
#[repr(C)]
#[derive(Debug)]
pub struct StaticPreviewMediaData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _coverSprite: *mut crate::UnityEngine::Sprite,
    pub _previewAudioClip: *mut crate::UnityEngine::AudioClip,
}
#[cfg(feature = "StaticPreviewMediaData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::StaticPreviewMediaData => ""
    ."StaticPreviewMediaData"
);
#[cfg(feature = "StaticPreviewMediaData")]
impl std::ops::Deref for crate::GlobalNamespace::StaticPreviewMediaData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StaticPreviewMediaData")]
impl std::ops::DerefMut for crate::GlobalNamespace::StaticPreviewMediaData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StaticPreviewMediaData")]
impl crate::GlobalNamespace::StaticPreviewMediaData {
    pub fn GetCoverSpriteAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut crate::UnityEngine::Sprite>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::UnityEngine::Sprite,
        > = __cordl_object.invoke("GetCoverSpriteAsync", (cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn GetPreviewAudioClip(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut crate::UnityEngine::AudioClip>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::UnityEngine::AudioClip,
        > = __cordl_object.invoke("GetPreviewAudioClip", (cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        coverSprite: *mut crate::UnityEngine::Sprite,
        previewAudioClip: *mut crate::UnityEngine::AudioClip,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (coverSprite, previewAudioClip))?;
        Ok(__cordl_object)
    }
    pub fn UnloadPreviewAudioClip(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnloadPreviewAudioClip", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        coverSprite: *mut crate::UnityEngine::Sprite,
        previewAudioClip: *mut crate::UnityEngine::AudioClip,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (coverSprite, previewAudioClip))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "StaticPreviewMediaData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::StaticPreviewMediaData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
