#[cfg(feature = "IMediaAsyncLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct IMediaAsyncLoader {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IMediaAsyncLoader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IMediaAsyncLoader => ""
    ."IMediaAsyncLoader"
);
#[cfg(feature = "IMediaAsyncLoader")]
impl std::ops::Deref for crate::GlobalNamespace::IMediaAsyncLoader {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IMediaAsyncLoader")]
impl std::ops::DerefMut for crate::GlobalNamespace::IMediaAsyncLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IMediaAsyncLoader")]
impl crate::GlobalNamespace::IMediaAsyncLoader {
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
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IMediaAsyncLoader")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::IMediaAsyncLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
