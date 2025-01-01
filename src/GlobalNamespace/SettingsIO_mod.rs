#[cfg(feature = "SettingsIO")]
#[repr(C)]
#[derive(Debug)]
pub struct SettingsIO {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "SettingsIO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SettingsIO => ""."SettingsIO"
);
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
    #[cfg(feature = "SettingsIO+_LoadAsync_d__8")]
    pub type _LoadAsync_d__8 = crate::GlobalNamespace::SettingsIO__LoadAsync_d__8;
    #[cfg(feature = "SettingsIO+_SaveAsync_d__7")]
    pub type _SaveAsync_d__7 = crate::GlobalNamespace::SettingsIO__SaveAsync_d__7;
    #[cfg(feature = "SettingsIO+_WipeAsync_d__6")]
    pub type _WipeAsync_d__6 = crate::GlobalNamespace::SettingsIO__WipeAsync_d__6;
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
