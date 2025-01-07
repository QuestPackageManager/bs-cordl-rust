#[cfg(feature = "SettingsIO")]
#[repr(C)]
#[derive(Debug)]
pub struct SettingsIO {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "SettingsIO")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::SettingsIO {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "SettingsIO";
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
#[cfg(feature = "SettingsIO")]
impl std::ops::Deref for crate::GlobalNamespace::SettingsIO {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SettingsIO")]
impl std::ops::DerefMut for crate::GlobalNamespace::SettingsIO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SettingsIO")]
impl crate::GlobalNamespace::SettingsIO {
    pub const kGraphicsSettingsPath: &'static str = "GraphicsSettings.json";
    pub const kLegacySettingsPath: &'static str = "settings.cfg";
    pub const kMainSettingsPath: &'static str = "MainSettings.json";
    pub const kSettingsPath: &'static str = "settings.ini";
    pub fn Decode(
        settings: quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::Settings::Settings>,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Decode", (settings, text))?;
        Ok(__cordl_ret.into())
    }
    pub fn Encode(
        settings: quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::Settings::Settings>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Encode", (settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn Load(
        fileStorage: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IFileStorage>,
        platform: crate::GlobalNamespace::HardwareCategory,
    ) -> quest_hook::libil2cpp::Result<crate::BeatSaber::Settings::Settings> {
        let __cordl_ret: crate::BeatSaber::Settings::Settings = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Load", (fileStorage, platform))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAsync(
        fileStorage: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IFileStorage>,
        platform: crate::GlobalNamespace::HardwareCategory,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<crate::BeatSaber::Settings::Settings>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<crate::BeatSaber::Settings::Settings>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadAsync", (fileStorage, platform))?;
        Ok(__cordl_ret.into())
    }
    pub fn SaveAsync(
        fileStorage: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IFileStorage>,
        settings: crate::BeatSaber::Settings::Settings,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SaveAsync", (fileStorage, settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn WipeAsync(
        fileStorage: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IFileStorage>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WipeAsync", (fileStorage))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SettingsIO")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SettingsIO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
