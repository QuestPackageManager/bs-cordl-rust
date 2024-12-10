#[cfg(feature = "MediaAsyncLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct MediaAsyncLoader {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "MediaAsyncLoader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MediaAsyncLoader => ""
    ."MediaAsyncLoader"
);
#[cfg(feature = "MediaAsyncLoader")]
impl std::ops::Deref for crate::GlobalNamespace::MediaAsyncLoader {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MediaAsyncLoader")]
impl std::ops::DerefMut for crate::GlobalNamespace::MediaAsyncLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MediaAsyncLoader")]
impl crate::GlobalNamespace::MediaAsyncLoader {
    #[cfg(feature = "MediaAsyncLoader+_LoadAudioClipAsync_d__2")]
    pub type _LoadAudioClipAsync_d__2 = crate::GlobalNamespace::MediaAsyncLoader__LoadAudioClipAsync_d__2;
    #[cfg(feature = "MediaAsyncLoader+_LoadSpriteAsync_d__4")]
    pub type _LoadSpriteAsync_d__4 = crate::GlobalNamespace::MediaAsyncLoader__LoadSpriteAsync_d__4;
    #[cfg(feature = "MediaAsyncLoader+_LoadTextureAsync_d__3")]
    pub type _LoadTextureAsync_d__3 = crate::GlobalNamespace::MediaAsyncLoader__LoadTextureAsync_d__3;
    #[cfg(feature = "MediaAsyncLoader+_LoadWebpage_d__0")]
    pub type _LoadWebpage_d__0 = crate::GlobalNamespace::MediaAsyncLoader__LoadWebpage_d__0;
    pub fn LoadAudioClipFromFilePathAsync(
        &mut self,
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
        > = __cordl_object.invoke("LoadAudioClipFromFilePathAsync", (filePath))?;
        Ok(__cordl_ret.into())
    }
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
}
#[cfg(feature = "MediaAsyncLoader")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::MediaAsyncLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MediaAsyncLoader")]
impl AsRef<crate::GlobalNamespace::IMediaAsyncLoader>
for crate::GlobalNamespace::MediaAsyncLoader {
    fn as_ref(&self) -> &crate::GlobalNamespace::IMediaAsyncLoader {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MediaAsyncLoader")]
impl AsMut<crate::GlobalNamespace::IMediaAsyncLoader>
for crate::GlobalNamespace::MediaAsyncLoader {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IMediaAsyncLoader {
        unsafe { std::mem::transmute(self) }
    }
}
