#[cfg(feature = "MediaAsyncLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct MediaAsyncLoader {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "MediaAsyncLoader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MediaAsyncLoader => ""."MediaAsyncLoader"
);
#[cfg(feature = "MediaAsyncLoader")]
impl std::ops::Deref for MediaAsyncLoader {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MediaAsyncLoader")]
impl std::ops::DerefMut for MediaAsyncLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MediaAsyncLoader")]
impl MediaAsyncLoader {
    #[cfg(feature = "MediaAsyncLoader+_LoadSpriteAsync_d__4")]
    pub type _LoadSpriteAsync_d__4 = crate::GlobalNamespace::MediaAsyncLoader__LoadSpriteAsync_d__4;
    #[cfg(feature = "MediaAsyncLoader+_LoadWebpage_d__0")]
    pub type _LoadWebpage_d__0 = crate::GlobalNamespace::MediaAsyncLoader__LoadWebpage_d__0;
    #[cfg(feature = "MediaAsyncLoader+_LoadAudioClipAsync_d__2")]
    pub type _LoadAudioClipAsync_d__2 = crate::GlobalNamespace::MediaAsyncLoader__LoadAudioClipAsync_d__2;
    #[cfg(feature = "MediaAsyncLoader+_LoadTextureAsync_d__3")]
    pub type _LoadTextureAsync_d__3 = crate::GlobalNamespace::MediaAsyncLoader__LoadTextureAsync_d__3;
    pub fn LoadAudioClipFromFilePathAsync(
        &mut self,
        filePath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut crate::UnityEngine::AudioClip>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::UnityEngine::AudioClip,
        > = __cordl_object.invoke("LoadAudioClipFromFilePathAsync", (filePath))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MediaAsyncLoader")]
impl quest_hook::libil2cpp::ObjectType for MediaAsyncLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
