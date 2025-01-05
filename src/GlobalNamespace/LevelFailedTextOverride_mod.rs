#[cfg(feature = "LevelFailedTextOverride")]
#[repr(C)]
#[derive(Debug)]
pub struct LevelFailedTextOverride {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _perLanguageOverrides: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::LevelFailedTextOverride_LanguageOverride,
        >,
    >,
    pub _beatmapLevelPacks: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::AddressableAssets::AssetReferenceT_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPackSO>,
            >,
        >,
    >,
    pub _beatmapLevels: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::AddressableAssets::AssetReferenceT_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelSO>,
            >,
        >,
    >,
    pub _levelFailedTextEffect: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LevelFailedText,
    >,
    pub _beatmapKey: crate::GlobalNamespace::BeatmapKey,
}
#[cfg(feature = "LevelFailedTextOverride")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LevelFailedTextOverride => ""
    ."LevelFailedTextOverride"
);
#[cfg(feature = "LevelFailedTextOverride")]
impl std::ops::Deref for crate::GlobalNamespace::LevelFailedTextOverride {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LevelFailedTextOverride")]
impl std::ops::DerefMut for crate::GlobalNamespace::LevelFailedTextOverride {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LevelFailedTextOverride")]
impl crate::GlobalNamespace::LevelFailedTextOverride {
    #[cfg(feature = "LevelFailedTextOverride+LanguageOverride")]
    pub type LanguageOverride = crate::GlobalNamespace::LevelFailedTextOverride_LanguageOverride;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "LevelFailedTextOverride")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LevelFailedTextOverride {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LevelFailedTextOverride+LanguageOverride")]
#[repr(C)]
#[derive(Debug)]
pub struct LevelFailedTextOverride_LanguageOverride {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _language: crate::BGLib::Polyglot::Language,
    pub _overrideText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn get_language(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BGLib::Polyglot::Language> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BGLib::Polyglot::Language = __cordl_object
            .invoke("get_language", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_overrideText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_overrideText", ())?;
        Ok(__cordl_ret.into())
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
