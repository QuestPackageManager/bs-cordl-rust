#[cfg(feature = "LevelFailedTextOverride+LanguageOverride")]
#[repr(C)]
#[derive(Debug)]
pub struct LevelFailedTextOverride_LanguageOverride {
    __cordl_parent: crate::System::Object,
    pub _language: crate::BGLib::Polyglot::Language,
    pub _overrideText: *mut crate::System::String,
}
#[cfg(feature = "LevelFailedTextOverride+LanguageOverride")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LevelFailedTextOverride_LanguageOverride => ""
    ."LevelFailedTextOverride/LanguageOverride"
);
#[cfg(feature = "LevelFailedTextOverride+LanguageOverride")]
impl std::ops::Deref
for crate::GlobalNamespace::LevelFailedTextOverride_LanguageOverride {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LevelFailedTextOverride+LanguageOverride")]
impl std::ops::DerefMut
for crate::GlobalNamespace::LevelFailedTextOverride_LanguageOverride {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LevelFailedTextOverride+LanguageOverride")]
impl crate::GlobalNamespace::LevelFailedTextOverride_LanguageOverride {
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
    pub fn get_language(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BGLib::Polyglot::Language> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BGLib::Polyglot::Language = __cordl_object
            .invoke("get_language", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_overrideText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_overrideText", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "LevelFailedTextOverride+LanguageOverride")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LevelFailedTextOverride_LanguageOverride {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LevelFailedTextOverride")]
#[repr(C)]
#[derive(Debug)]
pub struct LevelFailedTextOverride {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _perLanguageOverrides: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::LevelFailedTextOverride_LanguageOverride,
    >,
    pub _beatmapLevelPacks: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::AddressableAssets::AssetReferenceT_1<
            *mut BeatmapLevelPackSO,
        >,
    >,
    pub _beatmapLevels: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::AddressableAssets::AssetReferenceT_1<
            *mut BeatmapLevelSO,
        >,
    >,
    pub _levelFailedTextEffect: *mut LevelFailedText,
    pub _beatmapKey: BeatmapKey,
}
#[cfg(feature = "LevelFailedTextOverride")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for LevelFailedTextOverride => ""."LevelFailedTextOverride"
);
#[cfg(feature = "LevelFailedTextOverride")]
impl std::ops::Deref for LevelFailedTextOverride {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LevelFailedTextOverride")]
impl std::ops::DerefMut for LevelFailedTextOverride {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LevelFailedTextOverride")]
impl LevelFailedTextOverride {
    #[cfg(feature = "LevelFailedTextOverride+__c__DisplayClass6_0")]
    pub type __c__DisplayClass6_0 = crate::GlobalNamespace::LevelFailedTextOverride___c__DisplayClass6_0;
    #[cfg(feature = "LevelFailedTextOverride+_Start_d__6")]
    pub type _Start_d__6 = crate::GlobalNamespace::LevelFailedTextOverride__Start_d__6;
    #[cfg(feature = "LevelFailedTextOverride+LanguageOverride")]
    pub type LanguageOverride = crate::GlobalNamespace::LevelFailedTextOverride_LanguageOverride;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
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
#[cfg(feature = "LevelFailedTextOverride")]
impl quest_hook::libil2cpp::ObjectType for LevelFailedTextOverride {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
