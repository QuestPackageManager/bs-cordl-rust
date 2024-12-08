#[cfg(feature = "UnityEngine+Networking+DownloadHandlerAudioClip")]
#[repr(C)]
#[derive(Debug)]
pub struct DownloadHandlerAudioClip {
    __cordl_parent: crate::UnityEngine::Networking::DownloadHandler,
    pub m_NativeData: crate::Unity::Collections::NativeArray_1<u8>,
}
#[cfg(feature = "UnityEngine+Networking+DownloadHandlerAudioClip")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Networking::DownloadHandlerAudioClip => "UnityEngine.Networking"
    ."DownloadHandlerAudioClip"
);
#[cfg(feature = "UnityEngine+Networking+DownloadHandlerAudioClip")]
impl std::ops::Deref for crate::UnityEngine::Networking::DownloadHandlerAudioClip {
    type Target = crate::UnityEngine::Networking::DownloadHandler;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Networking+DownloadHandlerAudioClip")]
impl std::ops::DerefMut for crate::UnityEngine::Networking::DownloadHandlerAudioClip {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Networking+DownloadHandlerAudioClip")]
impl crate::UnityEngine::Networking::DownloadHandlerAudioClip {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_streamAudio(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_streamAudio", (value))?;
        Ok(__cordl_ret)
    }
    pub fn InternalCreateAudioClip(
        &mut self,
        url: *mut crate::System::String,
        audioType: crate::UnityEngine::AudioType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalCreateAudioClip", (url, audioType))?;
        Ok(__cordl_ret)
    }
    pub fn GetText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetText", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        url: *mut crate::System::String,
        audioType: crate::UnityEngine::AudioType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (url, audioType))?;
        Ok(__cordl_ret)
    }
    pub fn GetNativeData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeArray_1<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Unity::Collections::NativeArray_1<u8> = __cordl_object
            .invoke("GetNativeData", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_audioClip(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::AudioClip> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::AudioClip = __cordl_object
            .invoke("get_audioClip", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        url: *mut crate::System::String,
        audioType: crate::UnityEngine::AudioType,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (url, audioType))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+Networking+DownloadHandlerAudioClip")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Networking::DownloadHandlerAudioClip {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
