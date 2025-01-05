#[cfg(feature = "UnityEngine+Networking+UnityWebRequestMultimedia")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityWebRequestMultimedia {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+Networking+UnityWebRequestMultimedia")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Networking::UnityWebRequestMultimedia => "UnityEngine.Networking"
    ."UnityWebRequestMultimedia"
);
#[cfg(feature = "UnityEngine+Networking+UnityWebRequestMultimedia")]
impl std::ops::Deref for crate::UnityEngine::Networking::UnityWebRequestMultimedia {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Networking+UnityWebRequestMultimedia")]
impl std::ops::DerefMut for crate::UnityEngine::Networking::UnityWebRequestMultimedia {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Networking+UnityWebRequestMultimedia")]
impl crate::UnityEngine::Networking::UnityWebRequestMultimedia {
    pub fn GetAudioClip(
        uri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        audioType: crate::UnityEngine::AudioType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Networking::UnityWebRequest>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::UnityWebRequest,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAudioClip", (uri, audioType))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Networking+UnityWebRequestMultimedia")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Networking::UnityWebRequestMultimedia {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
