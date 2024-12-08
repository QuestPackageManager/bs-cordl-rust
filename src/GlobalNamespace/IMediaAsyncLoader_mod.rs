#[cfg(feature = "IMediaAsyncLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct IMediaAsyncLoader {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IMediaAsyncLoader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for IMediaAsyncLoader => ""."IMediaAsyncLoader"
);
#[cfg(feature = "IMediaAsyncLoader")]
impl std::ops::Deref for IMediaAsyncLoader {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IMediaAsyncLoader")]
impl std::ops::DerefMut for IMediaAsyncLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IMediaAsyncLoader")]
impl IMediaAsyncLoader {
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
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IMediaAsyncLoader")]
impl quest_hook::libil2cpp::ObjectType for IMediaAsyncLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}