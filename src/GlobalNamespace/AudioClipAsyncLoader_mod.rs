#[cfg(feature = "AudioClipAsyncLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioClipAsyncLoader {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _cache: *mut crate::GlobalNamespace::IReferenceCountingCache_2<
        i32,
        *mut crate::System::Threading::Tasks::Task_1<*mut crate::UnityEngine::AudioClip>,
    >,
    pub _mediaAsyncLoader: *mut crate::GlobalNamespace::IMediaAsyncLoader,
}
#[cfg(feature = "AudioClipAsyncLoader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::AudioClipAsyncLoader => ""
    ."AudioClipAsyncLoader"
);
#[cfg(feature = "AudioClipAsyncLoader")]
impl std::ops::Deref for crate::GlobalNamespace::AudioClipAsyncLoader {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    #[cfg(feature = "AudioClipAsyncLoader+_Unload_d__18")]
    pub type _Unload_d__18 = crate::GlobalNamespace::AudioClipAsyncLoader__Unload_d__18;
    #[cfg(feature = "AudioClipAsyncLoader+__c")]
    pub type __c = crate::GlobalNamespace::AudioClipAsyncLoader___c;
    #[cfg(feature = "AudioClipAsyncLoader+__c__DisplayClass13_0")]
    pub type __c__DisplayClass13_0 = crate::GlobalNamespace::AudioClipAsyncLoader___c__DisplayClass13_0;
    #[cfg(feature = "AudioClipAsyncLoader+__c__DisplayClass14_0")]
    pub type __c__DisplayClass14_0 = crate::GlobalNamespace::AudioClipAsyncLoader___c__DisplayClass14_0;
    pub fn GetCacheKey_AudioClip0(
        &mut self,
        audioClip: *mut crate::UnityEngine::AudioClip,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetCacheKey", (audioClip))?;
        Ok(__cordl_ret)
    }
    pub fn GetCacheKey_Il2CppString1(
        &mut self,
        audioClipFilePath: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetCacheKey", (audioClipFilePath))?;
        Ok(__cordl_ret)
    }
    pub fn Load_AudioClip4(
        &mut self,
        audioClip: *mut crate::UnityEngine::AudioClip,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut crate::UnityEngine::AudioClip>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::UnityEngine::AudioClip,
        > = __cordl_object.invoke("Load", (audioClip))?;
        Ok(__cordl_ret)
    }
    pub fn Load_IAssetSongAudioClipProvider1(
        &mut self,
        source: *mut crate::GlobalNamespace::IAssetSongAudioClipProvider,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut crate::UnityEngine::AudioClip>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::UnityEngine::AudioClip,
        > = __cordl_object.invoke("Load", (source))?;
        Ok(__cordl_ret)
    }
    pub fn Load_IAssetSongPreviewAudioClipProvider0(
        &mut self,
        source: *mut crate::GlobalNamespace::IAssetSongPreviewAudioClipProvider,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut crate::UnityEngine::AudioClip>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::UnityEngine::AudioClip,
        > = __cordl_object.invoke("Load", (source))?;
        Ok(__cordl_ret)
    }
    pub fn Load_IFilePathSongAudioClipProvider3(
        &mut self,
        source: *mut crate::GlobalNamespace::IFilePathSongAudioClipProvider,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut crate::UnityEngine::AudioClip>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::UnityEngine::AudioClip,
        > = __cordl_object.invoke("Load", (source))?;
        Ok(__cordl_ret)
    }
    pub fn Load_IFilePathSongPreviewAudioClipProvider2(
        &mut self,
        source: *mut crate::GlobalNamespace::IFilePathSongPreviewAudioClipProvider,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut crate::UnityEngine::AudioClip>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::UnityEngine::AudioClip,
        > = __cordl_object.invoke("Load", (source))?;
        Ok(__cordl_ret)
    }
    pub fn Load_Il2CppString5(
        &mut self,
        audioClipFilePath: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut crate::UnityEngine::AudioClip>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::UnityEngine::AudioClip,
        > = __cordl_object.invoke("Load", (audioClipFilePath))?;
        Ok(__cordl_ret)
    }
    pub fn Load_i32_AudioClipAsyncLoader_LoadMethodDelegate6(
        &mut self,
        cacheKey: i32,
        loadMethodDelegate: *mut crate::GlobalNamespace::AudioClipAsyncLoader_LoadMethodDelegate,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut crate::UnityEngine::AudioClip>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::UnityEngine::AudioClip,
        > = __cordl_object.invoke("Load", (cacheKey, loadMethodDelegate))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        cache: *mut crate::GlobalNamespace::IReferenceCountingCache_2<
            i32,
            *mut crate::System::Threading::Tasks::Task_1<
                *mut crate::UnityEngine::AudioClip,
            >,
        >,
        mediaAsyncLoader: *mut crate::GlobalNamespace::IMediaAsyncLoader,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cache, mediaAsyncLoader))?;
        Ok(__cordl_object)
    }
    pub fn Unload_AudioClip4(
        &mut self,
        audioClip: *mut crate::UnityEngine::AudioClip,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Unload", (audioClip))?;
        Ok(__cordl_ret)
    }
    pub fn Unload_IAssetSongAudioClipProvider1(
        &mut self,
        source: *mut crate::GlobalNamespace::IAssetSongAudioClipProvider,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Unload", (source))?;
        Ok(__cordl_ret)
    }
    pub fn Unload_IAssetSongPreviewAudioClipProvider0(
        &mut self,
        source: *mut crate::GlobalNamespace::IAssetSongPreviewAudioClipProvider,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Unload", (source))?;
        Ok(__cordl_ret)
    }
    pub fn Unload_IFilePathSongAudioClipProvider3(
        &mut self,
        source: *mut crate::GlobalNamespace::IFilePathSongAudioClipProvider,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Unload", (source))?;
        Ok(__cordl_ret)
    }
    pub fn Unload_IFilePathSongPreviewAudioClipProvider2(
        &mut self,
        source: *mut crate::GlobalNamespace::IFilePathSongPreviewAudioClipProvider,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Unload", (source))?;
        Ok(__cordl_ret)
    }
    pub fn Unload_Il2CppString5(
        &mut self,
        audioClipFilePath: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Unload", (audioClipFilePath))?;
        Ok(__cordl_ret)
    }
    pub fn Unload_i32_Action_1_6(
        &mut self,
        cacheKey: i32,
        onDelete: *mut crate::System::Action_1<*mut crate::UnityEngine::AudioClip>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Unload", (cacheKey, onDelete))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        cache: *mut crate::GlobalNamespace::IReferenceCountingCache_2<
            i32,
            *mut crate::System::Threading::Tasks::Task_1<
                *mut crate::UnityEngine::AudioClip,
            >,
        >,
        mediaAsyncLoader: *mut crate::GlobalNamespace::IMediaAsyncLoader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cache, mediaAsyncLoader))?;
        Ok(__cordl_ret)
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
    __cordl_parent: crate::System::MulticastDelegate,
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
    type Target = crate::System::MulticastDelegate;
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
        callback: *mut crate::System::AsyncCallback,
        object: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut crate::UnityEngine::AudioClip>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::UnityEngine::AudioClip,
        > = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut crate::UnityEngine::AudioClip>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::UnityEngine::AudioClip,
        > = __cordl_object.invoke("Invoke", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut quest_hook::libil2cpp::Il2CppObject,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut quest_hook::libil2cpp::Il2CppObject,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
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
