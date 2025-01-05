#[cfg(feature = "AudioTypeHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioTypeHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "AudioTypeHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::AudioTypeHelper => ""
    ."AudioTypeHelper"
);
#[cfg(feature = "AudioTypeHelper")]
impl std::ops::Deref for crate::GlobalNamespace::AudioTypeHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AudioTypeHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::AudioTypeHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AudioTypeHelper")]
impl crate::GlobalNamespace::AudioTypeHelper {
    pub fn GetAudioTypeFromPath(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::AudioType> {
        let __cordl_ret: crate::UnityEngine::AudioType = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAudioTypeFromPath", (path))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "AudioTypeHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::AudioTypeHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
