#[cfg(feature = "FileSystemPreviewMediaData")]
#[repr(C)]
#[derive(Debug)]
pub struct FileSystemPreviewMediaData {
    __cordl_parent: crate::System::Object,
    pub _spriteAsyncLoader: *mut SpriteAsyncLoader,
    pub _audioClipAsyncLoader: *mut AudioClipAsyncLoader,
    pub _coverSpritePath: *mut crate::System::String,
    pub _previewAudioClipPath: *mut crate::System::String,
}
#[cfg(feature = "FileSystemPreviewMediaData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for FileSystemPreviewMediaData => ""
    ."FileSystemPreviewMediaData"
);
#[cfg(feature = "FileSystemPreviewMediaData")]
impl std::ops::Deref for FileSystemPreviewMediaData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FileSystemPreviewMediaData")]
impl std::ops::DerefMut for FileSystemPreviewMediaData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FileSystemPreviewMediaData")]
impl FileSystemPreviewMediaData {
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
        spriteAsyncLoader: *mut SpriteAsyncLoader,
        audioClipAsyncLoader: *mut AudioClipAsyncLoader,
        rootPath: *mut crate::System::String,
        coverSpritePath: *mut crate::System::String,
        previewAudioClipPath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
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
        spriteAsyncLoader: *mut SpriteAsyncLoader,
        audioClipAsyncLoader: *mut AudioClipAsyncLoader,
        rootPath: *mut crate::System::String,
        coverSpritePath: *mut crate::System::String,
        previewAudioClipPath: *mut crate::System::String,
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
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "FileSystemPreviewMediaData")]
impl quest_hook::libil2cpp::ObjectType for FileSystemPreviewMediaData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
