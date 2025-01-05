#[cfg(feature = "AudioClipAsyncLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioClipAsyncLoader {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _cache: quest_hook::libil2cpp::Gc<
        i32,
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        >,
    >,
    pub _mediaAsyncLoader: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IMediaAsyncLoader,
    >,
}
#[cfg(feature = "AudioClipAsyncLoader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::AudioClipAsyncLoader => ""
    ."AudioClipAsyncLoader"
);
#[cfg(feature = "AudioClipAsyncLoader")]
impl std::ops::Deref for crate::GlobalNamespace::AudioClipAsyncLoader {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AudioClipAsyncLoader")]
impl std::ops::DerefMut for crate::GlobalNamespace::AudioClipAsyncLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AudioClipAsyncLoader")]
impl crate::GlobalNamespace::AudioClipAsyncLoader {
    #[cfg(feature = "AudioClipAsyncLoader+LoadMethodDelegate")]
    pub type LoadMethodDelegate = crate::GlobalNamespace::AudioClipAsyncLoader_LoadMethodDelegate;
    pub fn CreateDefault() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::AudioClipAsyncLoader>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::AudioClipAsyncLoader,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("CreateDefault", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCacheKey_Gc0(
        &mut self,
        audioClip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetCacheKey", (audioClip))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCacheKey_Gc1(
        &mut self,
        audioClipFilePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetCacheKey", (audioClipFilePath))?;
        Ok(__cordl_ret.into())
    }
    pub fn Load_Gc0(
        &mut self,
        source: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IAssetSongPreviewAudioClipProvider,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        > = __cordl_object.invoke("Load", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn Load_Gc1(
        &mut self,
        source: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IAssetSongAudioClipProvider,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        > = __cordl_object.invoke("Load", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn Load_Gc2(
        &mut self,
        source: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IFilePathSongPreviewAudioClipProvider,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        > = __cordl_object.invoke("Load", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn Load_Gc3(
        &mut self,
        source: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IFilePathSongAudioClipProvider,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        > = __cordl_object.invoke("Load", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn Load_Gc4(
        &mut self,
        audioClip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        > = __cordl_object.invoke("Load", (audioClip))?;
        Ok(__cordl_ret.into())
    }
    pub fn Load_Gc5(
        &mut self,
        audioClipFilePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        > = __cordl_object.invoke("Load", (audioClipFilePath))?;
        Ok(__cordl_ret.into())
    }
    pub fn Load_i32_Gc6(
        &mut self,
        cacheKey: i32,
        loadMethodDelegate: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::AudioClipAsyncLoader_LoadMethodDelegate,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        > = __cordl_object.invoke("Load", (cacheKey, loadMethodDelegate))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogError(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LogError", (message))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        cache: quest_hook::libil2cpp::Gc<
            i32,
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
            >,
        >,
        mediaAsyncLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMediaAsyncLoader,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cache, mediaAsyncLoader))?;
        Ok(__cordl_object.into())
    }
    pub fn Unload_Gc0(
        &mut self,
        source: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IAssetSongPreviewAudioClipProvider,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Unload", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn Unload_Gc1(
        &mut self,
        source: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IAssetSongAudioClipProvider,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Unload", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn Unload_Gc2(
        &mut self,
        source: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IFilePathSongPreviewAudioClipProvider,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Unload", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn Unload_Gc3(
        &mut self,
        source: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IFilePathSongAudioClipProvider,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Unload", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn Unload_Gc4(
        &mut self,
        audioClip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Unload", (audioClip))?;
        Ok(__cordl_ret.into())
    }
    pub fn Unload_Gc5(
        &mut self,
        audioClipFilePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Unload", (audioClipFilePath))?;
        Ok(__cordl_ret.into())
    }
    pub fn Unload_i32_Gc6(
        &mut self,
        cacheKey: i32,
        onDelete: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Unload", (cacheKey, onDelete))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        cache: quest_hook::libil2cpp::Gc<
            i32,
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
            >,
        >,
        mediaAsyncLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMediaAsyncLoader,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cache, mediaAsyncLoader))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "AudioClipAsyncLoader")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::AudioClipAsyncLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "AudioClipAsyncLoader+LoadMethodDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioClipAsyncLoader_LoadMethodDelegate {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
}
#[cfg(feature = "AudioClipAsyncLoader+LoadMethodDelegate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::AudioClipAsyncLoader_LoadMethodDelegate => ""
    ."AudioClipAsyncLoader/LoadMethodDelegate"
);
#[cfg(feature = "AudioClipAsyncLoader+LoadMethodDelegate")]
impl std::ops::Deref
for crate::GlobalNamespace::AudioClipAsyncLoader_LoadMethodDelegate {
    type Target = quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AudioClipAsyncLoader+LoadMethodDelegate")]
impl std::ops::DerefMut
for crate::GlobalNamespace::AudioClipAsyncLoader_LoadMethodDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AudioClipAsyncLoader+LoadMethodDelegate")]
impl crate::GlobalNamespace::AudioClipAsyncLoader_LoadMethodDelegate {
    pub fn BeginInvoke(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        > = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        > = __cordl_object.invoke("Invoke", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "AudioClipAsyncLoader+LoadMethodDelegate")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::AudioClipAsyncLoader_LoadMethodDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
