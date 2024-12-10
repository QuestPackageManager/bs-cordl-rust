#[cfg(feature = "FileSystemPreviewMediaData")]
#[repr(C)]
#[derive(Debug)]
pub struct FileSystemPreviewMediaData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _spriteAsyncLoader: *mut crate::GlobalNamespace::SpriteAsyncLoader,
    pub _audioClipAsyncLoader: *mut crate::GlobalNamespace::AudioClipAsyncLoader,
    pub _coverSpritePath: *mut quest_hook::libil2cpp::Il2CppString,
    pub _previewAudioClipPath: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "FileSystemPreviewMediaData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::FileSystemPreviewMediaData =>
    ""."FileSystemPreviewMediaData"
);
#[cfg(feature = "FileSystemPreviewMediaData")]
impl std::ops::Deref for crate::GlobalNamespace::FileSystemPreviewMediaData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FileSystemPreviewMediaData")]
impl std::ops::DerefMut for crate::GlobalNamespace::FileSystemPreviewMediaData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FileSystemPreviewMediaData")]
impl crate::GlobalNamespace::FileSystemPreviewMediaData {
    pub fn GetCoverSpriteAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<*mut crate::UnityEngine::Sprite>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<*mut crate::UnityEngine::Sprite>,
        > = __cordl_object.invoke("GetCoverSpriteAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPreviewAudioClip(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<*mut crate::UnityEngine::AudioClip>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<*mut crate::UnityEngine::AudioClip>,
        > = __cordl_object.invoke("GetPreviewAudioClip", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        spriteAsyncLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SpriteAsyncLoader,
        >,
        audioClipAsyncLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::AudioClipAsyncLoader,
        >,
        rootPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        coverSpritePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        previewAudioClipPath: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    spriteAsyncLoader,
                    audioClipAsyncLoader,
                    rootPath,
                    coverSpritePath,
                    previewAudioClipPath,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn UnloadPreviewAudioClip(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnloadPreviewAudioClip", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        spriteAsyncLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SpriteAsyncLoader,
        >,
        audioClipAsyncLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::AudioClipAsyncLoader,
        >,
        rootPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        coverSpritePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        previewAudioClipPath: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    spriteAsyncLoader,
                    audioClipAsyncLoader,
                    rootPath,
                    coverSpritePath,
                    previewAudioClipPath,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "FileSystemPreviewMediaData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::FileSystemPreviewMediaData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
