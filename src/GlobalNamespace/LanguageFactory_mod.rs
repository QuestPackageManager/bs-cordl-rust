#[cfg(feature = "LanguageFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct LanguageFactory {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _playerDataModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerDataModel,
    >,
    pub _settingsManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SettingsManager,
    >,
    pub _platformUserModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IPlatformUserModel,
    >,
}
#[cfg(feature = "LanguageFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LanguageFactory => ""
    ."LanguageFactory"
);
#[cfg(feature = "LanguageFactory")]
impl std::ops::Deref for crate::GlobalNamespace::LanguageFactory {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LanguageFactory")]
impl std::ops::DerefMut for crate::GlobalNamespace::LanguageFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LanguageFactory")]
impl crate::GlobalNamespace::LanguageFactory {
    pub fn Create(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BGLib::Polyglot::Language> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BGLib::Polyglot::Language = __cordl_object
            .invoke("Create", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        playerDataModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerDataModel,
        >,
        settingsManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SettingsManager,
        >,
        platformUserModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IPlatformUserModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (playerDataModel, settingsManager, platformUserModel),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        playerDataModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerDataModel,
        >,
        settingsManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SettingsManager,
        >,
        platformUserModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IPlatformUserModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (playerDataModel, settingsManager, platformUserModel))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LanguageFactory")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::LanguageFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LanguageFactory")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::BGLib::Polyglot::Language>>
for crate::GlobalNamespace::LanguageFactory {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::BGLib::Polyglot::Language> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "LanguageFactory")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::BGLib::Polyglot::Language>>
for crate::GlobalNamespace::LanguageFactory {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::BGLib::Polyglot::Language> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "LanguageFactory")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::Zenject::IFactory>>
for crate::GlobalNamespace::LanguageFactory {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::Zenject::IFactory> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "LanguageFactory")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::Zenject::IFactory>>
for crate::GlobalNamespace::LanguageFactory {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<crate::Zenject::IFactory> {
        unsafe { std::mem::transmute(self) }
    }
}
