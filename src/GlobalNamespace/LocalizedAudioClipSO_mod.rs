#[cfg(feature = "LocalizedAudioClipSO")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalizedAudioClipSO {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub _localizedAudioClipInfo: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::LocalizedAudioClipSO_LocalizedAudioClipInfo,
    >,
    pub _lastLocalizedAudioClipInfo: *mut crate::GlobalNamespace::LocalizedAudioClipSO_LocalizedAudioClipInfo,
}
#[cfg(feature = "LocalizedAudioClipSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LocalizedAudioClipSO => ""
    ."LocalizedAudioClipSO"
);
#[cfg(feature = "LocalizedAudioClipSO")]
impl std::ops::Deref for crate::GlobalNamespace::LocalizedAudioClipSO {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LocalizedAudioClipSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::LocalizedAudioClipSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LocalizedAudioClipSO")]
impl crate::GlobalNamespace::LocalizedAudioClipSO {
    #[cfg(feature = "LocalizedAudioClipSO+LocalizedAudioClipInfo")]
    pub type LocalizedAudioClipInfo = crate::GlobalNamespace::LocalizedAudioClipSO_LocalizedAudioClipInfo;
    #[cfg(feature = "LocalizedAudioClipSO+__c__DisplayClass4_0")]
    pub type __c__DisplayClass4_0 = crate::GlobalNamespace::LocalizedAudioClipSO___c__DisplayClass4_0;
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
    pub fn get_localizedAudioClip(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::AudioClip> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::AudioClip = __cordl_object
            .invoke("get_localizedAudioClip", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "LocalizedAudioClipSO")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::LocalizedAudioClipSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LocalizedAudioClipSO+LocalizedAudioClipInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalizedAudioClipSO_LocalizedAudioClipInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub language: crate::BGLib::Polyglot::Language,
    pub localizedAudioClip: *mut crate::UnityEngine::AudioClip,
}
#[cfg(feature = "LocalizedAudioClipSO+LocalizedAudioClipInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LocalizedAudioClipSO_LocalizedAudioClipInfo => ""
    ."LocalizedAudioClipSO/LocalizedAudioClipInfo"
);
#[cfg(feature = "LocalizedAudioClipSO+LocalizedAudioClipInfo")]
impl std::ops::Deref
for crate::GlobalNamespace::LocalizedAudioClipSO_LocalizedAudioClipInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LocalizedAudioClipSO+LocalizedAudioClipInfo")]
impl std::ops::DerefMut
for crate::GlobalNamespace::LocalizedAudioClipSO_LocalizedAudioClipInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LocalizedAudioClipSO+LocalizedAudioClipInfo")]
impl crate::GlobalNamespace::LocalizedAudioClipSO_LocalizedAudioClipInfo {
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
#[cfg(feature = "LocalizedAudioClipSO+LocalizedAudioClipInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LocalizedAudioClipSO_LocalizedAudioClipInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
