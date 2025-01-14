#[cfg(feature = "LanguageFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct LanguageFactory {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
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
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::LanguageFactory {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "LanguageFactory";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "LanguageFactory")]
impl std::ops::Deref for crate::GlobalNamespace::LanguageFactory {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::BGLib::Polyglot::Language, 0usize>("Create")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Create", 0usize
                )
            });
        let __cordl_ret: crate::BGLib::Polyglot::Language = unsafe {
            method.invoke_unchecked(self, ())
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerDataModel>,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SettingsManager>,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IPlatformUserModel>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (playerDataModel, settingsManager, platformUserModel),
                )
        };
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
impl AsRef<crate::Zenject::IFactory> for crate::GlobalNamespace::LanguageFactory {
    fn as_ref(&self) -> &crate::Zenject::IFactory {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "LanguageFactory")]
impl AsMut<crate::Zenject::IFactory> for crate::GlobalNamespace::LanguageFactory {
    fn as_mut(&mut self) -> &mut crate::Zenject::IFactory {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "LanguageFactory")]
impl AsRef<crate::Zenject::IFactory_1<crate::BGLib::Polyglot::Language>>
for crate::GlobalNamespace::LanguageFactory {
    fn as_ref(&self) -> &crate::Zenject::IFactory_1<crate::BGLib::Polyglot::Language> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "LanguageFactory")]
impl AsMut<crate::Zenject::IFactory_1<crate::BGLib::Polyglot::Language>>
for crate::GlobalNamespace::LanguageFactory {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Zenject::IFactory_1<crate::BGLib::Polyglot::Language> {
        unsafe { std::mem::transmute(self) }
    }
}
